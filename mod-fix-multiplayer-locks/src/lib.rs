use std::sync::atomic::{AtomicU16, Ordering};

pub(crate) mod ffi;

static PENDING_OPS_LOCK: AtomicU16 = AtomicU16::new(0);
static CURRENT_OPS_LOCK: AtomicU16 = AtomicU16::new(0);

pub(crate) fn lock(lock: &AtomicU16) {
    loop {
        if lock.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            break;
        }
    }
}

pub(crate) fn try_lock(lock: &AtomicU16) -> bool {
    lock.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_ok()
}

pub(crate) fn unlock(lock: &AtomicU16) {
    let old = lock.swap(0, Ordering::SeqCst);
    if old != 1 {
        panic!("Lock is dirty ({old})");
    }
}
