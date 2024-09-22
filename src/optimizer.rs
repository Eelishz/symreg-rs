use crate::{
    dataloader::DataLoader,
    expr::Expr,
    metrics::{mse, regularize},
    vec2d::Vec2d,
};

pub fn naive_montecarlo(iterations: usize, x: Vec2d<f64>, y: Vec<f64>) -> (f64, Expr) {
    let mut best_loss = f64::INFINITY;
    let mut best_expr = Expr::new();

    let (rows, cols) = x.shape();

    'outer: for i in 0..iterations {
        let step = iterations / 10;
        if i % step == step - 1 {
            // TODO: composable info printers
            println!("{}: loss: {best_loss} expr: {}", i + 1, best_expr.rpn());
        }

        let mut preds = Vec::new();
        let mut trues = Vec::new();

        let mut expr = Expr::new();
        expr.random_tree(10, cols);

        for ii in 0..rows - 2 {
            let x_row = x.get_row(ii).unwrap();
            let y_row = y[ii];

            let result = expr.evaluate(&Vec::from(x_row));

            // discard nan results
            if result.is_nan() {
                continue 'outer;
            }

            preds.push(result);
            trues.push(y_row);
        }

        let loss = mse(&preds, &trues) + regularize(&expr, 0.005);

        if loss < best_loss {
            best_loss = loss;
            best_expr = expr;
        }
    }

    (best_loss, best_expr)
}
