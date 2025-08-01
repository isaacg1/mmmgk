use rand::prelude::*;

const STATES: usize = 3;
const SERVERS: usize = 2;

fn sim(
    lambdas: [f64; STATES],
    muss: [[f64; SERVERS]; STATES],
    alphass: [[f64; STATES]; STATES],
    num_jobs: u64,
    seed: u64,
) -> f64 {
    assert!(STATES >= 1);
    assert!(SERVERS >= 1);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut total_gap = 0.0;
    let mut num_arrivals = 0;
    let mut queues = [0; SERVERS];
    let mut state = 0;
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
            let total_queue: u64 = queues.iter().sum();
            let mean_queue = total_queue as f64 / SERVERS as f64;
            let gap: f64 = queues.iter().map(|&q| (q as f64 - mean_queue).abs()).sum();
            total_gap += gap;
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
    total_gap / num_arrivals as f64
}

fn main() {
    let lambda = 7.5;
    let lambdas = [lambda; STATES];
    let num_jobs = 100_000_000;
    let seed = 0;
    let musss = [[[10.0, 1.0], [1.0, 10.0], [1.0, 1.0]], [[7.0, 4.0], [4.0, 7.0], [1.0, 1.0]]];
    println!(
        "lambdas {lambdas:?} musss {musss:?} num_jobs {num_jobs} seed {seed}"
    );
    println!("alpha; E[gap] high; E[gap] low");
    // Need variable mu_sum to really demonstrate
    for alpha in [1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01, 0.005, 0.002, 0.001] {
        print!("{alpha};");
        for muss in musss {
            let alphass = [[0.0, alpha, 0.0], [0.0, 0.0, alpha], [alpha, 0.0, 0.0]];
            let mean_gap = sim(lambdas, muss, alphass, num_jobs, seed);
            print!("{mean_gap};");
        }
        println!();
    }
}
