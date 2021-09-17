use rand::{distributions, rngs::StdRng, Rng, SeedableRng};
use smart_str::SmartStr;

#[cfg(target_pointer_width = "64")]
const MAX_INLINED_SIZE: usize = 24;
#[cfg(target_pointer_width = "32")]
const MAX_INLINED_SIZE: usize = 12;

#[test]
fn test_randomized_roundtrip() {
    // create an rng
    let seed: u64 = rand::thread_rng().gen();
    eprintln!("using seed: {}_u64", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    let runs = option_env!("RANDOMIZED_RUNS")
        .map(|v| v.parse().expect("provided non-integer value?"))
        .unwrap_or(20_000);
    println!("Running with RANDOMIZED_RUNS: {}", runs);

    // generate a list of word with each word being up to 60 characters long
    let words: Vec<String> = (0..runs)
        .map(|_| {
            let len = rng.gen_range(0..60);
            rng.clone()
                .sample_iter::<char, _>(&distributions::Standard)
                .take(len)
                .map(char::from)
                .collect()
        })
        .collect();

    for word in words {
        let smart = SmartStr::new(&word);

        // assert the word roundtrips
        assert_eq!(smart, word);

        // assert it's properly allocated
        if smart.len() < MAX_INLINED_SIZE {
            assert!(!smart.is_heap_allocated())
        } else if smart.len() == MAX_INLINED_SIZE && smart.as_bytes()[0] <= 127 {
            assert!(!smart.is_heap_allocated())
        } else {
            assert!(smart.is_heap_allocated())
        }
    }
}
