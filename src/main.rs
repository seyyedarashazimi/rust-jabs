use rust_jabs::log::block_confirmation_logger::BlockConfirmationLogger;
use rust_jabs::log::blockchain_reorg_logger::BlockchainReorgLogger;
use rust_jabs::log::EventLogger;
use rust_jabs::scenario::bitcoin_global_network_scenario::BitcoinGlobalNetworkScenario;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    const AVERAGE_BLOCK_INTERVAL: f64 = 600.0;
    const CONFIRMATION_DEPTH: i32 = 6;
    const STOP_TIME: f64 = 86400.0;
    const SEED: u64 = 0;
    const NAME: &str = "One day in the life of Bitcoin";
    let logger_dir = Path::new("output");

    let mut bitcoin_scenario = BitcoinGlobalNetworkScenario::new(
        AVERAGE_BLOCK_INTERVAL,
        CONFIRMATION_DEPTH,
        NAME,
        SEED,
        STOP_TIME,
    );

    bitcoin_scenario.add_new_logger(Box::new(EventLogger::from_path(
        &logger_dir.join("bitcoin-confirmations-log.csv"),
        BlockConfirmationLogger,
    )?));
    bitcoin_scenario.add_new_logger(Box::new(EventLogger::from_path(
        &logger_dir.join("bitcoin-reorgs-log.csv"),
        BlockchainReorgLogger::new(),
    )?));

    bitcoin_scenario.run()?;

    Ok(())
}
