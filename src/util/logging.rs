use error_iter::ErrorIter as _;
use log::error;

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
#[macro_export]
macro_rules! dev {
    ($($arg:tt)*) => {
        log::debug!(target: "DEV", $($arg)*);
    }
}
