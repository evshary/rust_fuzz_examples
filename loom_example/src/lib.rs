use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::thread;

pub fn publish_and_read() -> usize {
    let value = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let writer_value = Arc::clone(&value);
    let writer_ready = Arc::clone(&ready);
    let writer = thread::spawn(move || {
        writer_value.store(1, Ordering::Release);
        writer_ready.store(true, Ordering::Release);
    });

    let reader_value = Arc::clone(&value);
    let reader_ready = Arc::clone(&ready);
    let reader = thread::spawn(move || {
        while !reader_ready.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        reader_value.load(Ordering::Acquire)
    });

    writer.join().unwrap();
    reader.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::publish_and_read;

    #[test]
    fn publishes_the_value() {
        assert_eq!(publish_and_read(), 1);
    }
}
