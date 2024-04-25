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

    call_function(&contract).await?;

    loop {
        time::sleep(Duration::from_secs(10)).await; // Esperar 10 segundos
        call_function(&contract).await?; // Llamar a la función
    }
}

async fn call_function(contract: &Contract) -> anyhow::Result<()>{
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
    use near_workspaces::result::{self, ExecutionFailure, ExecutionResult, ExecutionSuccess};
    use tokio;
    use serde_json::json;
   

    const NFT_WASM_FILEPATH: &str = "./examples/counter.wasm";

    #[tokio::test]
    async fn test_contract() -> anyhow::Result<()> {

        let worker = near_workspaces::sandbox().await?;
        let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
        let contract = worker.dev_deploy(&wasm).await?;

        println!("Contract deployed at: {}", contract.id());

          let outcome = contract
            .call("get_num")
            .transact() // note: we use the contract's keys here to sign the transaction
            .await?;       
            println!("--------------\n{:?}", outcome);

        // let outcome : serde_json::Value= contract
        // .call("get_num")
        // .transact() // note: we use the contract's keys here to sign the transaction
        // .await?.json()?;       
        // println!("--------------\n{:?}", outcome);

        Ok(())
    }
}

     // let outcome = contract
        //     .call("get_num")
        //     .args_json(json!({}))
        //     .view() // note: we use the contract's keys here to sign the transaction
        //     .await?;       
        //     println!("--------------\n{:?}", outcome);

