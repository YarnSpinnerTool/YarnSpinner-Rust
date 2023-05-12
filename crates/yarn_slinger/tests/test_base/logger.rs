use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use yarn_slinger::log::{self, Level, Metadata, Record};

pub(crate) struct TestLogger {
    runtime_errors_cause_failure: Arc<AtomicBool>,
}

impl TestLogger {
    pub(crate) fn new(runtime_errors_cause_failure: Arc<AtomicBool>) -> Self {
        Self {
            runtime_errors_cause_failure,
        }
    }
}

impl log::Log for TestLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = record.args().to_string();
            match record.level() {
                Level::Debug => {
                    println!("{msg}")
                }
                Level::Error => {
                    eprintln!("{msg}");
                    if self.runtime_errors_cause_failure.load(Ordering::Relaxed) {
                        assert!(!msg.is_empty())
                    }
                }
                _ => {}
            }
        }
    }

    fn flush(&self) {}
}
