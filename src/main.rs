use clap::Parser;

/// Calculate the square root with the Heron method.
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    /// Number to calculate the square root of
    sq: f64,
    /// Precision to be calculated to
    #[arg(short, long, default_value_t = 0.000001)]
    eps: f64,
    /// Starting point of calculation
    #[arg(short, long, default_value_t = 10.0)]
    start: f64,
    /// Starting point of calculation
    #[arg(short, long, default_value_t = 256)]
    max_iterations: i32,
}

fn main() {
    let args = Args::parse();

    let mut x0: f64;
    let mut x1: f64 = (args.start + args.sq / args.start) / 2.0;
    let mut n: i32 = 1;

    while (x1 * x1 - args.sq).abs() > args.eps && n < args.max_iterations {
        println!(
            "    n={iteration:0>10} :: current_guess={guess} >> {guess_squared} :: error={error}",
            iteration = n,
            guess = x1,
            guess_squared = x1 * x1,
            error = (x1 * x1 - args.sq).abs()
        );
        x0 = x1;
        x1 = (x0 + args.sq / x0) / 2.0;
        n += 1;
    }

    println!("\n\nResult: {}", x1);
    println!("Error: {}", (x1 * x1 - args.sq).abs());
    println!("Result squared: {}", x1*x1);
}
