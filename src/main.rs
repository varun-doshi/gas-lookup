use chrono::prelude::*;
use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use ethers::{
    etherscan::{
        account::{Sort, TxListParams},
        Client,
    },
    types::{Chain, H160},
    utils::hex::ToHex,
};
use web3::api::Namespace;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Account {
    /// The address to calculate gas for
    address_or_name: String,

    /// Start date of ags calculation
    start_date: String,

    /// End date of ags calculation
    end_date: String,

    /// Mainnet chain to calculate gas on. Default=eth
    #[arg(value_enum)]
    #[arg(default_value_t = Chains::eth)]
    chain: Chains,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Chains {
    eth,
    pol,
    opt,
}

impl Chains {
    pub fn get_keys(&self) -> Vec<&str> {
        let mut keys = Vec::with_capacity(4);
        match self {
            Chains::eth => {
                keys = vec![
                    "https://eth-mainnet.g.alchemy.com/v2/7g_cCnr4aef00M2NeaCpHPvmREVArJT0",
                    "ER9VKT8AXAI2WTPSCRNANN69W67V7PRU59",
                    "mainnet",
                    "GWEI",
                ];
            }
            Chains::pol => {
                keys = vec![
                    "https://polygon-mainnet.g.alchemy.com/v2/4I9z653r1BmGa5uNjsvu1cTLQptS3PG8",
                    "YFV2FG443XMWB774JJ4PD56Q7UBWFQ6KE1",
                    "polygon",
                    "MATIC",
                ];
            }
            Chains::opt => {
                keys = vec![
                    "https://opt-mainnet.g.alchemy.com/v2/vvyoCFsZrTcZvNdpuUdvXSI_lpgwjR25",
                    "I9KU6GZQHZEUAFAK9KNUXBFX3KXZH7UD6R",
                    "arbitrum",
                    "GWEI",
                ];
            }
        }
        keys.to_owned()
    }
}

impl Account {
    pub fn validate_date(&self) {
        if NaiveDate::parse_from_str(&self.start_date, "%d/%m/%Y").is_err()
            || NaiveDate::parse_from_str(&self.end_date, "%d/%m/%Y").is_err()
        {
            println!("The date is not valid or in invalid format. Example date: 20/12/2023");
            std::process::exit(1)
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Account::parse();

    let rpc_client = args.chain.get_keys()[0];
    let network_key = args.chain.get_keys()[1];
    args.validate_date();
    let address = classify_address(args.address_or_name).await;

    println!(
        "Address: {:?}, Start Date: {:?}, End Date: {:?}, Chain: {:?}",
        address,
        args.start_date,
        args.end_date,
        args.chain.get_keys()[2]
    );

    println!("{}", "Calculating total gas spent:");
    let start_block = format_date(&args.start_date, rpc_client).await;
    let end_block = format_date(&args.end_date, rpc_client).await;

    let _ = fetch_gas(
        &address,
        start_block,
        end_block,
        &network_key,
        &args.chain.get_keys()[2],
        &args.chain.get_keys()[3],
    )
    .await;
}

async fn classify_address(address: String) -> String {
    if address.starts_with("0x") {
        verify_address(&address);
    } else if address.ends_with(".eth") {
        let address = ens_to_address(address).await;
        // println!("{}",address);
        return address;
    } else {
        println!("The  address is not a valid Ethereum address.");
        std::process::exit(1)
    }
    return address;
}

pub async fn ens_to_address(address: String) -> String {
    let rpc_client = Chains::eth.get_keys()[0];
    let client = web3::transports::http::Http::new(&rpc_client).unwrap();
    let ens = web3::contract::ens::Ens::new(client);

    let ens_result = ens.eth_address(&address).await;
    match ens_result {
        Ok(s) => return format!("{}{}", "0x", s.encode_hex::<String>()),
        Err(_) => {
            println!("The ENS name is not a valid.");
            std::process::exit(1)
        }
    }
}

// Verifies whether an ETH address is valid
fn verify_address(address: &str) {
    let mut is_valid = true;

    if address.len() != 42 {
        is_valid = false;
    }

    if !address[2..].chars().all(|a| a.is_ascii_hexdigit()) {
        is_valid = false;
    }

    if !is_valid {
        println!("The address is not a valid Ethereum address.");
        std::process::exit(1)
    }
}

/// format date from string to dd,mm,yyyy format
async fn format_date(date: &str, rpc_client: &str) -> u64 {
    let dates: Vec<&str> = date.split('/').collect();

    let day = dates[0].parse::<u32>().unwrap();
    let month: u32 = dates[1].parse::<u32>().unwrap();
    let year: i32 = dates[2].parse::<i32>().unwrap();

    get_blocks(day, month, year, rpc_client).await
}

//get block number from date
async fn get_blocks(day: u32, month: u32, year: i32, rpc_client: &str) -> u64 {
    let hour = 3600;
    let datetime = chrono::FixedOffset::east_opt(5 * hour)
        .unwrap()
        .with_ymd_and_hms(year, month, day, 0, 0, 0)
        .unwrap();

    let client = web3::transports::http::Http::new(&rpc_client).unwrap();

    let web3client = web3::api::Web3::new(client);

    let mut web3_dater = web3_dater::Web3Dater::new(web3client);
    let block: u64 = web3_dater::Web3Dater::get_block_by_date(&mut web3_dater, datetime, true)
        .await
        .unwrap()
        .number
        .unwrap()
        .as_u64();

    block
}

//calculate total gas spent from start date to end date
async fn fetch_gas(
    eth: &str,
    start_block: u64,
    end_block: u64,
    network_api: &str,
    chain: &str,
    currency: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let r_address: Result<H160, _> = std::str::FromStr::from_str(eth);
    let address = match r_address {
        Ok(addr) => addr,
        Err(error) => {
            eprintln!(
                "{}",
                "Invalid ETH address. Please provide valid address"
            );
            return Err(error.into());
        }
    };

    // let network_api: String=String::from("ER9VKT8AXAI2WTPSCRNANN69W67V7PRU59");
    let chain_id = <Chain as std::str::FromStr>::from_str(chain).unwrap();

    let client = Client::builder()
        .with_api_key(network_api)
        .chain(chain_id)
        .unwrap()
        .build()
        .unwrap();

    let params = TxListParams {
        start_block,
        end_block,
        page: 0,
        offset: 0,
        sort: Sort::Asc,
    };

    let txns = client
        .get_transactions(&address, Some(params))
        .await
        .unwrap();
    let mut cumulative_gas_used: f64 = 0.0;
    for txn in 0..txns.len() {
        let gas_price = txns[txn].gas_price.unwrap().as_u128() as f64;
        let gwei_gas_price: f64 = gas_price / 1_000_000_000.0;
        let gas_used = txns[0].gas_used.as_u64() as f64;
        // println!("{}{}","Showing txn:".blue(),txn);
        // println!("Txn Hash:{:?} ",txns[txn].hash.value().unwrap());
        // println!("{:?}",gwei_gas_price);
        // println!("{:?}",gas_used);

        let total_gas = gwei_gas_price * gas_used;
        // println!("Gas Spent:{:?}",total_gas);
        cumulative_gas_used += total_gas;
        // println!("{}{}","Updated value: ".yellow(),cumulative_gas_used);
    }

    println!(
        "{}{} {}",
        "Total Gas Spent:",
        cumulative_gas_used,
        currency
    );
    Ok(())
}
