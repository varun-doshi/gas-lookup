use chrono::prelude::*;
use colored::Colorize;
use ethers::{types::{Chain, H160}, etherscan::{Client, account::{TxListParams, Sort}}};
use clap::Parser;
use dotenv::dotenv;

#[derive(Parser)]
struct Account {
    /// The address to calculate gas for
    address: String,
    /// Start date of ags calculation
    start_date:String,
    /// End date of ags calculation
    end_date:String
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Account::parse();

    println!("Address: {:?}, Start Date: {:?}, End Date: {:?}", args.address,args.start_date, args.end_date);

    println!("{}","Calculating total gas spent:".blue());
    let start_block=foramt_date(&args.start_date).await;
    // println!("{}{}","Selected Start Block:".blue(),start_block);
    let end_block=foramt_date(&args.end_date).await;
    // println!("{}{}","Selected End Block:".blue(),end_block);

    
    let _ =fetch_gas(&args.address,start_block,end_block).await;


}

//format date from string to dd,mm,yyyy format
async fn foramt_date(date:&str)->u64{
    let dates:Vec<&str>=date.split("/").collect();
    
    let day=dates[0].parse::<u32>().unwrap();
    let month:u32=dates[1].parse::<u32>().unwrap();
    let year: i32=dates[2].parse::<i32>().unwrap();

    let block=get_blocks(day,month,year).await;
     block
}

//get block number from date
async fn get_blocks(day:u32,month:u32,year:i32)->u64{
    let rpc_api_key = std::env::var("RPC_API_KEY").expect("RPC api key must be set.");
    let hour = 3600;
    let datetime = chrono::FixedOffset::east_opt(5 * hour)
    .unwrap()
    .with_ymd_and_hms(year, month, day, 0, 0, 0)
    .unwrap();


    let web3_client_url=format!("{}{}","https://eth-mainnet.g.alchemy.com/v2/",&rpc_api_key);
    let client=web3::transports::http::Http::new(&web3_client_url).unwrap();
    let web3client=web3::api::Web3::new(client);

    let mut web3_dater=web3_dater::Web3Dater::new(web3client);
    let block: u64=web3_dater::Web3Dater::get_block_by_date(&mut web3_dater,datetime,true).await.unwrap().number.unwrap().as_u64();

     block
}

//calculate total gas sepnt from start date to end date
async fn fetch_gas(eth:&str,start_block:u64,end_block:u64)->Result<(), Box<dyn std::error::Error>>{
    let etherscan_api_key = std::env::var("ETHERSCAN_API_KEY").expect("RPC api key not present.");
    let r_address: Result<H160, _>=std::str::FromStr::from_str(eth);
    let address = match r_address {
        Ok(addr) => { addr },
        Err(error) => { 
            eprintln!("{}","Invalid ETH address. Please provide valid address".red());
            return Err(error.into()); }
    };

    let network_api: String=String::from(etherscan_api_key);
    let chain_id = <Chain as std::str::FromStr>::from_str("mainnet").unwrap();

    let client = Client::builder()
        .with_api_key(network_api)
        .chain(chain_id)
        .unwrap()
        .build()
        .unwrap();

    let params = TxListParams {
                start_block: start_block,
                end_block: end_block,
                page: 0,
                offset: 0,
                sort: Sort::Asc,
            };

    let txns = client
        .get_transactions(
            &address,
            Some(params),
        )
        .await
        .unwrap();
    let mut cumulative_gas_used:f64=0.0;
    for txn in 0..txns.len(){
        
        let gas_price=txns[txn].gas_price.unwrap().as_u128() as f64;
        let gwei_gas_price:f64=gas_price/1000000000.0;
        let gas_used=txns[0].gas_used.as_u64() as f64;
        // println!("{}{}","Showing txn:".blue(),txn);
        // println!("Txn Hash:{:?} ",txns[txn].hash.value().unwrap());
        // println!("{:?}",gwei_gas_price);
        // println!("{:?}",gas_used);
        
        let total_gas=gwei_gas_price*gas_used;
        // println!("Gas Spent:{:?}",total_gas);
        cumulative_gas_used+=total_gas;
        // println!("{}{}","Updated value: ".yellow(),cumulative_gas_used);
    }

    println!("{}{}{}","Total Gas Spent:".green(),cumulative_gas_used," GWEI");
    Ok(())
}