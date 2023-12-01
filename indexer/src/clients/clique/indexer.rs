use web3::transports::Http;
use web3::types::{ Block, FilterBuilder };
use web3::Web3;
use crate::config::EVMIndexerConfig;
use crate::logger::factory::AppLogger;

pub async fn init(config: EVMIndexerConfig, logger: &AppLogger) {
    logger.info("Clique indexer started");
    indexer(config).await;

    async fn indexer(config: EVMIndexerConfig) {
        let http = Http::new(&config.rpc_url).expect("Failed to create HTTP transport");
        let web3 = Web3::new(http);
        let contract_address = &config.master_registry_contract;

        // todo ../ nesting
        let contract_abi = include_bytes!("../../../assets/clique_master_registry_abi.json");

        // Set up the filter
        let filter = FilterBuilder::default()
            .address(vec![contract_address.parse().unwrap()])
            // Add more filter parameters if needed, e.g., topics, fromBlock, toBlock, etc.
            .build();

        // Request logs
        let logs = web3.eth().logs(filter.clone()).await.expect("Failed to get logs");

        for log in logs {
            println!("Log: {:?}", log);
        }
    }
}
