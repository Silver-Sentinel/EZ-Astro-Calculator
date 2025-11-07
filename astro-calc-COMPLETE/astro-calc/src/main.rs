use dioxus_desktop::{Config, WindowBuilder, LogicalSize};

mod app;
mod components;

fn main() {
    // Initialize Swiss Ephemeris (same as before!)
    astro_calc::init_sweph();

    // Configure the desktop window
    let config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("Astro Calculator - Fagan-Bradley Sidereal")
                .with_inner_size(LogicalSize::new(1400.0, 900.0))
                .with_resizable(true)
                .with_minimizable(true)
                .with_maximizable(true),
        );

    // Launch the app
    dioxus_desktop::launch::launch(app::App, vec![], config);
}
