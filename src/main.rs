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
    ContextBuilder, PossiblyCurrent,
};

use simple_logger::SimpleLogger;

use rand::prelude::*;

pub mod gl_wrapper;
pub mod lfg;
pub mod ui;

use gl_wrapper::{
    framebuffer::{Blend, Framebuffer},
    geometry,
    texture::{Texture2d, TextureFormat},
};
use lfg::{effect::Effect, ghost, shader_lib::ShaderLib};
use ui::ImguiUi;

/// run at 60 fps
const TARGET_FPS: u64 = 60;
const TARGET_MICROS: u64 = 1_000_000 / TARGET_FPS;

const SPECTRUM_BYTES: &[u8] = include_bytes!("../images/spectral.png");

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let (event_loop, context) = create_window();

    let mut ui = ImguiUi::init(&context);

    let mut last_frame = Instant::now();

    let shader_lib = ShaderLib::new().context("Shader compilation error")?;
    let mut effect = Effect::new();

    let mut flare_color = [1.0_f32, 1.0, 1.0, 1.0];

    let mut main_hdr_buf = Framebuffer::hdr(1280, 720);
    let mut side_hdr_buf = Framebuffer::hdr(1280, 720);

    let quad = geometry::quad();
    let ghost_geo = ghost::gen_ghost_geo(8);

    let mut cap = true;

    let mut state = WindowState::default();

    let noise = generate_noise_texture();
    let spectrum_texture = generate_spectrum_texture()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        ui.handle_events(&context, &event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => unsafe {
                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                    main_hdr_buf.resize(size.width, size.height);
                    side_hdr_buf.resize(size.width, size.height);
                    state.size = (size.width, size.height);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    *new_inner_size = context.window().inner_size();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    state.cursor = (position.x as u32, position.y as u32);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(glutin::event::VirtualKeyCode::Space),
                            ..
                        },
                    ..
                } => {
                    cap = !cap;
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let delta = now - last_frame;
                last_frame = now;

                Framebuffer::draw_with_default(|fb| {
                    fb.clear();
                    fb.blend(Blend::Enable(gl::SRC_ALPHA, gl::ONE));
                });

                effect.flare.set_color(&flare_color);
                effect.draw(
                    &shader_lib,
                    &noise,
                    &mut main_hdr_buf,
                    &mut side_hdr_buf,
                    &quad,
                    &ghost_geo,
                    &state,
                    &spectrum_texture,
                );

                Framebuffer::draw_with_default(|_fb| {
                    shader_lib.tonemap.bind();
                    main_hdr_buf.bind_as_color_texture(0);

                    quad.draw();
                });

                ui.frame(&context, delta, &mut flare_color);

                context.swap_buffers().unwrap();
                context.window().request_redraw();

                if cap {
                    let frame_delta = Instant::now() - now;
                    if (frame_delta.as_micros() as u64) < TARGET_MICROS {
                        thread::sleep(Duration::from_micros(TARGET_MICROS) - frame_delta);
                    }
                }
            }
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

fn generate_noise_texture() -> Texture2d {
    let mut rng = rand::thread_rng();
    let mut noise_data = [0.5; 64 * 64 * 4];
    for val in &mut noise_data {
        *val = rng.gen();
    }
    Texture2d::new(64, 64, &noise_data, TextureFormat::Rgba)
}

fn generate_spectrum_texture() -> Result<Texture2d> {
    let spectrum_img = image::load_from_memory(SPECTRUM_BYTES)?;
    let spectrum = spectrum_img.as_rgba8().unwrap();

    Ok(Texture2d::new(
        spectrum.width(),
        spectrum.height(),
        spectrum.as_flat_samples().samples,
        TextureFormat::Srgba,
    ))
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
