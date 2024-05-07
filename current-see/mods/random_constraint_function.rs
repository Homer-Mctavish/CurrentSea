use rand::prelude::*;

// Define a struct to represent the random constraint function
struct RandomConstraintFunction {
    coefficients: Vec<f64>,
}

impl RandomConstraintFunction {
    // Constructor to create a new RandomConstraintFunction with random coefficients
    fn new(d: usize) -> Self {
        let mut rng = rand::thread_rng();
        let coefficients: Vec<f64> = (0..=d).map(|_| rng.gen::<f64>()).collect();
        Self { coefficients }
    }

    // Evaluate the function at a given point t
    fn evaluate(&self, t: f64) -> f64 {
        let mut result = 0.0;
        for (exp, &coeff) in self.coefficients.iter().enumerate() {
            result += coeff * t.powi(exp as i32);
        }
        result
    }
}

fn main() {
    let d = 3; // The dimension of the space
    let t = 2.5; // The point at which to evaluate the function

    // Create a random constraint function with dimension d
    let constraint_function = RandomConstraintFunction::new(d);

    // Evaluate the function at point t
    let result = constraint_function.evaluate(t);

    println!("Value of the random constraint function at t = {}: {}", t, result);
}
