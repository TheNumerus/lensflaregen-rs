use std::time::Instant;

use glutin::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};

use crate::{ui::ImguiUi, window_state::WindowState};

pub struct Window {
    ui: ImguiUi,
    event_loop: EventLoop<()>,
    context: WindowedContext<PossiblyCurrent>,
    state: WindowState,
}

impl Window {
    pub fn with_size(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title("Lens Flare Generator");
        let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
        let context = unsafe { context.make_current().unwrap() };
        gl::load_with(|s| context.get_proc_address(s) as *const _);

        let ui = ImguiUi::init(&context);
        let state = WindowState::default();

        Self {
            event_loop,
            context,
            ui,
            state,
        }
    }

    pub fn run<F>(self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(Event<()>, &EventLoopWindowTarget<()>, &mut ControlFlow, &mut ImguiUi, &WindowedContext<PossiblyCurrent>, &mut WindowState),
    {
        let Self {
            event_loop,
            mut ui,
            context,
            mut state,
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, target, flow| {
            *flow = ControlFlow::Poll;

            match &event {
                Event::NewEvents(_) => {
                    let delta = Instant::now() - last_frame;
                    last_frame = Instant::now();
                    ui.imgui_mut().io_mut().update_delta_time(delta);
                }
                Event::MainEventsCleared => {
                    ui.prepare_frame(&context);
                }
                _ => {
                    ui.handle_events(&context, &event);
                }
            }

            event_handler(event, target, flow, &mut ui, &context, &mut state);
        });
    }
}
