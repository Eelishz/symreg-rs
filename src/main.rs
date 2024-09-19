mod dataloader;
mod expr;
mod metrics;

use crate::dataloader::DataLoader;
use crate::expr::Expr;
use crate::metrics::mse;

fn main() {
    let mut data = DataLoader::new("data/IRIS.csv").unwrap();
    // Discard first row
    let _ = data.pop_row();

    let row = data.pop_row().unwrap();
    println!("{row:?}");

    let mut tree = Expr::new();

    let x_row: Vec<f64> = row[0..row.len() - 1]
        .iter()
        .map(|e| (*e).parse::<f64>().unwrap())
        .collect();

    let y_row: f64 = match *row.last().unwrap() {
        "Iris-setosa" => 1.0,
        _ => 0.0,
    };

    let root = tree.generate_tree(3, x_row.len());

    let result = tree.evaluate(root, &x_row);

    let loss = mse(vec![result], vec![y_row]);
    println!("loss: {loss}");
    println!("RPN: {} eval: {result}", tree.generate_rpn(root));
}
