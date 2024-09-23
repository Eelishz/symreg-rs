use crate::expr::{BinaryOp, Expr, Node, UnaryOp};

pub fn mae(y_pred: &Vec<f64>, y_true: &Vec<f64>) -> f64 {
    assert_eq!(y_pred.len(), y_true.len());

    y_pred
        .iter()
        .zip(y_true)
        .map(|(a, b)| (b - a).abs())
        .sum::<f64>()
        / y_pred.len() as f64
}

#[inline(always)]
pub fn mse(y_pred: &[f64], y_true: &[f64]) -> f64 {
    assert_eq!(y_pred.len(), y_true.len());

    y_pred
        .iter()
        .zip(y_true)
        .map(|(a, b)| (b - a).powi(2))
        .sum::<f64>()
        / (y_pred.len() as f64)
}

#[inline(always)]
pub fn regularize(model: &Expr, alpha: f64) -> f64 {
    alpha
        * model
            .nodes
            .iter()
            .map(|n| match n {
                Node::Number(_) => 1.0,
                Node::Variable(_) => 2.0,
                Node::UnOp(op) => match op.op {
                    UnaryOp::Neg => 1.0,
                    UnaryOp::Abs => 2.0,
                    _ => 5.0,
                },
                Node::BinOp(op) => match op.op {
                    BinaryOp::Add => 1.0,
                    BinaryOp::Sub => 1.0,
                    BinaryOp::Mul => 2.0,
                    BinaryOp::Div => 2.0,
                    BinaryOp::Pow => 3.0,
                },
            })
            .sum::<f64>()
}
