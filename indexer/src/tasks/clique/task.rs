use web3::transports::Http;
use web3::types::{ Block, FilterBuilder };
use web3::Web3;
use crate::config::EVMIndexerConfig;
use crate::logger::factory::AppLogger;

// server here
pub fn init(config: EVMIndexerConfig, logger: &AppLogger) {

    logger.info("Clique task started");
}

