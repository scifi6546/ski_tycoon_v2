use gfx_hal::{prelude::*, window};
use nalgebra::Vector2;
#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "gl",
)))]
use ski_tycoon_v2::{Game, Window as GameWindow};
#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as back;
#[cfg(feature = "dx11")]
use ski_tycoon_dx11::{Game, Window as GameWindow};

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "dx12")]
use ski_tycoon_dx12::{Game, Window as GameWindow};

#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;

#[cfg(feature = "gl")]
use ski_tycoon_gl::{Game, Window as GameWindow};
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "metal")]
use ski_tycoon_metal::{Game, Window as GameWindow};

#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
#[cfg(feature = "vulkan")]
use ski_tycoon_vulkan::{Game, Window as GameWindow};
#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "gl",
)))]
extern crate gfx_backend_empty as back;

const DIMS: window::Extent2D = window::Extent2D {
    width: 1024,
    height: 768,
};
const DEFAULT_SIZE: &'static [u32] = &[1024, 1024];
fn main() {
    println!("Hello, world!");
    let event_loop = winit::event_loop::EventLoop::new();

    let wb = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(
            64.0, 64.0,
        )))
        .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize::new(
            DIMS.width,
            DIMS.height,
        )))
        .with_title("quad".to_string());

    // instantiate backend
    let window = wb.build(&event_loop).unwrap();
    let instance = back::Instance::create("gfx-rs quad", 1).expect("Failed to create an instance!");
    let adapter = {
        let mut adapters = instance.enumerate_adapters();
        for adapter in &adapters {
            println!("{:?}", adapter.info);
        }
        adapters.remove(0)
    };
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create a surface!")
    };

    let mut game = Game::new(
        Vector2::new(DEFAULT_SIZE[0], DEFAULT_SIZE[1]),
        GameWindow {
            instance,
            adapter,
            surface,
            window_dimensions: DIMS,
        },
    )
    .expect("failed to create game");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => (),
        }
        // todo accumulate events
        game.run_frame(vec![]);
    });
}
