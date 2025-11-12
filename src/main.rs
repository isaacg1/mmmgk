use rand::prelude::*;
use std::array;
use noisy_float::prelude::*;

const STATES: usize = 3;
const SERVERS: usize = 3;

struct Results {
    means: [f64; SERVERS],
    gaps: [f64; SERVERS],
    hists: [Vec<u64>; SERVERS],
}

fn sim(
    lambdas: [f64; STATES],
    muss: [[f64; SERVERS]; STATES],
    alphass: [[f64; STATES]; STATES],
    num_jobs: u64,
    seed: u64,
) -> Results {
    assert!(STATES >= 1);
    assert!(SERVERS >= 1);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut total_queue = [0; SERVERS];
    let mut total_gap = [0.0; SERVERS];
    let mut num_arrivals = 0;
    let mut queues = [0; SERVERS];
    let mut state = 0;
    let mut counts = array::from_fn(|_| vec![]);
    while num_arrivals < num_jobs {
        let lambda = lambdas[state];
        let mus = muss[state];
        let alphas = alphass[state];
        let mu_sum: f64 = mus.iter().sum();
        let alpha_sum: f64 = alphas.iter().sum();
        let rate = lambda + mu_sum + alpha_sum;
        let sample = rng.random_range(0.0..rate);
        if sample < lambda {
            // arrival
            // data
            // let total_queue: u64 = queues.iter().sum();
            // total += total_queue;
            total_queue = total_queue
                .iter()
                .zip(&queues)
                .map(|(v, q)| v + q)
                .collect::<Vec<u64>>()
                .try_into()
                .expect("Correct length");
            let sum: u64 = queues.iter().sum();
            total_gap = total_gap
                .iter()
                .zip(&queues)
                .map(|(g, q)| g + (*q as f64 - sum as f64/SERVERS as f64).abs())
                .collect::<Vec<f64>>()
                .try_into()
                .expect("Correct length");
            for (i, q) in queues.iter().enumerate() {
                let q = *q as usize;
                while counts[i].len() <= q {
                    counts[i].push(0)
                }
                counts[i][q] += 1;
            }
            //let mean_queue = total_queue as f64 / SERVERS as f64;
            //let gap: f64 = queues.iter().map(|&q| (q as f64 - mean_queue).abs()).sum();
            //total_gap += gap;
            num_arrivals += 1;
            // JSQ
            let (min_index, _) = queues
                .iter()
                .enumerate()
                .min_by_key(|&(_i, q)| q)
                .expect("SERVERS >= 1");
            queues[min_index] += 1;
        } else if sample < lambda + mu_sum {
            // completion
            let mu_sample = sample - lambda;
            let mut running = 0.0;
            for i in 0..SERVERS {
                running += mus[i];
                if running >= mu_sample {
                    queues[i] = queues[i].saturating_sub(1);
                    break;
                }
            }
        } else {
            // state change
            let alpha_sample = sample - lambda - mu_sum;
            let mut running = 0.0;
            for i in 0..STATES {
                running += alphas[i];
                if running >= alpha_sample {
                    state = i;
                    break;
                }
            }
        }
    }
    let means = total_queue.map(|v| v as f64 / num_arrivals as f64);
    let gaps = total_gap.map(|f| f / num_arrivals as f64);
    Results {
        means,
        gaps,
        hists: counts,
    }
}

