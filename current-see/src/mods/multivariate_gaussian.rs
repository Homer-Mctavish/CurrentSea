use ndarray::{Array2, Axis, stack, Array};
use rand_distr::{Distribution, MultivariateNormal};

// Function to generate a centered multivariate Gaussian distribution
fn generate_multivariate_gaussian(mu: Array<f64, ndarray::Ix1>, sigma: Array2<f64>, n_samples: usize) -> Array2<f64> {
    let normal = MultivariateNormal::new(mu.into_raw_vec(), sigma.into_raw_vec()).unwrap();
    let mut rng = rand::thread_rng();
    let samples = normal.sample(&mut rng, n_samples);
    Array::from_shape_vec((n_samples, mu.len()), samples).unwrap()
}

fn main() {
    // Define the mean vector mu
    let mu = Array::from(vec![0.0, 0.0]);

    // Define the covariance matrix sigma
    let sigma = Array::from_shape_vec((2, 2), vec![1.0, 0.5, 0.5, 1.0]).unwrap();

    // Generate 100 samples from the centered multivariate Gaussian distribution
    let n_samples = 100;
    let samples = generate_multivariate_gaussian(mu, sigma, n_samples);

    // Print the generated samples
    println!("Generated samples:");
    for sample in samples.axis_iter(Axis(0)) {
        println!("{:?}", sample);
    }
}
