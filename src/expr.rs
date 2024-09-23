use rand::{self, rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,

    // Built-in functioins
    Abs,
    Loge,
    Log2,
    Log10,
    Sin,
    Cos,
    Tan,
}

#[derive(Debug, Clone)]
pub struct BinOp {
    pub op: BinaryOp,

    // pointres (via Vec index)
    // to child nodes
    pub a: usize,
    pub b: usize,
}

#[derive(Debug, Clone)]
pub struct UnOp {
    pub op: UnaryOp,
    // pointer (via Vec index)
    // to child node.
    pub a: usize,
}

#[derive(Debug, Clone)]
pub enum Node {
    Number(f64),

    //Index to the variables Vec
    Variable(usize),

    UnOp(UnOp),
    BinOp(BinOp),
}

#[derive(Debug, Clone)]
pub struct Expr {
    // Using vec as an arena style
    // allocator and indecies as
    // easy to work with pointers.
    pub nodes: Vec<Node>,
    pub root: usize,
    pub n_inputs: usize,
}

impl Expr {
    pub fn new(n_inputs: usize) -> Expr {
        Expr {
            nodes: Vec::new(),
            root: 0,
            n_inputs,
        }
    }

    pub fn evaluate(&self, inputs: &[f64]) -> f64 {
        self.eval(self.root, inputs)
    }

    fn eval(&self, node: usize, inputs: &[f64]) -> f64 {
        match &self.nodes[node] {
            Node::Number(x) => *x,
            Node::Variable(ptr) => inputs[*ptr],
            Node::UnOp(op) => {
                let x = self.eval(op.a, inputs);

                match op.op {
                    UnaryOp::Neg => -x,
                    UnaryOp::Abs => x.abs(),
                    UnaryOp::Loge => x.ln(),
                    UnaryOp::Log2 => x.log2(),
                    UnaryOp::Log10 => x.log10(),
                    UnaryOp::Sin => x.sin(),
                    UnaryOp::Cos => x.cos(),
                    UnaryOp::Tan => x.tan(),
                }
            }
            Node::BinOp(op) => {
                let a = self.eval(op.a, inputs);
                let b = self.eval(op.b, inputs);

                match op.op {
                    BinaryOp::Add => a + b,
                    BinaryOp::Sub => a - b,
                    BinaryOp::Mul => a * b,
                    BinaryOp::Div => a / b,
                    BinaryOp::Pow => a.powf(b),
                }
            }
        }
    }

    pub fn random_tree(&mut self, max_depth: usize) {
        self.root = self.generate_tree(max_depth);
    }

