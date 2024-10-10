pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    

    use solana_client::{rpc_client::RpcClient};
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, pubkey::Pubkey};
    use bs58;
    use std::io::{self, BufRead};
    use dotenv::dotenv; 
    use std::env;

    #[test]
    fn keygen() {

        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); 
        println!(""); 
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());

    } 
    
    #[test]
    fn base58_to_wallet() {

        println!("Input your private key as base58:");
        let stdin = io::stdin();

        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        println!("Yuour wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);

    }

    #[test]
    fn wallet_to_base58() {

        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();

        let wallet = 
        stdin.lock().lines().next().unwrap().unwrap()
        .trim_start_matches('[').trim_end_matches(']').split(',') 
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);

    }

    #[test] 
    fn airdrop() {
        // Load environment variables from '.env'
        dotenv().ok();

        // Get RPC_URL from .env
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(rpc_url);

        // Claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

            Ok(s) => {

                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());

            },

            Err(e) => { println!("Oops, something went wrong: {}", e.to_string()); }
        }

    } 
    
    #[test] 
    fn transfer_sol() {}
}
