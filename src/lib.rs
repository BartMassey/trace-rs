use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

static DEBUG: AtomicBool = AtomicBool::new(false);

#[macro_export]
macro_rules! trace {
    ($fmt:expr, $($args:expr),*) => {
        if DEBUG.load(SeqCst) {
            println!($fmt, $($args),*);
        }
    }
}

pub fn trace(status: bool) {
    DEBUG.store(status, SeqCst);
}
