use anyhow::{Context as _, Result};

use glutin::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::ControlFlow,
};

use simple_logger::SimpleLogger;

pub mod fps_cap;
pub mod lfg;
pub mod ui;
pub mod window;
pub mod window_state;

use fps_cap::FpsCap;
use gl_wrapper::{
    framebuffer::Framebuffer,
    geometry,
    state::{Blend, State},
    texture::{Texture2d, TextureFormat},
};
use lfg::{effect::Effect, ghost, shader_lib::ShaderLib};
use window::Window;

const SPECTRUM_BYTES: &[u8] = include_bytes!("../images/spectral.png");
const NOISE_BYTES: &[u8] = include_bytes!("../images/noise.png");

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let window = Window::new();
    let mut fps_cap = FpsCap::with_target_fps(60);

    let shader_lib = ShaderLib::new().context("Shader compilation error")?;
    let mut effect = Effect::new();

    let mut main_hdr_buf = Framebuffer::hdr(1280, 720);
    let mut side_hdr_buf = Framebuffer::hdr(1280, 720);

    let quad = geometry::quad();
    let ghost_geo = ghost::gen_ghost_geo(8);

    let noise = texture_from_bytes(NOISE_BYTES)?;
    let spectrum_texture = texture_from_bytes(SPECTRUM_BYTES)?;

    window.run(move |event, _, control_flow, ui, context, state| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                State::viewport(0, 0, size.width, size.height);
                main_hdr_buf.resize(size.width, size.height);
                side_hdr_buf.resize(size.width, size.height);
                state.size = (size.width, size.height);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                *new_inner_size = context.window().inner_size();
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.cursor = (position.x as u32, position.y as u32);
                effect.set_position(state.relative_cursor());
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
                state.fps_capped = !state.fps_capped;
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            context.window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            let delta = fps_cap.delta();

            Framebuffer::draw_with_default(|fb| {
                fb.clear();
                State::blend(Blend::Enable(gl::SRC_ALPHA, gl::ONE));
            });

            noise.bind(2);
            spectrum_texture.bind(3);

            effect.draw(&shader_lib, &mut main_hdr_buf, &mut side_hdr_buf, &quad, &ghost_geo, &state);

            Framebuffer::draw_with_default(|_fb| {
                shader_lib.tonemap.bind();
                shader_lib.tonemap.set_int_uniform("tonemap", [effect.tonemap as i32]);
                main_hdr_buf.bind_as_color_texture(0);

                quad.draw();
            });

            Framebuffer::bind_default();

            ui.frame(context, delta, &mut effect);

            context.swap_buffers().unwrap();

            if state.fps_capped {
                fps_cap.cap();
            }
        }
        _ => (),
    });
}

fn texture_from_bytes(bytes: &[u8]) -> Result<Texture2d> {
    let img = image::load_from_memory(bytes)?;
    let buf = img.as_rgba8().unwrap();

    Ok(Texture2d::new(buf.width(), buf.height(), buf.as_flat_samples().samples, TextureFormat::Rgba))
}
