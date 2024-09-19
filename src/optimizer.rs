use crate::{dataloader::DataLoader, expr::Expr, metrics::mse, vec2d::Vec2d};

pub fn naive_montecarlo(iterations: usize, data_loader: DataLoader) -> (f64, Expr) {
    let data = data_loader.vec2d();

    let (rows, cols) = data.shape();

    let mut x: Vec2d<f64> = Vec2d::new(cols - 1);
    let mut y: Vec<f64> = Vec::new();

    //TODO: rework hard coding.

    for i in 1..rows - 1 {
        let row = Vec::from(data.get_row(i).unwrap());
        let x_row: Vec<f64> = row[0..cols - 1]
            .iter()
            .map(|e| e.parse::<f64>().unwrap())
            .collect();
        assert_eq!(x_row.len(), x.shape().1);
        x.push_slice(&x_row);

        let y_row = match row[cols - 1] {
            "Iris-setosa" => 0.0,
            "Iris-versicolor" => 1.0,
            "Iris-virginica" => 2.0,
            _ => unreachable!(),
        };

        y.push(y_row);
    }

    let mut best_loss = f64::INFINITY;
    let mut best_expr = Expr::new();

    'outer: for i in 0..iterations {
        let step = iterations / 10;
        if i % step == step - 1 {
            // TODO: composable info printers
            println!("{}: loss: {best_loss} expr: {}", i + 1, best_expr.rpn());
        }

        let mut preds = Vec::new();
        let mut trues = Vec::new();

        let mut expr = Expr::new();
        expr.random_tree(2, cols - 1);

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

        let loss = mse(&preds, &trues);

        if loss < best_loss {
            best_loss = loss;
            best_expr = expr;
        }
    }

    (best_loss, best_expr)
}
