pub(crate) mod console;

use std::{cell::RefCell, sync::Arc};

use frclib_core::{value::{FrcEntry, IntoFrcValue}, units::time::{Time, Microsecond}};

/// A function that will be called when the data log is flushed.
///
/// Takes an Arc slice of all the data logged since the last flush,
/// this data type allows for low cost cloning for receivers that don't need ownership to log.
pub type TelemetryConsumer = fn(Arc<[FrcEntry]>);

#[linkme::distributed_slice]
pub static TELEMETRY_CONSUMERS: [TelemetryConsumer];

thread_local! {
    static TELEMETRY_CACHE: RefCell<Vec<FrcEntry>> = RefCell::new(Vec::with_capacity(128));
}


pub fn flush_datalog() {
    TELEMETRY_CACHE.with(|thread_cache| {
        //get cache capacity
        let capacity = thread_cache.borrow().capacity();
        //swap out the cache
        let cache: Vec<FrcEntry> = thread_cache.replace(Vec::with_capacity(capacity));
        let rc_cache: Arc<[FrcEntry]> = Arc::from(cache);
        //call all consumers
        for consumer in TELEMETRY_CONSUMERS {
            consumer(rc_cache.clone());
        }
    });
}

fn log_entry(data: FrcEntry) {
    TELEMETRY_CACHE.with(|thread_cache| {
        thread_cache.borrow_mut().push(data);
    });
}

pub fn log(key: &'static str, value: impl IntoFrcValue) {
    log_entry(FrcEntry {
        timestamp: u64::from(Microsecond::from(frclib_core::time::uptime())),
        value: value.into_frc_value(),
        key
    });
}

pub fn log_with_timestamp(key: &'static str, value: impl IntoFrcValue, timestamp: impl Time) {
    log_entry(FrcEntry {
        timestamp: u64::from(Microsecond::from(timestamp.standard())),
        value: value.into_frc_value(),
        key
    });
}


#[cfg(test)]
use frclib_core::value::FrcValue;

#[cfg(test)]
#[must_use]
pub fn get_latest_value(key: &'static str) -> Option<FrcValue> {
    TELEMETRY_CACHE.with(|thread_cache| {
        let cache = thread_cache.borrow();
        cache.iter().rev().find(|entry| entry.key == key).cloned().map(|e| e.value)
    })
}

/// Will assert that the latest value logged for the given key is equal to the given value.
/// This allows you to use the default logging system as a testing tool.
#[cfg(test)]
#[macro_export]
macro_rules! assert_emitted_value {
    ($key:expr, $value:expr) => {
        assert_eq!(std::datalogging::get_latest_value($key), Some($value.into_frc_value()));
    };
}