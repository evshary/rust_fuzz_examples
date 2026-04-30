use libafl_example::{mutate_seed, parse_port};

fn main() {
    let seed = b"8080";
    println!("Seed input: {}", String::from_utf8_lossy(seed));

    for i in 0..5 {
        let mutated = mutate_seed(seed);
        println!("Mutated #{i}: {:?}", String::from_utf8_lossy(&mutated));
        let _ = parse_port(&mutated);
    }
}
