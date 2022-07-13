mod ui;

fn main() {
    dioxus::desktop::launch_cfg(ui::app, |c| c.with_window(|w| w.with_title("ZeroPool Client")));
}

