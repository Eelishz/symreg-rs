use crate::expr::{BinaryOp, Expr, Node, UnaryOp};

#[derive(Debug, Clone)]
enum Value {
    Literal(f64),
    Ptr(usize),
}

#[derive(Debug, Clone)]
enum BuiltinFunction {
    Abs,
    Loge,
    Log2,
    Log10,
    Sin,
    Cos,
    Tan,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Program {
    ops: Vec<Op>,
}

impl Program {
    pub fn evaluate(&self, inputs: &[f64]) -> Option<f64> {
        let mut vm = VM {
            pc: 0,
            stack: Vec::new(),
            memory: Vec::new(),
        };

        vm.memory.extend_from_slice(inputs);

        loop {
            if vm.pc >= self.ops.len() {
                break;
            }
            let op = &self.ops[vm.pc];

            match op {
                Op::Push(v) => match v {
                    Value::Literal(x) => vm.stack.push(*x),
                    Value::Ptr(x) => vm.stack.push(vm.memory[*x]),
                },
                Op::Add => {
                    let b = vm.stack.pop()?;
                    let a = vm.stack.pop()?;
                    vm.stack.push(a + b);
                }
                Op::Sub => {
                    let b = vm.stack.pop()?;
                    let a = vm.stack.pop()?;
                    vm.stack.push(a - b);
                }
                Op::Mul => {
                    let b = vm.stack.pop()?;
                    let a = vm.stack.pop()?;
                    vm.stack.push(a * b);
                }
                Op::Div => {
                    let b = vm.stack.pop()?;
                    let a = vm.stack.pop()?;
                    vm.stack.push(a / b);
                }
                Op::Pow => {
                    let b = vm.stack.pop()?;
                    let a = vm.stack.pop()?;
                    vm.stack.push(a.powf(b));
                }
                Op::Neg => {
                    let a = vm.stack.pop()?;
                    vm.stack.push(-a);
                }
                Op::Call(f) => match f {
                    BuiltinFunction::Abs => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.abs());
                    }
                    BuiltinFunction::Loge => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.ln());
                    }
                    BuiltinFunction::Log2 => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.log2());
                    }
                    BuiltinFunction::Log10 => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.log10());
                    }
                    BuiltinFunction::Sin => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.sin());
                    }
                    BuiltinFunction::Cos => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.cos());
                    }
                    BuiltinFunction::Tan => {
                        let a = vm.stack.pop()?;
                        vm.stack.push(a.tan());
                    }
                },
            }

            vm.pc += 1;
        }

        assert_eq!(vm.stack.len(), 1);
        vm.stack.pop()
    }

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
    pc: usize,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler() {
        let n_vars = 10;
        for _ in 0..10_000 {
            let vars: Vec<f64> = (0..n_vars).map(|_| rand::random::<f64>()).collect();
            let mut expr = Expr::new(n_vars);
            expr.random_tree(10);
            let compiled_expr = compile_expr(&expr);

            let expr_eval = expr.evaluate(&vars);
            let vm_eval = compiled_expr
                .evaluate(&vars)
                .expect("program should be valid");

            if expr_eval.is_nan() && vm_eval.is_nan() {
                continue;
            }

            println!("--------------");
            println!("{}", expr.rpn());
            compiled_expr.pprint();

            assert_eq!(expr_eval, vm_eval);
        }
    }
}
