use std::f64::consts::PI;

// Define the equation
fn equation(x: f64) -> f64 {
    // Example equation: f(x) = sin(x)
    x.sin()
}

// Define a function to evaluate the equation over a range of values
fn evaluate_equation(start: f64, end: f64, step: f64) {
    let mut x = start;
    while x <= end {
        let result = equation(x);
        println!("f({:.2}) = {:.2}", x, result);
        x += step;
    }
}

fn main() {
    // Define the range over which to evaluate the equation
    let start = 0.0;
    let end = 2.0 * PI; // For example, from 0 to 2*pi
    let step = 0.1;     // Step size

    // Evaluate the equation over the specified range
    evaluate_equation(start, end, step);
}
