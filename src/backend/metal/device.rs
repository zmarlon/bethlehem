use crate::backend::metal::*;
use crate::{Error, ShaderDesc, ShaderKind, ShaderModule, util};
use dispatch2::{DispatchData, dispatch_block_t};
use metal_irconverter::sys;
use metal_irconverter::sys::{
    IRErrorGetCode, IRObjectGetReflection, IRShaderReflectionCreate,
    IRShaderReflectionGetResourceCount, IRShaderReflectionGetResourceLocations, IRShaderStage,
    IRShaderStage_IRShaderStageAmplification, IRShaderStage_IRShaderStageCompute,
    IRShaderStage_IRShaderStageFragment, IRShaderStage_IRShaderStageMesh,
    IRShaderStage_IRShaderStageVertex,
};
use objc2_foundation::NSString;
use objc2_metal::{MTLDevice, MTLLibrary};
use std::ffi::CString;
use std::ptr::NonNull;
use std::sync::Arc;

pub struct MetalDevice {
    physical_device: Arc<MetalPhysicalDevice>,
}

impl MetalDevice {
    pub fn new(physical_device: Arc<MetalPhysicalDevice>) -> Result<Self, Error> {
        Ok(Self { physical_device })
    }

    pub fn create_shader_module(
        self: &Arc<Self>,
        desc: &ShaderDesc,
    ) -> Result<ShaderModule, Error> {
        let dxil_code = util::compile_hlsl(desc, false)?;

        unsafe {
            let entry_point_cstr = CString::new(desc.entry_point.as_str())?;

            let compiler = sys::IRCompilerCreate();
            if (compiler.is_null()) {
                return Err(Error::MetalShaderConverter(
                    "Failed to create ir compiler".to_owned(),
                ));
            }

            sys::IRCompilerSetEntryPointName(compiler, entry_point_cstr.as_ptr());

            let dxil = sys::IRObjectCreateFromDXIL(
                dxil_code.as_ptr(),
                dxil_code.len(),
                sys::IRBytecodeOwnership_IRBytecodeOwnershipNone,
            );

            let mut error = std::ptr::null_mut();
            let out_ir = sys::IRCompilerAllocCompileAndLink(
                compiler,
                entry_point_cstr.as_ptr(),
                dxil,
                &mut error,
            );

            if out_ir.is_null() {
                let error_message = format!(
                    "Failed to compile and link shader: {}",
                    IRErrorGetCode(error)
                );
                sys::IRErrorDestroy(error);

                return Err(Error::MetalShaderConverter(error_message));
            }

            let ir_shader_stage = ir_shader_stage(desc.kind);

            let metal_lib = sys::IRMetalLibBinaryCreate();
            sys::IRObjectGetMetalLibBinary(out_ir, ir_shader_stage, metal_lib);
            let size = sys::IRMetalLibGetBytecodeSize(metal_lib);
            let mut bytecode = vec![0; size];
            sys::IRMetalLibGetBytecode(metal_lib, bytecode.as_mut_ptr());

            //Reflection
            let reflection = IRShaderReflectionCreate();
            if !IRObjectGetReflection(out_ir, ir_shader_stage, reflection) {
                return Err(Error::MetalShaderConverter(
                    "IRObjectGetReflection failed".to_owned(),
                ));
            }

            let count = IRShaderReflectionGetResourceCount(reflection);
            let mut locations = Vec::with_capacity(count);
            IRShaderReflectionGetResourceLocations(reflection, locations.as_mut_ptr());
            locations.set_len(count);

            sys::IRShaderReflectionDestroy(reflection);
            sys::IRMetalLibBinaryDestroy(metal_lib);
            sys::IRObjectDestroy(dxil);
            sys::IRObjectDestroy(out_ir);

            sys::IRCompilerDestroy(compiler);

            let library =
                self.physical_device
                    .mtl_device
                    .newLibraryWithData_error(&DispatchData::new(
                        NonNull::new(bytecode.as_ptr() as _)
                            .ok_or(Error::MetalShaderConverter("Bytecode is null".to_owned()))?,
                        bytecode.len(),
                        None,
                        dispatch_block_t::default(),
                    ))?;

            let function = library
                .newFunctionWithName(&NSString::from_str(desc.entry_point.as_str()))
                .ok_or(Error::MetalError("Failed to create function".to_owned()))?;

            Ok(ShaderModule::Metal(MetalShaderModule::new(
                library, function, locations,
            )?))
        }
    }
}

fn ir_shader_stage(kind: ShaderKind) -> IRShaderStage {
    match kind {
        ShaderKind::Vertex => IRShaderStage_IRShaderStageVertex,
        ShaderKind::Fragment => IRShaderStage_IRShaderStageFragment,
        ShaderKind::Task => IRShaderStage_IRShaderStageAmplification,
        ShaderKind::Mesh => IRShaderStage_IRShaderStageMesh,
        ShaderKind::Compute => IRShaderStage_IRShaderStageCompute,
    }
}
