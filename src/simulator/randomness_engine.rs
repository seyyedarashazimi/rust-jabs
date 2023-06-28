use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Exp, LogNormal, Pareto};

pub struct RandomnessEngine {
    _seed: u64,
    pub rng: StdRng,
}

impl RandomnessEngine {
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn sample_usize(&mut self, max: usize) -> usize {
        self.rng.gen_range(0..max)
    }

    pub fn sample_exponential_distribution(&mut self, mean: f64) -> f64 {
        let exponential = Exp::new(mean).unwrap();
        exponential.sample(&mut self.rng)
    }

    pub fn sample_log_normal_distribution(&mut self, median: f64, stddev: f64) -> f64 {
        let log_normal = LogNormal::new(median.ln(), stddev).unwrap();
        log_normal.sample(&mut self.rng)
    }

    pub fn sample_pareto_distribution(&mut self, scale: f64, shape: f64) -> f64 {
        let pareto = Pareto::new(scale, shape).unwrap();
        pareto.sample(&mut self.rng)
    }

    pub fn sample_nodes(&mut self, nodes: &Vec<usize>, size: usize) -> Vec<usize> {
        nodes
            .choose_multiple(&mut self.rng, size)
            .cloned()
            .collect()
    }
}
