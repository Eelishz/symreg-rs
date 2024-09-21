mod dataloader;
mod expr;
mod metrics;
mod optimizer;
mod vec2d;

use optimizer::naive_montecarlo;

use crate::dataloader::DataLoader;

fn main() {
    let data = DataLoader::new("data/IRIS.csv").unwrap();
    let (_loss, _tree) = naive_montecarlo(10_000_000, data);
}
