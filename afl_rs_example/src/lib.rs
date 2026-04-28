pub fn parse_port(input: &[u8]) -> u16 {
    let text = match std::str::from_utf8(input) {
        Ok(text) => text,
        Err(_) => return 0,
    };
    let trimmed = text.trim();

    if trimmed == "0" {
        std::process::abort();
    }

    trimmed.parse::<u16>().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::parse_port;

    #[test]
    fn parses_numeric_ports() {
        assert_eq!(parse_port(b"8080"), 8080);
    }

    #[test]
    fn invalid_input_falls_back_to_zero() {
        assert_eq!(parse_port(b"not a number"), 0);
    }

    #[test]
    fn invalid_utf8_falls_back_to_zero() {
        assert_eq!(parse_port(&[0xff, 0xfe]), 0);
    }
}