fn mean_by_load() {
    let num_jobs = 100_000_000;
    for seed in 0..10 {
        /*
        let lambda = 7.5;
        let lambdas = [lambda; STATES];
        let musss = [[[10.0, 1.0], [1.0, 10.0], [1.0, 1.0]], [[7.0, 4.0], [4.0, 7.0], [1.0, 1.0]]];
        */
        let lambdas_norm = [3.0, 6.0, 9.0];
        let musss = [[[0.5, 0.5, 1.0], [1.0, 2.5, 2.0], [5.0, 3.0, 2.5]]];
        let alpha = 0.1;
        let alphass = [[0.0, alpha, 0.0], [0.0, 0.0, alpha], [alpha, 0.0, 0.0]];
        println!(
            "lambdas_norm {lambdas_norm:?} musss {musss:?} alphass {alphass:?} num_jobs {num_jobs} seed {seed}"
        );
        for lambda_mult in [
            0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.83, 0.86, 0.9, 0.92, 0.94, 0.96, 0.97, 0.98, 0.99,
        ] {
            //println!("alpha; E[gap] high; E[gap] low");
            // Need variable mu_sum to really demonstrate
            //for alpha in [1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01, 0.005, 0.002, 0.001] {
            print!("{lambda_mult};");
            let lambdas = lambdas_norm.map(|ln| ln * lambda_mult);
            for muss in musss {
                let mean_vec = sim(lambdas, muss, alphass, num_jobs, seed).means;
                for v in mean_vec {
                    print!("{v};");
                }
            }
            println!();
        }
    }
}

fn dist_by_load() {
    let num_jobs = 100_000_000;
    let seed = 0;
    let lambdas_norm = [3.0, 6.0, 9.0];
    let musss = [[[0.5, 0.5, 1.0], [1.0, 2.5, 2.0], [5.0, 3.0, 2.5]]];
    let alpha = 0.1;
    let alphass = [[0.0, alpha, 0.0], [0.0, 0.0, alpha], [alpha, 0.0, 0.0]];
    println!(
        "lambdas_norm {lambdas_norm:?} musss {musss:?} alphass {alphass:?} num_jobs {num_jobs} seed {seed}"
    );
    for lambda_mult in [0.8, 0.9, 0.95, 0.98, 0.99] {
        println!("{lambda_mult};");
        let lambdas = lambdas_norm.map(|ln| ln * lambda_mult);
        for muss in musss {
            let hists = sim(lambdas, muss, alphass, num_jobs, seed).hists;
            for (i, hist) in hists.iter().enumerate() {
                print!("{i};");
                for entry in hist {
                    print!("{entry};");
                }
                println!();
            }
        }
    }
}

fn ssc_and_mean_by_alpha() {
    let num_jobs = 100_000_000;
    let seed = 0;
    let lambdas_norm = [3.0, 6.0, 9.0];
    let lambda_mult = 0.95;
    let lambdas = lambdas_norm.map(|ln| ln * lambda_mult);
    let musss = [[[0.5, 0.5, 1.0], [1.0, 2.5, 2.0], [5.0, 3.0, 2.5]], [[8.0, 1.0, 1.5], [0.5, 1.0, 0.5], [1.5, 1.5, 2.5]]];
    let mut alphas = vec![1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01, 0.005, 0.002, 0.001];
    alphas.sort_by_key(|f| n64(-f));
    println!(
        "lambdas_norm {lambdas_norm:?} lambda_mult {lambda_mult} musss {musss:?} alphas {alphas:?} num_jobs {num_jobs} seed {seed}"
    );
    for muss in musss {
        println!("muss {muss:?}");
        println!("alpha;mean;mean;mean;gap;gap;gap");
        for &alpha in &alphas {
            let alphass = [[0.0, alpha, 0.0], [0.0, 0.0, alpha], [alpha, 0.0, 0.0]];
            print!("{alpha};");
            let result = sim(lambdas, muss, alphass, num_jobs, seed);
            for mean in result.means {
                print!("{mean};");
            }
            for gap in result.gaps {
                print!("{gap};");
            }
            println!()
        }
    }
}

fn main() {
    let setting: u64 = std::env::args().nth(1).expect("arg present").parse().expect("arg num");
    match setting {
        0 => mean_by_load(),
        1 => dist_by_load(),
        2 => ssc_and_mean_by_alpha(),
        _ => unimplemented!(),
    }
}
