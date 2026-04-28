use arbitrary_example::{PortInput, parse_port_text, render_port_input};

fn main() {
    let input = PortInput {
        port: 8080,
        padded: false,
    };
    let text = render_port_input(&input);
    let port = parse_port_text(&text);

    println!("Generated input: {input:?}");
    println!("Parsed port: {port}");
}
