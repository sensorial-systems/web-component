use crate::prelude::*;

pub struct Runner {

}

impl Default for Runner {
    fn default() -> Self {
        Self {}
    }
}

impl Runner {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "desktop")]
    pub fn run(&self, f: fn() -> Element) {
        use dioxus::desktop::{LogicalPosition, LogicalSize};
        // Init logger
        dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
        let monitor = {
            let event_loop = dioxus::desktop::tao::event_loop::EventLoop::new();
            let window = dioxus::desktop::tao::window::WindowBuilder::new()
                .with_visible(false).build(&event_loop).expect("Failed to create window");
            window.available_monitors().next().expect("No monitor found")
        };
        let window_size = LogicalSize::new(800.0, 600.0);
        let size = monitor.size();
        let position = LogicalPosition::new((size.width as f64 - window_size.width) / 2.0, (size.height as f64 - window_size.height) / 2.0);
        let window = dioxus::desktop::WindowBuilder::new()
            .with_title("File Explorer")
            .with_inner_size(window_size)
            .with_position(position)
            .with_transparent(true)
            // .with_decorations(false)
            .with_always_on_top(true);
        let cfg = dioxus::desktop::Config::new()
            .with_window(window)
            .with_menu(None)
            .with_disable_context_menu(true);
        LaunchBuilder::desktop()
            .with_cfg(cfg)
            .launch(Application);
    }

    #[cfg(not(feature = "desktop"))]
    pub fn run(&self, f: fn() -> Element) {
        LaunchBuilder::web()
            .launch(f);
    }
}