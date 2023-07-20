use super::{Downlink, Link, Uplink};
use crate::network::stats::eighty_six_countries::{
    sample_download_bandwidth, sample_upload_bandwidth, Country,
};
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

const BITS_PER_BYTE: u64 = 8;

pub fn remaining_time_to_load(link: &mut Link, simulator: &Simulator, size: u64) -> f64 {
    let loading_time = ((size * BITS_PER_BYTE) as f64) / link.bandwidth;
    let start_time = link.latest_loaded_time_done.max(simulator.simulation_time);
    let end_time = start_time + loading_time;
    link.latest_loaded_time_done = end_time;
    end_time - simulator.simulation_time
}

pub fn assign_all_bandwidths(
    uplink: &mut [Uplink],
    downlink: &mut [Downlink],
    country: &[Country],
    rand: &mut RandomnessEngine,
    size: usize,
) {
    assert_eq!(uplink.len(), size);
    assert_eq!(downlink.len(), size);
    for i in 0..size {
        uplink[i].link.bandwidth = sample_upload_bandwidth(country[i], rand);
        downlink[i].link.bandwidth = sample_download_bandwidth(country[i], rand);
    }
}
