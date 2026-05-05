fn main() {
    let port = cargo_fuzz_example::parse_port(b"8080");
    println!("Parsed port: {port}");
}
