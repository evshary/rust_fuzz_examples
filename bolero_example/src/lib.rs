pub fn parse_port(input: &str) -> u16 {
    let trimmed = input.trim();

    if trimmed == "0" {
        panic!("bug: port 0 is handled incorrectly");
    }

    trimmed.parse::<u16>().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::parse_port;

    #[test]
    fn parses_numeric_ports() {
        assert_eq!(parse_port("8080"), 8080);
    }

    #[test]
    fn invalid_input_falls_back_to_zero() {
        assert_eq!(parse_port("not a number"), 0);
    }
}
