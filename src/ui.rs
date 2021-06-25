use glutin::{event::Event, PossiblyCurrent, WindowedContext};
use imgui::{im_str, Condition, Ui};
use std::time::Duration;

use crate::lfg::{effect::Effect, flare::FlareStyle};

pub struct ImguiUi {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_opengl_renderer::Renderer,
}

impl ImguiUi {
    pub fn init(context: &WindowedContext<PossiblyCurrent>) -> Self {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);

        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| context.get_proc_address(s) as *const _);

        platform.attach_window(imgui.io_mut(), context.window(), imgui_winit_support::HiDpiMode::Locked(1.0));

        Self { imgui, platform, renderer }
    }

    pub fn handle_events(&mut self, context: &WindowedContext<PossiblyCurrent>, event: &Event<()>) {
        self.platform.handle_event(self.imgui.io_mut(), context.window(), event);
    }

    // TODO have proper state
    pub fn frame(&mut self, context: &WindowedContext<PossiblyCurrent>, delta: Duration, state: &mut Effect) {
        let io = self.imgui.io_mut();
        self.platform.prepare_frame(io, context.window()).expect("Failed to start frame");

        io.update_delta_time(delta);

        let ui = self.imgui.frame();

        imgui::Window::new(im_str!("FPS counter"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(&ui, || {
                Self::window_build(&ui, state);
            });
        self.renderer.render(ui);
    }

    fn window_build(ui: &Ui, state: &mut Effect) {
        use imgui::{ColorEdit, EditableColor, Slider};

        ui.text(format!("FPS: {}", ui.io().framerate));
        Slider::new(im_str!("Flare Intensity")).range(0.0..=5.0).build(&ui, &mut state.flare.intensity);
        ColorEdit::new(im_str!("Flare Color"), EditableColor::Float4(&mut state.flare.color)).build(&ui);

        let mut anam = match state.flare.style {
            FlareStyle::Normal => false,
            FlareStyle::Anamorphic => true,
        };
        if ui.checkbox(im_str!("Anamorphic flare"), &mut anam) {
            match anam {
                true => state.flare.style = FlareStyle::Anamorphic,
                false => state.flare.style = FlareStyle::Normal,
            }
        }

        Slider::new(im_str!("Samples")).range(1..=128).build(&ui, &mut state.samples);
        ui.checkbox(im_str!("Tonemap"), &mut state.tonemap);
    }
}
