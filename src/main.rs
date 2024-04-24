// macro allowing us to convert args into JSON bytes to be read by the contract.

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tokio;
    use serde_json::json;

    const NFT_WASM_FILEPATH: &str = "./examples/counter.wasm";

    #[tokio::test]
    async fn test_nft_contract() -> anyhow::Result<()> {
        let worker = near_workspaces::sandbox().await?;
        let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
        let contract = worker.dev_deploy(&wasm).await?;

        println!("Contract deployed at: {}", contract.id());

        // let outcome = contract
        //     .call("get_num")
        //     .args_json(json!({}))
        //     .view() // note: we use the contract's keys here to sign the transaction
        //     .await?;       
        //     println!("--------------\n{:?}", outcome);


        let outcome = contract
        .call("get_num")
        .transact() // note: we use the contract's keys here to sign the transaction
        .await?;       
        println!("--------------\n{:?}", outcome);

        Ok(())
    }
}
