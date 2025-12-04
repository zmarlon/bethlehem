use crate::{Error, ShaderDesc, ShaderKind, ShaderSource};

fn get_target(stage: ShaderKind) -> &'static str {
    match stage {
        ShaderKind::Vertex => "vs_6_7",
        ShaderKind::Task => "as_6_7",
        ShaderKind::Mesh => "ms_6_7",
        ShaderKind::Fragment => "ps_6_7",
        ShaderKind::Compute => "cs_6_7",
    }
}

pub fn compile_hlsl(desc: &ShaderDesc, spirv: bool) -> Result<Vec<u8>, Error> {
    let mut args = vec!["-HV 2021"];
    if spirv {
        args.push("-spirv");
        args.push("-fspv-target-env=vulkan1.3")
    }

    match &desc.source {
        ShaderSource::Hlsl { source, defines } => {
            let defines = defines
                .iter()
                .map(|(k, v)| (k.as_ref(), v.as_deref()))
                .collect::<Vec<_>>();
            let shader = hassle_rs::compile_hlsl(
                &desc.name,
                source,
                &desc.entry_point,
                get_target(desc.kind),
                &args,
                &defines,
            )?;

            Ok(shader)
        }
        _ => Err(Error::InvalidShaderSource),
    }
}
