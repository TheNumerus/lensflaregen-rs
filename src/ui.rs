use glutin::{event::Event, PossiblyCurrent, WindowedContext};
use imgui::*;
use std::time::Duration;

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
    pub fn frame(&mut self, context: &WindowedContext<PossiblyCurrent>, delta: Duration, state: &mut [f32; 4]) {
        let io = self.imgui.io_mut();
        self.platform.prepare_frame(io, context.window()).expect("Failed to start frame");

        io.update_delta_time(delta);

        let ui = self.imgui.frame();

        let color = imgui::EditableColor::Float4(state);

        imgui::Window::new(im_str!("FPS counter"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(format!("FPS: {}", ui.io().framerate));
                imgui::ColorEdit::new(im_str!("Flare Color"), color).build(&ui);
            });
        self.renderer.render(ui);
    }
}
