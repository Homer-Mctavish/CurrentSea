fn mean(data: &Vec<f64>) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

fn covariance(x: &Vec<f64>, y: &Vec<f64>, mean_x: f64, mean_y: f64) -> f64 {
    let n = x.len();
    let mut cov = 0.0;
    for i in 0..n {
        cov += (x[i] - mean_x) * (y[i] - mean_y);
    }
    cov / (n - 1) as f64
}

fn covariance_matrix(data: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = data.len(); // number of observations
    let d = data[0].len(); // number of variables (dimensions)
    
    // Calculate the mean of each variable (column)
    let mut means = vec![0.0; d];
    for j in 0..d {
        let col: Vec<f64> = data.iter().map(|row| row[j]).collect();
        means[j] = mean(&col);
    }

    // Initialize covariance matrix
    let mut cov_matrix = vec![vec![0.0; d]; d];

    // Calculate covariance between each pair of variables
    for i in 0..d {
        for j in 0..d {
            let col_i: Vec<f64> = data.iter().map(|row| row[i]).collect();
            let col_j: Vec<f64> = data.iter().map(|row| row[j]).collect();
            cov_matrix[i][j] = covariance(&col_i, &col_j, means[i], means[j]);
        }
    }

    cov_matrix
}

fn main() {
    // Example dataset with 3 variables (columns) and 4 observations (rows)
    let data = vec![
        vec![2.0, 3.0, 1.0],
        vec![4.0, 5.0, 2.0],
        vec![6.0, 7.0, 3.0],
        vec![8.0, 9.0, 4.0],
    ];

    let cov_matrix = covariance_matrix(&data);
    
    // Print covariance matrix
    for row in cov_matrix {
        println!("{:?}", row);
    }
}
