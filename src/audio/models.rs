use std::sync::atomic::AtomicU8;

pub struct Levels {
    left_peak: AtomicU8,
    right_peak: AtomicU8,
}
