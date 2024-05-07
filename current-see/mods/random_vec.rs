use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Distribution, Normal};

fn main() {
    // Define the size of the vector
    let size = 5;

    // Seed the random number generator for reproducibility
    let mut rng = StdRng::seed_from_u64(42);

    // Define the parameters of the normal distribution
    let mean = 0.0;
    let std_dev = 1.0;

    // Create a normal distribution
    let normal_dist = Normal::new(mean, std_dev).unwrap();

    // Generate a random vector of specified size with values from the normal distribution
    let random_vector: Vec<f64> = (0..size)
        .map(|_| {
            let sample = normal_dist.sample(&mut rng);
            // You can add additional operations here if needed
            sample
        })
        .collect();

    // Print the random vector
    println!("Random vector: {:?}", random_vector);
}
