#![feature(test)]

extern crate test;

use dataloader::DataLoader;
use optimizer::{genetic_optimizer, GeneticParameters};
use vec2d::{categorize_cols, Vec2d};
use vm::compile_expr;

mod dataloader;
mod expr;
mod metrics;
mod optimizer;
mod vec2d;
mod vm;

fn main() {
    let data_loader = DataLoader::new("data/IRIS.csv").unwrap();
    let mut data = data_loader.vec2d();
    let _headers = data.pop_head();
    let data = categorize_cols(data);
    let (x, y) = data.split_right();
    let params = GeneticParameters {
        population_size: 100000,
        cutoff: 0.1,
        mutation_rate: 0.1,
    };

    let (_loss, tree) = genetic_optimizer(10, &x, &y, &params);
    let program = compile_expr(&tree);
    program.pprint();
}

#[cfg(test)]
mod tests {
    use crate::{
        dataloader::DataLoader,
        optimizer::{genetic_optimizer, GeneticParameters},
        vec2d::categorize_cols,
    };

    use self::{expr::Expr, vm::Program};

    use super::*;
    use test::Bencher;

    #[test]
    fn iris() {
        let data_loader = DataLoader::new("data/IRIS.csv").unwrap();
        let mut data = data_loader.vec2d();
        let _headers = data.pop_head();
        let data = categorize_cols(data);
        let (x, y) = data.split_right();
        let params = GeneticParameters {
            population_size: 1000,
            cutoff: 0.1,
            mutation_rate: 0.1,
        };

        let (_loss, _tree) = genetic_optimizer(20, &x, &y, &params);
    }

    #[bench]
    fn exprs(b: &mut Bencher) {
        let exprs: Vec<Expr> = (0..10_000)
            .map(|_| {
                let mut expr = Expr::new(0);
                expr.random_tree(10);
                expr
            })
            .collect();

        b.iter(|| {
            exprs.iter().for_each(|e| {
                e.evaluate(&[]);
            });
        });
    }

    #[bench]
    fn compiled_exprs(b: &mut Bencher) {
        let exprs: Vec<Program> = (0..10_000)
            .map(|_| {
                let mut expr = Expr::new(0);
                expr.random_tree(10);
                compile_expr(&expr)
            })
            .collect();

        b.iter(|| {
            exprs.iter().for_each(|e| {
                e.evaluate(&[]);
            });
        });
    }
}
