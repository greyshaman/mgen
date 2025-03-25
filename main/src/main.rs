use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
enum Generator {
    Fib { number: usize },

    Prime { number: usize },

    Rand { upper_bound: Option<usize> },
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    generator: Generator,
}

fn main() {
    let args = Args::parse();

    match args.generator {
        Generator::Fib { number } => {
            println!("Число Фибоначчи №{}: {}", number, fib::gen_fib(number));
        }
        Generator::Prime { number } => {
            println!("Простое число №{}: {}", number, prime::gen_prime(number));
        }
        Generator::Rand { upper_bound } => {
            let mut rng = rand::rng();
            if let Some(bound) = upper_bound {
                println!("Случайное число: {}", rng.random_range(0..=bound));
            } else {
                println!("Случайное число: {}", rng.random::<f64>());
            }
        }
    }
}
