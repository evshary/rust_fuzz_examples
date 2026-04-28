use arbitrary::{Arbitrary, Result, Unstructured};

// `Arbitrary` teaches the crate how to build this type from raw bytes.
// In real fuzzing, a fuzzer would mutate bytes first, then `arbitrary`
// would decode those bytes into a structured Rust value like this.
#[derive(Debug, Arbitrary, PartialEq, Eq)]
pub struct PortInput {
    pub port: u16,
    pub padded: bool,
}

// Turn the structured input back into the string form our parser expects.
pub fn render_port_input(input: &PortInput) -> String {
    if input.padded {
        format!(" {} ", input.port)
    } else {
        input.port.to_string()
    }
}

pub fn parse_port_text(text: &str) -> u16 {
    let trimmed = text.trim();

    if trimmed == "0" {
        panic!("bug: port 0 is handled incorrectly");
    }

    trimmed.parse::<u16>().unwrap_or_default()
}

// `Unstructured` is the core `arbitrary` API. It wraps raw bytes and lets
// us ask for typed values from them with `input.arbitrary::<T>()`.
pub fn parse_port_from_bytes(data: &[u8]) -> Result<u16> {
    let mut input = Unstructured::new(data);
    let port_input: PortInput = input.arbitrary()?;
    Ok(parse_port_text(&render_port_input(&port_input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_ports() {
        assert_eq!(parse_port_text("8080"), 8080);
    }

    #[test]
    fn invalid_input_falls_back_to_zero() {
        assert_eq!(parse_port_text("not a number"), 0);
    }

    #[test]
    fn arbitrary_can_build_a_structured_input() {
        let mut input = Unstructured::new(&[1; 32]);
        // Request one typed value from the byte buffer.
        let port_input = input.arbitrary::<PortInput>().unwrap();
        let rendered = render_port_input(&port_input);

        assert!(!rendered.is_empty());
    }

    #[test]
    fn raw_bytes_can_drive_the_target() {
        let parsed = parse_port_from_bytes(&[1; 32]).unwrap();
        assert!(parsed <= u16::MAX);
    }
}
