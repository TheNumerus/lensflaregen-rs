use std::time::{Duration, Instant};

use gl_wrapper::shader::Shader;
use glutin::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use simple_logger::SimpleLogger;

use anyhow::{Context as _, Result};

use imgui::*;
use imgui_opengl_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use lfg::flare::Flare;

pub mod gl_wrapper;
pub mod lfg;

const COMMON_SHADER: &str = include_str!("../shaders/common.glsl");

const FLARE_VERT: &str = include_str!("../shaders/quad.vert");
const FLARE_FRAG: &str = include_str!("../shaders/flare.frag");

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_title("Lens Flare Generator");

    let windowed_context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| windowed_context.get_proc_address(s) as *const _);
    platform.attach_window(imgui.io_mut(), windowed_context.window(), HiDpiMode::Locked(1.0));

    let mut last_frame = Instant::now();

    let flare_s = Shader::from_str(FLARE_VERT, FLARE_FRAG).context("Flare shader compilation error")?;

    let mut flare = Flare::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        platform.handle_event(imgui.io_mut(), windowed_context.window(), &event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => unsafe {
                gl::Viewport(0, 0, size.width as i32, size.height as i32);
            },
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                ..
            } => {
                *new_inner_size = windowed_context.window().inner_size();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let size = windowed_context.window().inner_size();
                let (pos_x, pos_y) = (position.x / size.width as f64, 1.0 - position.y / size.height as f64);
                flare.set_position(pos_x as f32, pos_y as f32);
            }
            Event::RedrawRequested(_) => unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                flare.draw(&flare_s);

                let now = Instant::now();
                let delta = now - last_frame;
                last_frame = now;
                imgui_draw(&mut imgui, &platform, &windowed_context, delta, &renderer);
                windowed_context.swap_buffers().unwrap();
                windowed_context.window().request_redraw();
            },
            _ => (),
        }
    });
}

fn imgui_draw(imgui: &mut Context, platform: &WinitPlatform, windowed_context: &WindowedContext<PossiblyCurrent>, delta: Duration, renderer: &Renderer) {
    let io = imgui.io_mut();
    platform.prepare_frame(io, windowed_context.window()).expect("Failed to start frame");

    io.update_delta_time(delta);

    let ui = imgui.frame();

    Window::new(im_str!("FPS counter"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(format!("FPS: {}", ui.io().framerate));
        });

    renderer.render(ui);
}
