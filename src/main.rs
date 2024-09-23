#![feature(test)]

extern crate test;

use optimizer::{genetic_optimizer, GeneticParameters};
use vec2d::Vec2d;

mod dataloader;
mod expr;
mod metrics;
mod optimizer;
mod vec2d;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn sinxovercosx(x: f64) -> f64 {
    x.sin() / x.cos()
}

fn main() {
    let n = 10_000;
    let x_vec: Vec<f64> = (0..n).map(|e| e as f64 / 100.0).collect();
    let y: Vec<f64> = x_vec.iter().map(|e| sinxovercosx(*e)).collect();
    let mut x = Vec2d::new(1);
    x.push_slice(&x_vec);

    let (_loss, _tree) = genetic_optimizer(100, &x, &y, &GeneticParameters::default());
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
