use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    expr::Expr,
    metrics::{mse, regularize},
    vec2d::Vec2d,
    vm::{compile_expr, Program},
};

pub fn naive_montecarlo(iterations: usize, x: Vec2d<f64>, y: Vec<f64>) -> (f64, Expr) {
    let (rows, cols) = x.shape();

    let mut best_loss = f64::INFINITY;
    let mut best_expr = Expr::new(cols);

    'outer: for i in 0..iterations {
        let step = iterations / 10;
        if i % step == step - 1 {
            // TODO: composable info printers
            println!("{}: loss: {best_loss} expr: {}", i + 1, best_expr.rpn());
        }

        let mut preds = Vec::new();
        let mut trues = Vec::new();

        let mut expr = Expr::new(cols);
        expr.random_tree(10);

        for ii in 0..rows - 2 {
            let x_row = x.get_row(ii).unwrap();
            let y_row = y[ii];

            let result = expr.evaluate(&x_row);

            // discard nan results
            if result.is_nan() {
                continue 'outer;
            }

            preds.push(result);
            trues.push(y_row);
        }

        let loss = mse(&preds, &trues) + regularize(&expr, 0.005);

        if loss < best_loss {
            best_loss = loss;
            best_expr = expr;
        }
    }

    (best_loss, best_expr)
}

pub struct GeneticParameters {
    pub population_size: usize,
    pub cutoff: f64,
    pub mutation_rate: f64,
}

impl GeneticParameters {
    pub fn default() -> GeneticParameters {
        GeneticParameters {
            population_size: 1_000,
            cutoff: 0.1,
            mutation_rate: 0.01,
        }
    }
}

#[derive(Debug, Clone)]
struct Individual {
    expr: Expr,
    compiled_expr: Program,
    loss: f64,
}

pub fn genetic_optimizer(
    iterations: usize,
    x: &Vec2d<f64>,
    y: &Vec<f64>,
    params: &GeneticParameters,
) -> (f64, Expr) {
    let (rows, cols) = x.shape();
    let mut population = Vec::new();

    for _ in 0..params.population_size {
        let mut expr = Expr::new(cols);
        expr.random_tree(10);
        let compiled_expr = compile_expr(&expr);
        let loss = f64::INFINITY;
        population.push(Individual {
            expr,
            compiled_expr,
            loss,
        });
    }

    let n_selected = (population.len() as f64 * params.cutoff) as usize;
    assert_ne!(n_selected, 0);

    for generation in 1..=iterations {
        population.par_iter_mut().progress().for_each(|individual| {
            let mut preds = Vec::new();
            let mut trues = Vec::new();

            for i_row in 0..rows {
                let x_row = x.get_row(i_row).unwrap();
                let y_row = y[i_row];

                //let result = individual.expr.evaluate(x_row);
                let result = individual.compiled_expr.evaluate(x_row).unwrap();
                let result = if result.is_nan() {
                    f64::INFINITY
                } else {
                    result
                };
                preds.push(result);
                trues.push(y_row);
            }

            let loss = mse(&preds, &trues) + regularize(&individual.expr, 0.001);
            individual.loss = loss;
        });

        population.sort_by(|a, b| a.loss.total_cmp(&b.loss));
        println!(
            "Generation {generation}, best loss: {:0.4}, best expr: {}",
            population[0].loss,
            population[0].expr.rpn(),
        );

        if generation == iterations {
            break;
        }
        let mut new_population = Vec::new();

        let mut rng = rand::thread_rng();

        for _i in 0..params.population_size {
            let random_individual = population[rng.gen_range(0..n_selected)].clone();
            let new_individual = random_individual.expr.mutate(params.mutation_rate);
            let compiled_expr = compile_expr(&new_individual);
            new_population.push(Individual {
                expr: new_individual,
                compiled_expr,
                loss: f64::INFINITY,
            });
        }

        population = new_population;
    }

    (population[0].loss, population[0].expr.clone())
}
