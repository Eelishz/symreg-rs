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

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

    use super::*;
    use test::Bencher;

    #[bench]
    fn iris(b: &mut Bencher) {
        let data_loader = DataLoader::new("data/IRIS.csv").unwrap();
        let mut data = data_loader.vec2d();
        let _headers = data.pop_head();
        let data = categorize_cols(data);
        let (x, y) = data.split_right();
        let params = GeneticParameters {
            population_size: 100,
            cutoff: 0.1,
            mutation_rate: 0.1,
        };

        b.iter(|| {
            let (_loss, _tree) = genetic_optimizer(10, &x, &y, &params);
        });
    }
}
