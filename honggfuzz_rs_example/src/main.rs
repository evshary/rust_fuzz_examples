fn main() {
    let port = honggfuzz_rs_example::parse_port(b"8080");
    println!("Parsed port: {port}");
}
