use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = honggfuzz_rs_example::parse_port(data);
        });
    }
}
