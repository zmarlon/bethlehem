use bethlehem::*;
use sdl3::event::Event;

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
        debug: false,
        engine_name: "Test Engine",
        application_name: "Test App",
        window_handle: WindowHandle::Sdl(&window),
    })
    .unwrap();

    println!("Instance created: {:?}", instance.backend());

    let physical_devices = instance.get_physical_devices().unwrap();

    for physical_device in &physical_devices {
        println!("Found physical device: {}", physical_device.name());
    }

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
