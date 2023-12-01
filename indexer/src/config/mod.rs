use dotenv::dotenv;
use std::env;

// todo belongs to a component?
#[derive(Clone)]
pub struct EVMIndexerConfig {
    pub rpc_url: String,
    pub master_registry_contract: String,
    pub from_block: String,
}

pub struct Config {
    pub evm_indexer_config: EVMIndexerConfig,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        
        let rpc_url = env::var("CLIQUE_EVM_INDEXER_RPC_URL").expect("CLIQUE_EVM_INDEXER_RPC_URL not found in .env");
        let from_block = env::var("CLIQUE_EVM_INDEXER_FROM_BLOCK").expect("CLIQUE_EVM_INDEXER_FROM_BLOCK not found in .env");
        let master_registry_contract = env::var("CLIQUE_EVM_INDEXER_MASTER_REGISTRY_ADDRESS").expect("CLIQUE_EVM_INDEXER_MASTER_REGISTRY_ADDRESS not found in .env");

        let evm_indexer_config = EVMIndexerConfig {
            rpc_url,
            from_block,
            master_registry_contract,
        };

        Config {
            evm_indexer_config,
        }
    }
}