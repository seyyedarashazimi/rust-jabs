use rand::distributions::WeightedIndex;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Exp, Exp1, LogNormal, Pareto};

pub struct RandomnessEngine {
    _seed: u64,
    rng: StdRng,
}

impl RandomnessEngine {
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn sample_from_distribution(&mut self, dist: &[f64]) -> usize {
        let weights = WeightedIndex::new(dist).unwrap();
        weights.sample(&mut self.rng)
    }

    pub fn sample_from_distribution_with_bins(&mut self, dist: &[f64], bins: &[f64]) -> f64 {
        bins[self.sample_from_distribution(dist)]
    }

    pub fn sample_usize(&mut self, max: usize) -> usize {
        self.rng.gen_range(0..max)
    }

    pub fn sample_exponential_distribution_mean_1(&mut self) -> f64 {
        Exp1.sample(&mut self.rng)
    }

    pub fn sample_exponential_distribution(&mut self, mean: f64) -> f64 {
        let exponential = Exp::new(1.0 / mean).unwrap();
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
        assert_ne!(nodes.len(), 0, "Error: provide a non-empty nodes vector.");

        (0..size)
            .map(|_| *nodes.choose(&mut self.rng).unwrap())
            .collect()

        // nodes
        //     .choose_multiple(&mut self.rng, size)
        //     .cloned()
        //     .collect()
    }

    pub fn sample_nodes_bigger_than_size(&mut self, nodes: &Vec<usize>, size: usize) -> Vec<usize> {
        nodes
            .choose_multiple(&mut self.rng, size)
            .cloned()
            .collect()
    }
}
