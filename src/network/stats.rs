pub mod eighty_six_countries;

use self::eighty_six_countries::{sample_random_country, Country};
use crate::simulator::randomness_engine::RandomnessEngine;

pub fn assign_random_countries(country: &mut [Country], rand: &mut RandomnessEngine, size: usize) {
    assert_eq!(country.len(), size);
    for c in country.iter_mut() {
        *c = sample_random_country(rand);
    }
}
