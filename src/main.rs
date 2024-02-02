use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use vulkanalia::*;
use vulkanalia::prelude::v1_2::*;

fn main() {

    let entry: Entry;
    unsafe { entry = Entry::new(loader::LibloadingLoader(loader::LIBRARY)).map_err(|e| format!("{e}"))?; }
    
    let app_info = vk::ApplicationInfo::builder()
        .application_name(b"deez\0")
        .application_version(vk::make_version(0, 0, 0))
        .build();
    
    let instance_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .build();

    let instance = entry.create_instance(&instance_info);

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
}
