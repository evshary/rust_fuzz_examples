use bolero_example::parse_port;

#[test]
fn parse_port_matches_std() {
    bolero::check!()
        .with_type::<String>()
        .for_each(|text| {
            let expected = text.trim().parse::<u16>().unwrap_or_default();
            assert_eq!(parse_port(text), expected);
        });
}
