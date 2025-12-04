use bethlehem::*;

fn main() {
    let instance = Instance::new(&InstanceDesc {
        backend_type: BackendType::Vulkan,
        debug: false,
        engine_name: "Test Engine",
        application_name: "Test App",
    })
    .unwrap();

    println!("Instance created: {:?}", instance.backend());

    let physical_devices = instance.get_physical_devices().unwrap();

    for physical_device in &physical_devices {
        println!("Found physical device: {}", physical_device.name());
    }
}
