#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = cargo_fuzzy_example::parse_port(data);
});
