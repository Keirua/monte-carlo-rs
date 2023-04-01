extern crate rand;
use rand::Rng;

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

fn estimate_expectation(nb_toys: usize, nb_iterations: usize) -> f64 {
    assert!(nb_iterations > 0);
    let mut total = 0;
    for _ in 0..nb_iterations {
        total += buy_meals_until_all_toys(nb_toys, 5000);
    }
    total as f64 / nb_iterations as f64
}

fn main() {
    println!("{}", estimate_expectation(94, 5000));
}