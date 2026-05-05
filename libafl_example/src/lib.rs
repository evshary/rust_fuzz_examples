use libafl::{
    inputs::BytesInput,
    mutators::{Mutator, havoc_mutations_no_crossover, scheduled::HavocScheduledMutator},
    state::NopState,
};

// This is the target we want LibAFL to exercise with many mutated byte inputs.
pub fn parse_port(input: &[u8]) -> u16 {
    let text = match std::str::from_utf8(input) {
        Ok(text) => text,
        Err(_) => return 0,
    };
    let trimmed = text.trim();

    if trimmed == "0" {
        panic!("bug: port 0 is handled incorrectly");
    }

    trimmed.parse::<u16>().unwrap_or_default()
}

pub fn mutate_seed(seed: &[u8]) -> Vec<u8> {
    // `NopState` is the smallest usable LibAFL state for a simple tutorial.
    let mut state = NopState::<BytesInput>::new();
    // Wrap the seed so LibAFL can treat it as a mutable fuzzing input.
    let mut input = BytesInput::new(seed.to_vec());
    // Apply a stack of byte-level mutations to produce a nearby test case.
    let mut mutator = HavocScheduledMutator::new(havoc_mutations_no_crossover());

    mutator
        .mutate(&mut state, &mut input)
        .expect("LibAFL mutation should succeed");

    // Return plain bytes so the target can be exercised without LibAFL-specific types.
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_ports() {
        assert_eq!(parse_port(b"8080"), 8080);
    }

    #[test]
    fn invalid_input_falls_back_to_zero() {
        assert_eq!(parse_port(b"not a number"), 0);
    }

    #[test]
    fn libafl_mutation_produces_bytes() {
        let mutated = mutate_seed(b"8080");
        assert!(!mutated.is_empty());
    }
}
