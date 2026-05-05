use libafl_example::{mutate_seed, parse_port};

fn main() {
    let seed = b"8080";
    println!("Seed input: {}", String::from_utf8_lossy(seed));

    for i in 0..100 {
        // Start from one known input and let LibAFL generate nearby variations.
        let mutated = mutate_seed(seed);
        println!("Mutated #{i}: {:?}", String::from_utf8_lossy(&mutated));
        // Exercise the target with each mutated byte sequence.
        let _ = parse_port(&mutated);
    }
}
