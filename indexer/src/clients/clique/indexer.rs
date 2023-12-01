use web3::transports::Http;
use web3::types::{ Block, FilterBuilder };
use web3::Web3;
use crate::config::EVMIndexerConfig;
use crate::logger::factory::AppLogger;

pub fn init(config: EVMIndexerConfig, logger: &AppLogger) {

    logger.info("Clique indexer started");

    /*
    async fn indexer() {
        // Specify your Ethereum node URL
        let http = Http::new("http://localhost:8545").expect("Failed to create HTTP transport");
        let web3 = Web3::new(http);

        // Specify the contract address and ABI
        let contract_address = "0xYourContractAddress";
        let contract_abi = include_bytes!("path/to/your/contract_abi.json");

        // Set up the filter
        let filter = FilterBuilder::default()
            .address(vec![contract_address.parse().unwrap()])
            // Add more filter parameters if needed, e.g., topics, fromBlock, toBlock, etc.
            .build();

        // Request logs
        let logs = web3.eth().logs(filter.clone()).await.expect("Failed to get logs");

        // Process the logs
        for log in logs {
            println!("Log: {:?}", log);
        }
    }
     */
}

