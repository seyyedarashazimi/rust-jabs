pub trait PoW {
    fn get_difficulty(&self) -> f64;
    fn get_weight(&self) -> f64;

    fn set_difficulty(&mut self, difficulty: f64);
    fn set_weight(&mut self, weight: f64);

    fn set_pow_fields(&mut self, difficulty: f64, weight: f64) {
        self.set_difficulty(difficulty);
        self.set_weight(weight);
    }
}
