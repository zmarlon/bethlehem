use bethlehem::*;
use sdl3::event::Event;
use std::borrow::Cow;

fn main() {
    let sdl = sdl3::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Example", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    //Api Code
    let instance = Instance::new(&InstanceDesc {
        backend_type: BackendType::Vulkan,
        debug: true,
        engine_name: Cow::Borrowed("Test Engine"),
        application_name: Cow::Borrowed("Test App"),
        window_handle: WindowHandle::Sdl(&window),
    })
    .unwrap();

    println!("Instance created: {:?}", instance.backend());

    let physical_devices = instance.get_physical_devices().unwrap();

    for physical_device in &physical_devices {
        println!("Found physical device: {}", physical_device.name());
    }

    let device = instance
        .create_device(&DeviceDesc {
            physical_device: physical_devices.first().unwrap(),
        })
        .unwrap();

    let mesh_shader = device
        .create_shader_module(&ShaderDesc {
            name: "Mesh shader".into(),
            source: ShaderSource::Hlsl {
                source: MESH_SHADER.into(),
                defines: vec![],
            },
            kind: ShaderKind::Mesh,
            entry_point: "ms_main".into(),
        })
        .unwrap();

    let fragment_shader = device
        .create_shader_module(&ShaderDesc {
            name: "Fragment shader".into(),
            source: ShaderSource::Hlsl {
                source: FRAGMENT_SHADER.into(),
                defines: vec![],
            },
            kind: ShaderKind::Fragment,
            entry_point: "fs_main".into(),
        })
        .unwrap();

    //Game loop

    let mut event_pump = sdl.event_pump().unwrap();

    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                _ => {}
            }
        }
    }
}

static MESH_SHADER: &'static str = r#"
struct MSOutput {
    float4 Position: SV_Position;
    float3 Color: COLOR0;
};

[NumThreads(1, 1, 1)]
[OutputTopology("triangle")]
void ms_main(uint gtid: SV_GroupThreadID, uint gid: SV_GroupID, out indices uint3 triangles[124], out vertices MSOutput vertices[64]) {
    SetMeshOutputCounts(3, 1);
    triangles[0] = uint3(0, 1, 2);

    vertices[0].Position = float4(-0.5, 0.5, 0.0, 1.0);
    vertices[0].Color = float3(1.0, 0.0, 0.0);

    vertices[1].Position = float4(0.5, 0.5, 0.0, 1.0);
    vertices[1].Color = float3(0.0, 1.0, 0.0);

    vertices[2].Position = float4(0.0, -0.5, 0.0, 1.0);
    vertices[2].Color = float3(0.0, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER: &'static str = r#"
struct PSOutput {
    float4 color : SV_Target0;
};

PSOutput fs_main() {
    PSOutput output;
    output.color = float4(0.0, 1.0, 0.0, 1.0);
    return output;
}
"#;
