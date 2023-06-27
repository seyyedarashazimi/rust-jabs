use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Pareto};

#[derive(Default)]
pub struct RandomnessEngine {
    seed: u64,
}

impl RandomnessEngine {
    pub fn sample_pareto_distribution(&self, scale: f64, shape: f64) -> f64 {
        Pareto::new(scale, shape)
            .unwrap()
            .sample(&mut StdRng::seed_from_u64(self.seed))
    }
}

// public class RandomnessEngine extends MersenneTwister {
// public RandomnessEngine(long seed) {
// super(seed);
// }
//
// public <E> List<E> sampleSubset(List<E> list, int n) {
// int length = list.size();
// if (length < n) return null;
// for (int i = length - 1; i >= length - n; --i) {
// Collections.swap(list, i , this.nextInt(i + 1));
// }
// return list.subList(length - n, length);
// }
//
// public <E> E sampleFromList(List<E> list) {
// return list.get(this.nextInt(list.size()));
// }
//
// public long sampleDistributionWithBins(double[] dist, long[] bins) {
// double rand = this.nextDouble();
// for (int i = 0; i < dist.length-1; i++) {
// if (rand < dist[i]) {
// double diff = rand / dist[i];
// return (bins[i] + (long)(diff * (bins[i+1]-bins[i])));
// } else {
// rand -= dist[i];
// }
// }
// return bins[bins.length-1];
// }
//
// public long sampleDistributionWithBins(List<Double> dist, long[] bins) {
// double rand = this.nextDouble();
// for (int i = 0; i < dist.size()-1; i++) {
// if (rand < dist.get(i)) {
// double diff = rand / dist.get(i);
// return (bins[i] + (long)(diff * (bins[i+1]-bins[i])));
// } else {
// rand -= dist.get(i);
// }
// }
// return bins[bins.length-1];
// }
//
// public int sampleFromDistribution(double[] dist) {
// double rand = this.nextDouble();
// for (int i = 0; i < dist.length-1; i++) {
// if (rand < dist[i]) {
// double diff = rand / dist[i];
// return i;
// } else {
// rand -= dist[i];
// }
// }
// return dist.length-1;
// }
//
// public int sampleInt(int max) {
// return this.nextInt(max);
// }
//
// public double sampleDouble(double max) {
// return this.nextDouble() * max;
// }
//
// public double sampleExponentialDistribution(double mean) {
// ExponentialDistribution expDist = new ExponentialDistribution(this, mean);
// return expDist.sample();
// }
//
// public double sampleLogNormalDistribution(double median, double stdDev) {
// LogNormalDistribution expDist = new LogNormalDistribution(this, FastMath.log(median), stdDev);
// return expDist.sample();
// }
//
// public double sampleParetoDistribution(double scale, double shape) {
// ParetoDistribution pareto = new ParetoDistribution(this, scale, shape);
// return pareto.sample();
// }
// }
