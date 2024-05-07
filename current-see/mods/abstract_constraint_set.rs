struct BoxConstraints {
    lower_bounds: Vec<f64>,
    upper_bounds: Vec<f64>,
}

impl BoxConstraints {
    // Constructor to create a new BoxConstraints object
    fn new(lower_bounds: Vec<f64>, upper_bounds: Vec<f64>) -> Self {
        // Check if the number of lower bounds matches the number of upper bounds
        assert_eq!(lower_bounds.len(), upper_bounds.len(), "Bounds mismatch");
        
        // Check if each lower bound is less than or equal to its corresponding upper bound
        for (l, u) in lower_bounds.iter().zip(upper_bounds.iter()) {
            assert!(l <= u, "Invalid bounds");
        }
        
        BoxConstraints { lower_bounds, upper_bounds }
    }
    
    // Function to check if a point satisfies the box constraints
    fn is_feasible(&self, point: &[f64]) -> bool {
        // Check if the point satisfies each individual bound
        point.iter().enumerate().all(|(i, &x)| self.lower_bounds[i] <= x && x <= self.upper_bounds[i])
    }
}

fn main() {
    // Define lower and upper bounds for each variable
    let lower_bounds = vec![0.0, -1.0, 2.0];
    let upper_bounds = vec![10.0, 5.0, 8.0];
    
    // Create a new BoxConstraints object
    let constraints = BoxConstraints::new(lower_bounds, upper_bounds);
    
    // Test if some points satisfy the constraints
    let points = vec![
        vec![1.0, 0.0, 7.0],
        vec![5.0, -1.5, 3.0],
        vec![12.0, 2.0, 6.0],
    ];
    
    for point in &points {
        if constraints.is_feasible(point) {
            println!("Point {:?} satisfies the constraints", point);
        } else {
            println!("Point {:?} does not satisfy the constraints", point);
        }
    }
}
