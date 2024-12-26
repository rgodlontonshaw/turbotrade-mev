use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta};
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Solana RPC endpoint
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // Replace with the PumpFun program ID
    let pumpfun_program_id = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

    // Fetch recent transactions for the program
    match client.get_signatures_for_address(&pumpfun_program_id) {
        Ok(signatures) => {
            println!("Fetching recent transactions...");
            for signature_info in signatures {
                let tx_signature = Signature::from_str(&signature_info.signature).unwrap();
                match client.get_transaction(&tx_signature, UiTransactionEncoding::JsonParsed) {
                    Ok(transaction) => {
                        // Access `meta` from `transaction.transaction`
                        if let Some(meta) = transaction.transaction.meta {
                            if let Some(logs) = meta.log_messages.as_ref().map(|s| s.as_ref() as &[String]) {

                                for log in logs {
                                    println!("Log message: {:?}", log);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error fetching transaction details: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error fetching signatures: {:?}", e);
        }
    }

    Ok(())
}
