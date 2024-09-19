pub fn mae(y_pred: Vec<f64>, y_true: Vec<f64>) -> f64 {
    assert_eq!(y_pred.len(), y_true.len());

    y_pred
        .iter()
        .zip(y_true)
        .map(|(a, b)| (b - a).abs())
        .sum::<f64>()
        / y_pred.len() as f64
}

pub fn mse(y_pred: Vec<f64>, y_true: Vec<f64>) -> f64 {
    assert_eq!(y_pred.len(), y_true.len());

    y_pred
        .iter()
        .zip(y_true)
        .map(|(a, b)| (b - a).powi(2))
        .sum::<f64>()
        / y_pred.len() as f64
}
