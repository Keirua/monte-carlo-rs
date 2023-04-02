use clap::Parser;
use rand::Rng;
use statrs::statistics::*;
use monte_carlo_rs::*;

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

    /// Number of iterations the simulation will run
    #[arg(long, default_value_t = false)]
    with_pb: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let function_to_simulate = || buy_meals_until_all_toys(args.nb_toys, 100_000);

    let observations = if args.with_pb {
        mt_simulate(args.iterations, function_to_simulate)
    } else {
        mt_simulate_no_progressbar(args.iterations, function_to_simulate)
    };

    let nb_bins = 30u32;
    let (_min, _max, histogram) = create_histogram(&observations, nb_bins);

    let mut data = Data::new(observations);

    let mut deciles = [0.; 10];
    for i in 1..=10 {
        let q = i as f64 / 10.0;
        let quantile = data.quantile(q);
        deciles[i - 1] = quantile;
    }

    println!("mean: {:?}", data.mean());
    println!("deciles: {:?}", deciles);
    println!("histogram: {:?}", histogram);
    draw_histogram(&histogram, nb_bins)
}
