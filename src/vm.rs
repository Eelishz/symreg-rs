use crate::expr::{BinaryOp, Expr, Node, UnaryOp};

enum Value {
    Literal(f64),
    Ptr(usize),
}

enum BuiltinFunction {
    Abs,
    Loge,
    Log2,
    Log10,
    Sin,
    Cos,
    Tan,
}

enum Op {
    Push(Value),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
    Call(BuiltinFunction),
}

pub struct Program {
    ops: Vec<Op>,
}

impl Program {
    pub fn pprint(&self) {
        for op in &self.ops {
            match op {
                Op::Push(v) => match v {
                    Value::Literal(x) => println!("PUSH {x}"),
                    Value::Ptr(x) => println!("PUSH ${x}"),
                },
                Op::Add => println!("ADD"),
                Op::Sub => println!("SUB"),
                Op::Mul => println!("MUL"),
                Op::Div => println!("DIV"),
                Op::Pow => println!("POW"),
                Op::Neg => println!("NEG"),
                Op::Call(f) => match f {
                    BuiltinFunction::Abs => println!("CALL $abs"),
                    BuiltinFunction::Loge => println!("CALL $loge"),
                    BuiltinFunction::Log2 => println!("CALL $log2"),
                    BuiltinFunction::Log10 => println!("CALL $log10"),
                    BuiltinFunction::Sin => println!("CALL $sin"),
                    BuiltinFunction::Cos => println!("CALL $cos"),
                    BuiltinFunction::Tan => println!("CALL $tan"),
                },
            }
        }
    }
}

struct VM {
    stack: Vec<f64>,
    memory: Vec<f64>,
}

pub fn compile_expr(expr: &Expr) -> Program {
    let mut ops = Vec::new();
    flatten_expr(&mut ops, expr, expr.root);
    Program { ops }
}

fn flatten_expr(buf: &mut Vec<Op>, expr: &Expr, node: usize) {
    match &expr.nodes[node] {
        Node::Number(x) => buf.push(Op::Push(Value::Literal(*x))),
        Node::Variable(x) => buf.push(Op::Push(Value::Ptr(*x))),
        Node::BinOp(op) => match op.op {
            BinaryOp::Add => {
                flatten_expr(buf, expr, op.a);
                flatten_expr(buf, expr, op.b);
                buf.push(Op::Add);
            }
            BinaryOp::Sub => {
                flatten_expr(buf, expr, op.a);
                flatten_expr(buf, expr, op.b);
                buf.push(Op::Sub)
            }
            BinaryOp::Mul => {
                flatten_expr(buf, expr, op.a);
                flatten_expr(buf, expr, op.b);
                buf.push(Op::Mul)
            }
            BinaryOp::Div => {
                flatten_expr(buf, expr, op.a);
                flatten_expr(buf, expr, op.b);
                buf.push(Op::Div)
            }
            BinaryOp::Pow => {
                flatten_expr(buf, expr, op.a);
                flatten_expr(buf, expr, op.b);
                buf.push(Op::Pow)
            }
        },
        Node::UnOp(op) => match op.op {
            UnaryOp::Neg => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Neg);
            }
            UnaryOp::Abs => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Abs));
            }
            UnaryOp::Loge => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Loge));
            }
            UnaryOp::Log2 => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Log2));
            }
            UnaryOp::Log10 => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Log10));
            }
            UnaryOp::Sin => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Sin));
            }
            UnaryOp::Cos => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Cos));
            }
            UnaryOp::Tan => {
                flatten_expr(buf, expr, op.a);
                buf.push(Op::Call(BuiltinFunction::Tan));
            }
        },
    }
}
