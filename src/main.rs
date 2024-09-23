mod dataloader;
mod expr;
mod metrics;
mod optimizer;
mod vec2d;

use vec2d::categorize_cols;

use crate::{
    dataloader::DataLoader,
    optimizer::{genetic_optimizer, GeneticParameters},
};

fn main() {
    let data_loader = DataLoader::new("data/IRIS.csv").unwrap();
    let mut data = data_loader.vec2d();
    let _header = data.pop_head();
    let data = categorize_cols(data);
    let (x, y) = data.split_right();
    let params = GeneticParameters {
        population_size: 1_000_000,
        cutoff: 0.1,
        mutation_rate: 0.1,
    };
    let (_loss, _tree) = genetic_optimizer(100, x, y, &params);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2d::Vec2d;
    use optimizer::naive_montecarlo;

    #[test]
    fn iris() {
        let data_loader = DataLoader::new("data/IRIS.csv").unwrap();

        let data = data_loader.vec2d();

        let (rows, cols) = data.shape();

        let mut x: Vec2d<f64> = Vec2d::new(cols - 1);
        let mut y: Vec<f64> = Vec::new();

        for i in 1..rows - 1 {
            let row = Vec::from(data.get_row(i).unwrap());
            let x_row: Vec<f64> = row[0..cols - 1]
                .iter()
                .map(|e| e.parse::<f64>().unwrap())
                .collect();
            assert_eq!(x_row.len(), x.shape().1);
            x.push_slice(&x_row);

            let y_row = match row[cols - 1] {
                "Iris-setosa" => 1.0,
                _ => 0.0,
            };

            y.push(y_row);
        }

        let (loss, _tree) = naive_montecarlo(100_000, x, y);

        assert!(loss < 0.06);
    }
}
