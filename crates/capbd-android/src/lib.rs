#[cfg(target_os = "android")]
use android_activity::AndroidApp;
use log::info;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("CapBD"),
    );

    info!("Initializing CapBD on Android via Native Activity...");

    loop {
        app.poll_events(
            Some(std::time::Duration::from_millis(16)),
            |event| match event {
                android_activity::PollEvent::Wake => {
                    info!("Android event loop woke");
                }
                android_activity::PollEvent::Timeout => {}
                android_activity::PollEvent::Main(main_event) => {
                    info!("Android lifecycle event: {:?}", main_event);
                }
                _ => {}
            },
        );
    }
}

#[cfg(not(target_os = "android"))]
pub fn capbd_android_host_stub() {
    info!("CapBD Android crate built on non-Android host target.");
}
