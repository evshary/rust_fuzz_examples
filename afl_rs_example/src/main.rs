fn main() {
    let port = afl_rs_example::parse_port(b"8080");
    println!("Parsed port: {port}");
}
