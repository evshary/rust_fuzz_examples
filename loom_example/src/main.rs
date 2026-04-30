fn main() {
    let value = loom_example::publish_and_read();
    println!("Published value: {value}");
}