    fn generate_tree(&mut self, max_depth: usize) -> usize {
        let node = if max_depth == 0 || rand::random::<f64>() < 0.15 {
            if rand::random::<bool>() && self.n_inputs != 0 {
                Node::Variable(rand::random::<usize>() % self.n_inputs)
            } else {
                let x = rand::random::<f64>() * 10.0 - 5.0;
                Node::Number(x)
            }
        } else {
            if rand::random::<bool>() {
                // generate binop

                let a = self.generate_tree(max_depth - 1);
                let b = self.generate_tree(max_depth - 1);

                Node::BinOp(BinOp {
                    op: random_binop(),
                    a,
                    b,
                })
            } else {
                // generate unop

                let child = self.generate_tree(max_depth - 1);

                Node::UnOp(UnOp {
                    op: random_unop(),
                    a: child,
                })
            }
        };

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn rpn(&self) -> String {
        self.generate_rpn(self.root)
    }

    fn generate_rpn(&self, node: usize) -> String {
        match &self.nodes[node] {
            Node::Number(x) => format!("{x:0.2}"),
            Node::Variable(ptr) => format!("${ptr}"),
            Node::UnOp(op) => {
                let x = self.generate_rpn(op.a);

                let op = match op.op {
                    UnaryOp::Neg => "neg".to_string(),
                    UnaryOp::Abs => "abs".to_string(),
                    UnaryOp::Loge => "ln".to_string(),
                    UnaryOp::Log2 => "log2".to_string(),
                    UnaryOp::Log10 => "log10".to_string(),
                    UnaryOp::Sin => "sin".to_string(),
                    UnaryOp::Cos => "cos".to_string(),
                    UnaryOp::Tan => "tan".to_string(),
                };

                format!("{x} {op}")
            }
            Node::BinOp(op) => {
                let lhs = self.generate_rpn(op.a);
                let rhs = self.generate_rpn(op.b);

                let op = match op.op {
                    BinaryOp::Add => "+".to_string(),
                    BinaryOp::Sub => "-".to_string(),
                    BinaryOp::Mul => "*".to_string(),
                    BinaryOp::Div => "/".to_string(),
                    BinaryOp::Pow => "^".to_string(),
                };

                format!("{lhs} {rhs} {op}")
            }
        }
    }

    pub fn mutate(&self, rate: f64) -> Expr {
        let mut rng = rand::thread_rng();
        let mut expr = self.clone();

        for i in 0..expr.nodes.len() {
            let node = &expr.nodes[i];
            if sometimes(&mut rng, rate) {
                let node_type = rng.gen_range(0..=3);
                match node_type {
                    0 => expr.nodes[i] = Node::Number(rng.gen::<f64>() * 10.0 - 5.0),
                    1 => expr.nodes[i] = Node::Variable(rng.gen_range(0..expr.n_inputs)),
                    2 => {
                        let mut new_nodes = Vec::new();
                        let offset = expr.nodes.len();
                        let a =
                            generate_subtree(&mut new_nodes, 3, expr.n_inputs, offset, &mut rng);
                        let b =
                            generate_subtree(&mut new_nodes, 3, expr.n_inputs, offset, &mut rng);
                        let op = random_binop();

                        expr.nodes.append(&mut new_nodes);

                        expr.nodes[i] = Node::BinOp(BinOp { op, a, b });
                    }
                    3 => {
                        let mut new_nodes = Vec::new();
                        let offset = expr.nodes.len();
                        let a =
                            generate_subtree(&mut new_nodes, 3, expr.n_inputs, offset, &mut rng);
                        let op = random_unop();

                        expr.nodes.append(&mut new_nodes);

                        expr.nodes[i] = Node::UnOp(UnOp { op, a });
                    }
                    _ => unreachable!(),
                }
            } else {
                match node {
                    Node::Number(x) => expr.nodes[i] = Node::Number(jiggle(*x, rate)),
                    _ => continue,
                }
            }
        }

        expr
    }
}

fn generate_subtree(
    nodes: &mut Vec<Node>,
    depth: usize,
    n_inputs: usize,
    index_offset: usize,
    rng: &mut ThreadRng,
) -> usize {
    let node = if depth == 0 || rng.gen::<f64>() < 0.5 {
        if rand::random::<bool>() && n_inputs != 0 {
            Node::Variable(rand::random::<usize>() % n_inputs)
        } else {
            let x = rand::random::<f64>() * 10.0 - 5.0;
            Node::Number(x)
        }
    } else {
        if rand::random::<bool>() {
            // generate binop

            let a = generate_subtree(nodes, depth - 1, n_inputs, index_offset, rng);
            let b = generate_subtree(nodes, depth - 1, n_inputs, index_offset, rng);

            Node::BinOp(BinOp {
                op: random_binop(),
                a,
                b,
            })
        } else {
            // generate unop

            let a = generate_subtree(nodes, depth - 1, n_inputs, index_offset, rng);

            Node::UnOp(UnOp {
                op: random_unop(),
                a,
            })
        }
    };

    nodes.push(node);
    nodes.len() - 1 + index_offset
}

fn sometimes(rng: &mut ThreadRng, rate: f64) -> bool {
    (*rng).gen::<f64>() < rate
}

fn jiggle(x: f64, rate: f64) -> f64 {
    x + rand::random::<f64>() * rate * 10.0
}

fn random_unop() -> UnaryOp {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=7) {
        0 => UnaryOp::Neg,
        1 => UnaryOp::Abs,
        2 => UnaryOp::Loge,
        3 => UnaryOp::Log2,
        4 => UnaryOp::Log10,
        5 => UnaryOp::Sin,
        6 => UnaryOp::Cos,
        7 => UnaryOp::Tan,
        _ => unreachable!(),
    }
}

fn random_binop() -> BinaryOp {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=4) {
        0 => BinaryOp::Add,
        1 => BinaryOp::Sub,
        2 => BinaryOp::Mul,
        3 => BinaryOp::Div,
        4 => BinaryOp::Pow,
        _ => unreachable!(),
    }
}
