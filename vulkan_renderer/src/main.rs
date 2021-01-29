use nalgebra::Vector2;
use ski_tycoon_v2::Game;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
const DEFAULT_SIZE: &'static [u32] = &[1024, 1024];
fn main() {
    println!("Hello, world!");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ski tycoon")
        .with_inner_size(winit::dpi::LogicalSize::new(
            DEFAULT_SIZE[0] as f32,
            DEFAULT_SIZE[1] as f32,
        ))
        .build(&event_loop)
        .unwrap();
    let _game = Game::new(Vector2::new(DEFAULT_SIZE[0], DEFAULT_SIZE[1]), &window);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
