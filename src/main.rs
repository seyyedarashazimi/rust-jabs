use rust_jabs::scenario::bitcoin_global_network_scenario::run;

fn main() {
    const AVERAGE_BLOCK_INTERVAL: f64 = 600.0;
    const CONFIRMATION_DEPTH: i32 = 6;
    const STOP_TIME: f64 = 86400.0;
    const SEED: u64 = 0;

    run(AVERAGE_BLOCK_INTERVAL, CONFIRMATION_DEPTH, STOP_TIME, SEED);
}
