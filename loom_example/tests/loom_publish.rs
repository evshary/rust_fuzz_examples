#[test]
fn loom_publish_and_read() {
    #[cfg(loom)]
    loom::model(|| {
        use loom::sync::{
            atomic::{AtomicBool, AtomicUsize, Ordering},
            Arc,
        };
        use loom::thread;

        let value = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));

        let writer_value = Arc::clone(&value);
        let writer_ready = Arc::clone(&ready);
        let writer = thread::spawn(move || {
            writer_ready.store(true, Ordering::Release);
            // We add this line to trigger the bug intentionally
            thread::yield_now();
            writer_value.store(1, Ordering::Release);
        });

        let reader_value = Arc::clone(&value);
        let reader_ready = Arc::clone(&ready);
        let reader = thread::spawn(move || {
            while !reader_ready.load(Ordering::Acquire) {
                loom::thread::yield_now();
            }
            let observed = reader_value.load(Ordering::Acquire);
            assert_eq!(observed, 1);
        });

        writer.join().unwrap();
        reader.join().unwrap();
    });

    #[cfg(not(loom))]
    {
        // This test target is intended to be run with `RUSTFLAGS="--cfg loom"`.
        // Under a normal `cargo test`, we keep it as a no-op so the crate still builds.
    }
}
