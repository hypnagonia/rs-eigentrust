mod config;
mod logger;
mod clients;
mod tasks;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let logger: logger::factory::AppLogger = logger::factory::AppLogger::new();
    logger.init_global_default();

    logger.info("Application started");

    let indexer_config = config.evm_indexer_config.clone();
    let indexer =  clients::clique::indexer::init(
        indexer_config, 
        &logger
    ).await;
    
    let task_config = config.evm_indexer_config.clone();
    tasks::clique::task::init(
        task_config, 
        &logger
    );
    
    println!("Database URL: {}", config.evm_indexer_config.rpc_url);
}
