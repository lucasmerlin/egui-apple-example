use eframe::{NativeOptions};

fn _main(options: NativeOptions) {
    let mut demo = egui_demo_lib::DemoWindows::default();

    eframe::run_simple_native("egui mobile example", options, move |ctx, _frame| {
        demo.ui(ctx);
    }).unwrap();
}


#[cfg(any(target_os = "ios", target_os = "android"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(target_os = "ios")]
fn _start_app() {
    stop_unwind(|| main());
}

#[no_mangle]
#[inline(never)]
#[cfg(target_os = "ios")]
pub extern "C" fn start_app() {
    _start_app();
}

#[cfg(not(target_os = "android"))]
pub fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .parse_default_env()
        .init();

    _main(NativeOptions::default());
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Warn),
    );

    let mut opts = NativeOptions::default();
    opts.event_loop_builder = Some(Box::new(move |el| {
        el.with_android_app(app);
    }));

    stop_unwind(|| _main(opts));
}
