fn main() {
    let port = rust_fuzzy::parse_port(b"8080");
    println!("Parsed port: {port}");
}
