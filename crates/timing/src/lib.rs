use chrono::{Duration, Local};
use log::log;
use log::{log_enabled, Level};

pub fn run<T, F: FnMut() -> T>(mut f: F) -> (T, Duration) {
    let start = Local::now();
    let result = f();
    let end = Local::now();

    (result, end - start)
}

pub fn log_run<T, F: FnMut() -> T>(level: Level, prefix: &str, mut f: F) -> T {
    if log_enabled!(level) {
        let (result, duration) = run(f);

        log!(level, "{prefix} duration: {duration}",);

        result
    } else {
        f()
    }
}

macro_rules! generate_level_run {
    ($(($name: ident, $level: path)),* $(,)?) => {
        $(
            pub mod $name {
                use log::Level;

                pub fn log_run<T, F: FnMut() -> T>(prefix: &str, f: F) -> T {
                    $crate::log_run($level, prefix, f)
                }
            }
        )*
    };
}

generate_level_run!(
    (error, Level::Error),
    (warn, Level::Warn),
    (info, Level::Info),
    (debug, Level::Debug),
    (trace, Level::Trace),
);
