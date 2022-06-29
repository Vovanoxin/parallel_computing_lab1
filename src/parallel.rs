use clap::{Arg, arg, Parser};
use rand::prelude::*;
use std::time::Instant;
use itertools::izip;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short)]
    n: usize,
    #[clap(short)]
    m: usize,
    #[clap(short)]
    k: f64,
    #[clap(short, long)]
    threads: usize,
    #[clap(long)]
    debug: bool,
    #[clap(long)]
    time: bool,

}

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("Creating matrices with size {}x{}, k = {}; executing with {} threads;", args.m, args.n, args.k, args.threads);
    }

    let mut now = Instant::now();

    let mut a_matrix: Vec<f64> = vec![0.0; args.n * args.m];
    let mut b_matrix: Vec<f64> = vec![0.0; args.n * args.m];
    let mut c_matrix: Vec<f64> = vec![0.0; args.n * args.m];

    let allocation_time = now.elapsed();

    now = Instant::now();

    crossbeam::scope(|s| {
        let chunk_size = a_matrix.len()/ args.threads;
        for slices in a_matrix.chunks_mut(chunk_size).zip(b_matrix.chunks_mut(chunk_size)) {
            s.spawn(|_|{
                let mut rng = rand::thread_rng();
                let (a_slice, b_slice) = slices;
                for a in a_slice {
                    *a = rng.gen_range(-100.0..100.0);
                }
                for b in b_slice {
                    *b = rng.gen_range(-100.0..100.0);
                }
            });
        }
    });

    let fill_time = now.elapsed();

    if args.debug {
        println!("A matrix:");
        print_matrix(&a_matrix, args.m, args.n);
        println!("B matrix:");
        print_matrix(&b_matrix, args.m, args.n);
    }

    now = Instant::now();

    crossbeam::scope(|s| {
        let chunk_size = a_matrix.len() / args.threads;
        for (a_slice, b_slice, c_slice) in izip!(a_matrix.chunks_mut(chunk_size), b_matrix.chunks_mut(chunk_size), c_matrix.chunks_mut(chunk_size)) {
            s.spawn(|_|{
                for i in 0..a_slice.len() {
                    c_slice[i] = a_slice[i] + b_slice[i] * args.k;
                }
            });
        }
    });

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