use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};

use glutin::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use imgui::*;
use imgui_opengl_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use simple_logger::SimpleLogger;

use rand::prelude::*;

pub mod gl_wrapper;
pub mod lfg;

use gl_wrapper::{
    framebuffer::Framebuffer,
    geometry,
    texture::{Texture2d, TextureFormat},
};
use lfg::{effect::Effect, shader_lib::ShaderLib};

/// run at 60 fps
const TARGET_FPS: u64 = 60;
const TARGET_MICROS: u64 = 1_000_000 / TARGET_FPS;

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut rng = rand::thread_rng();

    let (event_loop, context) = create_window();

    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| context.get_proc_address(s) as *const _);
    platform.attach_window(imgui.io_mut(), context.window(), HiDpiMode::Locked(1.0));

    let mut last_frame = Instant::now();

    let shader_lib = ShaderLib::new().context("Shader compilation error")?;
    let mut effect = Effect::new();

    let mut flare_color = [1.0_f32, 1.0, 1.0, 1.0];

    let mut noise_data = [0.5; 64 * 64 * 4];
    for val in &mut noise_data {
        *val = rng.gen();
    }

    let main_hdr_buf = Framebuffer::hdr(1280, 720);
    let side_hdr_buf = Framebuffer::hdr(1280, 720);

    let noise = Texture2d::new(64, 64, &noise_data, TextureFormat::Rgba);

    let quad = geometry::quad();

    let mut cap = true;

    let mut state = WindowState::default();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        platform.handle_event(imgui.io_mut(), context.window(), &event);

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
                main_hdr_buf.resize(size.width, size.height);
                side_hdr_buf.resize(size.width, size.height);
                state.size = (size.width, size.height);
            },
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                ..
            } => {
                *new_inner_size = context.window().inner_size();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                state.cursor = (position.x as u32, position.y as u32);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(glutin::event::VirtualKeyCode::Space),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                cap = !cap;
            }
            Event::RedrawRequested(_) => unsafe {
                let now = Instant::now();
                let delta = now - last_frame;
                last_frame = now;

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);

                effect.flare.set_color(&flare_color);
                effect.draw(&shader_lib, &noise, &main_hdr_buf, &side_hdr_buf, &quad, &state);

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                shader_lib.tonemap.bind();
                main_hdr_buf.bind_as_color_texture(0);

                quad.draw();

                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);

                imgui_draw(&mut imgui, &platform, &context, delta, &renderer, &mut flare_color);
                context.swap_buffers().unwrap();
                context.window().request_redraw();

                if cap {
                    let frame_delta = Instant::now() - now;
                    if (frame_delta.as_micros() as u64) < TARGET_MICROS {
                        thread::sleep(Duration::from_micros(TARGET_MICROS) - frame_delta);
                    }
                }
            },
            _ => (),
        }
    });
}

fn create_window() -> (EventLoop<()>, glutin::ContextWrapper<PossiblyCurrent, glutin::window::Window>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_title("Lens Flare Generator");
    let context = ContextBuilder::new().with_srgb(false).build_windowed(window, &event_loop).unwrap();
    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(|s| context.get_proc_address(s) as *const _);
    (event_loop, context)
}

fn imgui_draw(
    imgui: &mut Context,
    platform: &WinitPlatform,
    windowed_context: &WindowedContext<PossiblyCurrent>,
    delta: Duration,
    renderer: &Renderer,
    color: &mut [f32; 4],
) {
    let io = imgui.io_mut();
    platform.prepare_frame(io, windowed_context.window()).expect("Failed to start frame");

    io.update_delta_time(delta);

    let ui = imgui.frame();

    let color = imgui::EditableColor::Float4(color);

    imgui::Window::new(im_str!("FPS counter"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(format!("FPS: {}", ui.io().framerate));
            imgui::ColorEdit::new(im_str!("Flare Color"), color).build(&ui);
        });

    renderer.render(ui);
}

#[derive(Debug, Default)]
pub struct WindowState {
    size: (u32, u32),
    cursor: (u32, u32),
}

impl WindowState {
    fn relative_cursor(&self) -> (f32, f32) {
        (self.cursor.0 as f32 / self.size.0 as f32, 1.0 - self.cursor.1 as f32 / self.size.1 as f32)
    }
}
