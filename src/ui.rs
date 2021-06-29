use glutin::{event::Event, PossiblyCurrent, WindowedContext};
use imgui::{im_str, Condition, SliderFlags, StyleColor, Ui};

use crate::{
    lfg::{effect::Effect, flare::FlareStyle},
    window_state::WindowState,
};

pub struct ImguiUi {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_opengl_renderer::Renderer,
}

impl ImguiUi {
    pub fn init(context: &WindowedContext<PossiblyCurrent>) -> Self {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);

        let style = imgui.style_mut();
        style.window_rounding = 5.0;
        style.colors[StyleColor::WindowBg as usize][3] = 0.8;

        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| context.get_proc_address(s) as *const _);

        platform.attach_window(imgui.io_mut(), context.window(), imgui_winit_support::HiDpiMode::Locked(1.0));

        Self { imgui, platform, renderer }
    }

    pub fn handle_events(&mut self, context: &WindowedContext<PossiblyCurrent>, event: &Event<()>) {
        let mut io = self.imgui.io_mut();
        self.platform.handle_event(&mut io, context.window(), event);
    }

    pub fn prepare_frame(&mut self, context: &WindowedContext<PossiblyCurrent>) {
        let io = self.imgui.io_mut();
        self.platform.prepare_frame(io, context.window()).expect("Failed to start frame");
    }

    pub fn render_frame(&mut self, context: &WindowedContext<PossiblyCurrent>, effect: &mut Effect, state: &mut WindowState) {
        let ui = self.imgui.frame();

        state.ui_focused = ui.is_any_item_active();

        imgui::Window::new(im_str!("Effect settings"))
            .size([400.0, 120.0], Condition::FirstUseEver)
            .build(&ui, || {
                Self::window_build(&ui, effect);
            });
        self.platform.prepare_render(&ui, context.window());
        self.renderer.render(ui);
    }

    fn window_build(ui: &Ui, effect: &mut Effect) {
        use imgui::{ColorEdit, EditableColor, Slider};

        ui.text(format!("FPS: {}", ui.io().framerate));

        if imgui::CollapsingHeader::new(im_str!("Effect")).default_open(true).build(&ui) {
            Slider::new(im_str!("Samples")).range(1..=128).build(&ui, &mut effect.samples);
            ui.checkbox(im_str!("Tonemap"), &mut effect.tonemap);
        }

        if imgui::CollapsingHeader::new(im_str!("Flare")).default_open(true).build(&ui) {
            Slider::new(im_str!("Intensity")).range(0.0..=5.0).build(&ui, &mut effect.flare.intensity);
            Slider::new(im_str!("Ray Intensity"))
                .range(0.0..=5.0)
                .build(&ui, &mut effect.flare.ray_intensity);

            Slider::new(im_str!("Size")).range(0.0..=100.0).build(&ui, &mut effect.flare.size);
            ColorEdit::new(im_str!("Color"), EditableColor::Float4(&mut effect.flare.color)).build(&ui);

            let mut anam = match effect.flare.style {
                FlareStyle::Normal => false,
                FlareStyle::Anamorphic => true,
            };
            if ui.checkbox(im_str!("Anamorphic flare"), &mut anam) {
                match anam {
                    true => effect.flare.style = FlareStyle::Anamorphic,
                    false => effect.flare.style = FlareStyle::Normal,
                }
            }
        }

        if imgui::CollapsingHeader::new(im_str!("Ghosts")).default_open(true).build(&ui) {
            for (idx, ghost) in &mut effect.ghosts.iter_mut().enumerate() {
                Slider::new(im_str!("Intensity {}", idx).as_ref())
                    .range(0.0..=5.0)
                    .build(&ui, &mut ghost.intensity);
                ColorEdit::new(im_str!("Color {}", idx).as_ref(), EditableColor::Float4(&mut ghost.color)).build(&ui);

                Slider::new(im_str!("Size {}", idx).as_ref()).range(0.0..=100.0).build(&ui, &mut ghost.size);

                Slider::new(im_str!("Offset {}", idx).as_ref()).range(-5.0..=5.0).build(&ui, &mut ghost.offset);

                Slider::new(im_str!("Perpendicular Offset {}", idx).as_ref())
                    .range(-5.0..=5.0)
                    .build(&ui, &mut ghost.perpendicular_offset);

                Slider::new(im_str!("Center Transparency {}", idx).as_ref())
                    .range(0.0..=20.0)
                    .build(&ui, &mut ghost.center_transparency);

                Slider::new(im_str!("Aspect Ratio {}", idx).as_ref())
                    .range(0.001..=100.0)
                    .flags(SliderFlags::LOGARITHMIC)
                    .build(&ui, &mut ghost.aspect_ratio);

                Slider::new(im_str!("Distortion {}", idx).as_ref())
                    .range(0.0..=1.0)
                    .build(&ui, &mut ghost.distortion);

                Slider::new(im_str!("Dispersion {}", idx).as_ref())
                    .range(-1.0..=1.0)
                    .build(&ui, &mut ghost.dispersion);

                let mut disp_center = match ghost.dispersion_center {
                    crate::lfg::ghost::DispersionCenter::Ghost => true,
                    crate::lfg::ghost::DispersionCenter::Image => false,
                };

                if ui.checkbox(im_str!("Disperse from ghost center"), &mut disp_center) {
                    match disp_center {
                        true => ghost.dispersion_center = crate::lfg::ghost::DispersionCenter::Ghost,
                        false => ghost.dispersion_center = crate::lfg::ghost::DispersionCenter::Image,
                    }
                }

                ui.separator();
            }
        }
    }

    /// Get a mutable reference to the imgui ui's imgui.
    pub fn imgui_mut(&mut self) -> &mut imgui::Context {
        &mut self.imgui
    }
}
