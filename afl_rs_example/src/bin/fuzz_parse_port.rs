use afl::fuzz;

fn main() {
    fuzz!(|data: &[u8]| {
        let _ = afl_rs_example::parse_port(data);
    });
}
