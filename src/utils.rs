use rand::Rng;

pub(crate) fn generate_socket_id() -> String {
    let mut rng = rand::thread_rng(); // Get a random number generator

    // Define min and max as u64, since Rust requires specifying the integer type
    let min: u64 = 0;
    let max: u64 = 10000000000;

    // Rust's rand crate handles generating a random number between min and max differently
    let mut random_number = |min: u64, max: u64| -> u64 { rng.gen_range(min..=max) };

    // Format the random numbers into a String with a dot separator
    format!("{}.{}", random_number(min, max), random_number(min, max))
}