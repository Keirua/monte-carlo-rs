#[macro_use]
extern crate statrs;
use rand::Rng;
use rayon::prelude::*;
use clap::Parser;
use statrs::statistics::*;
// use plotters::prelude::*;

fn buy_meals_until_all_toys(n: usize, iterlimit: usize) -> usize {
    let mut owned_toys = vec![false; n];

    for i in 0..iterlimit {
        owned_toys[rand::thread_rng().gen_range(0..n)] = true;
        if owned_toys.iter().all(|&x| x) {
            return i + 1;
        }
    }

    panic!("Expected simulation to finish in {} iterations.", iterlimit);
}

fn estimate_expectation(nb_toys: usize, nb_iterations: usize) -> Vec<f64> {
    assert!(nb_iterations > 0);
    let mut observations: Vec<_> = (0..nb_iterations)
        .into_par_iter()
        .map(|_| buy_meals_until_all_toys(nb_toys, 5000) as f64)
        .collect();
    observations
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of toys to find
    #[arg(short, long, default_value_t = 1)]
    nb_toys: usize,

    /// Number of iterations the simulation will run
    #[arg(short, long, default_value_t = 1)]
    iterations: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let observations = estimate_expectation(args.nb_toys, args.iterations);
    let mut data = Data::new(observations);

    let mut deciles = [0.; 10];
    for i in 1..=10 {
        let q = i as f64 / 10.0;
        let quantile = data.quantile(q);
        deciles[i - 1] = quantile;
    }


    println!("mean: {:?}", data.mean());
    println!("deciles: {:?}", deciles);
    Ok(())
}