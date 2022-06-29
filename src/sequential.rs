use clap::{Arg, arg, Parser};
use rand::prelude::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short)]
    n: usize,
    #[clap(short)]
    m: usize,
    #[clap(short)]
    k: f64,
    #[clap(long)]
    debug: bool,
    #[clap(long)]
    time: bool,

}

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("Creating matrices with size {}x{}, k = {};", args.m, args.n, args.k);
    }

    let mut rng = rand::thread_rng();

    let mut now = Instant::now();

    let mut a_matrix: Vec<f64> = vec![0.0; args.n * args.m];
    let mut b_matrix: Vec<f64> = vec![0.0; args.n * args.m];
    let mut c_matrix: Vec<f64> = vec![0.0; args.n * args.m];

    let allocation_time = now.elapsed();

    now = Instant::now();

    for i in 0..args.n * args.m {
        a_matrix[i] = rng.gen_range(-100.0..100.0);
        b_matrix[i] = rng.gen_range(-100.0..100.0);
    }

    let fill_time = now.elapsed();

    if args.debug {
        println!("A matrix:");
        print_matrix(&a_matrix, args.m, args.n);
        println!("B matrix:");
        print_matrix(&b_matrix, args.m, args.n);
    }

    now = Instant::now();

    for i in 0..args.m * args.n {
        c_matrix[i] = a_matrix[i] + args.k * b_matrix[i];
    }

    let function_time = now.elapsed();

    if args.debug {
        println!("Result C matrix:");
        print_matrix(&c_matrix, args.m, args.n);
    }


    if args.time {
        if args.debug {
            println!("alloc time: {} ns;\nfill time: {} ns;\nfunction time: {} ns;", allocation_time.as_nanos(), fill_time.as_nanos(), function_time.as_nanos());
        }
        else {
            println!("{}\t{}\t{}", allocation_time.as_nanos(), fill_time.as_nanos(), function_time.as_nanos());
        }
    }

}

fn print_matrix(matrix: &Vec<f64>, m: usize, n: usize){
    for i in 0..n * m {
        if i % n == 0 && i != 0{
            println!();
        }
        print!("{:>15.4} ", matrix[i]);
    }
    println!();
}