use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
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
