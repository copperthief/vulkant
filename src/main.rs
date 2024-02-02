use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use vulkanalia::prelude::v1_2::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};

fn main() {

    let instance: Instance;

    unsafe {
        let loader = LibloadingLoader::new(LIBRARY).unwrap();
        let entry = Entry::new(loader).unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(b"deez\0")
            .application_version(vk::make_version(0, 0, 0))
            .build();
    
        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .build();

        instance = entry.create_instance(&instance_info, None).unwrap();
    }

    

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("bird window (if not bird, do not look)")
        .build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll); //update window even without events
    
    event_loop.run(move |event, target| {
        match event {
            Event::AboutToWait => {
                //render here
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                target.exit();
            },
            _ => ()
        }
    }).unwrap();



    unsafe { instance.destroy_instance(None); }
}
