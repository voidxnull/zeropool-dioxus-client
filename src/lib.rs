mod tabs;
mod ui;

use mobile_entry_point::mobile_entry_point;
use zeropool_dioxus_client_mobile::app;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("zeropool-dioxus-client-mobile"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    simple_logger::SimpleLogger::new().init().unwrap();
}

#[mobile_entry_point]
fn main() {
    init_logging();

    dioxus::desktop::launch_cfg(ui::app, |c| {
        c.with_window(|w| w.with_title("ZeroPool Client"))
    });
}
