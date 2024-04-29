// macro allowing us to convert args into JSON bytes to be read by the contract.
use near_workspaces::Contract;
use tokio::time;
use tokio::time::Duration;

const NFT_WASM_FILEPATH: &str = "./examples/counter.wasm";


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;
    println!("Contract deployed at (MAIN): {}", contract.id());
    
    call_function(&contract).await?;
    println!("First call_function");
    let mut count = 0;
    loop {
        time::sleep(Duration::from_secs(10)).await; // Esperar 10 segundos
        call_function(&contract).await?; // Llamar a la función
        count = count + 1;
        if count == 10 {
            break;
            
        }
    }
    Ok(())
}

async fn call_function(contract: &Contract) -> anyhow::Result<()>{
    println!("call_function");

    let outcome = contract.call("create_token")
    .args_json(serde_json::json!({
        "token_args": {
            "owner_id": 1,
            "total_supply": 1000000,
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Hassel Token",
                "symbol": "HassTK",
                "icon": "",
                "reference" : "",
                "reference_hash": "", 
                "decimals" : 10
            },
        }
    })).transact().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    //use near_workspaces::result::{self, ExecutionFailure, ExecutionResult, ExecutionSuccess};
    use tokio;
    use crate::*;
    //use serde_json::json;
   

    const NFT_WASM_FILEPATH: &str = "./examples/counter.wasm";
    const NFT_WASM_FILEPATH_FACTORY: &str = "./examples/token_factory.wasm";

    #[tokio::test]
    async fn test_contract() -> anyhow::Result<()> {

        let worker = near_workspaces::sandbox().await?;
        let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
        let contract = worker.dev_deploy(&wasm).await?;

        println!("Contract deployed at: {}", contract.id());

        let outcome = contract
        .call("get_num")
        .view() // note: we use the contract's keys here to sign the transaction
        .await?;       
        println!("--------------\n{:?}", outcome);

        Ok(())
    }

    #[tokio::test]
    async fn test_contract_factory() -> anyhow::Result<()>{
        let worker = near_workspaces::sandbox().await?;
        let wasm = std::fs::read(NFT_WASM_FILEPATH_FACTORY)?;
        let contract = worker.dev_deploy(&wasm).await?;
        println!("Contract 2 deployed at: {}", contract.id());
        Ok(())

    }

    #[tokio::test]

    async fn test_create_token() ->anyhow::Result<()> {

        let worker = near_workspaces::sandbox().await?;
        let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
        let contract = worker.dev_deploy(&wasm).await?;
        println!("Test contract deployed at: {}", contract.id());

        call_function(&contract).await?;
        
        let outcome = contract.call("create_token")
        .args_json(serde_json::json!({
        "token_args": {
            "owner_id": 1,
            "total_supply": 1000000,
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Hassel Token",
                "symbol": "HassTK",
                "icon": "",
                "reference" : "",
                "reference_hash": "", 
                "decimals" : 10
            },
        }
        })).transact().await?;
        println!("outcome es: {:?}",outcome.outcome());
        //assert!(outcome.outcome())
        Ok(())
    }
}

   