use std::collections::HashMap;

// Define the utility function
fn utility_function(outcome: &str) -> f64 {
    // This function can be customized to represent the individual's preferences
    match outcome {
        "good" => 10.0,
        "bad" => 0.0,
        _ => panic!("Invalid outcome"),
    }
}

// Calculate expected utility of a lottery
fn calculate_expected_utility(lottery: &HashMap<&str, f64>) -> f64 {
    let mut expected_utility = 0.0;
    
    for (outcome, probability) in lottery.iter() {
        let utility = utility_function(outcome);
        expected_utility += probability * utility;
    }
    
    expected_utility
}

fn main() {
    // Example lottery: {"good": 0.6, "bad": 0.4}
    let mut lottery = HashMap::new();
    lottery.insert("good", 0.6);
    lottery.insert("bad", 0.4);

    let expected_utility = calculate_expected_utility(&lottery);
    println!("Expected utility of the lottery: {}", expected_utility);
}
