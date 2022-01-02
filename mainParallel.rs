use std::time::{Instant, Duration, Sleep};
use serde::Deserialize;
use futures::executor::block_on;
use std::collections::HashMap;
use hex_literal::hex;
use sha2::{Sha256, Digest};
use base64::encode;


struct Block {
    address: String,
    difficulty : i8,
    last_updated: Instant,
    id_of_miner: String,
    hash_of_preceding_coin: String,

}

impl Block {

    async fn new(unhashed_id_of_miner: String) -> Miner {
        let id_of_miner = hash_Sha256(unhashed_id_of_miner).await;
        let hash_of_preceding_coin = get_hash_of_preceding_coin().await;
        let difficulty = get_difficulty().await;
        let last_updated = Instant::now();
        let address = "".to_owned();
        address.push_str(&self.name);
        address.push_str(&self.hash_of_preceding_coin);
        address.push_str(&self.coin_blob);
        address.push_str(&self.id_of_miner);

        return Miner {
            address,
            difficulty,
            last_updated,
            id_of_miner,
            hash_of_preceding_coin,
        }
    }

    async fn update_coin_values(&mut self) -> bool {
        let last_coin = get_hash_of_preceding_coin().await;
        let difficulty = get_difficulty().await;
        self.last_updated = Instant::now();

        if self.hash_of_preceding_coin[] != last_coin || self.difficulty != difficulty {
            self.hash_of_preceding_coin = last_coin;
            self.difficulty = difficulty;
            return true;
        }
        return false;
    }

    async fn check_for_updates(&self) -> bool {
        loop {
            if self.last_updated.elapsed() >= Duration::from_secs(120){
                if self.update_coin_values(){
                    println!("Coin has been updated {}",self.hash_of_preceding_coin);
                    return true;
                }
            }
        }
        return false;
    }
}

struct Miner {
    id: i8,
    coin_blob : String,
    current_difficulty : i32,
    current_hash: String,
    interval: u32;
}

impl Miner {
    
    async fn new(id: i8, interval: u32) -> Miner {

        return Miner {
            id,
            coin_blob : "".to_owned()
            current_difficulty : 0,
            interval,
        }
    }

    async fn claim_coin(&self) -> u16 {
        let url = "http://cpen442coin.ece.ubc.ca/claim_coin".to_owned();

        let mut map = HashMap::new();
        map.insert("coin_blob", encode(self.coin_blob.to_owned()));
        map.insert("id_of_miner",Block.id_of_miner.to_owned());

        let client = reqwest::Client::new();
        let res = client.post(url).json(&map).send().await.unwrap();
    
        println!("response {}", &res.status().as_u16());
        return res.status().as_u16();
    }

    async fn mine_block(&mut self) {
        let mut i = self.id * self.interval;
        let temp_difficulty = difficulty;
        let padding = "0".repeat(temp_difficulty.try_into().unwrap());

        while i != (self.id + 1) * self.interval {
            self.coin_blob = (i).to_string();

            let current_hash = hash_Sha256(reference to shared resource).await;

            if current_hash.starts_with(&padding){
                println!("Miner #{} found coin blob {} with hash {}",self.id, self.coin_blob, self.getCoinHash().await);
                println!("Output :{}", self.claim_coin().await);
            }
            let current_difficulty = self.current_difficulty;
            let current_padding = "0".repeat(temp_difficulty.try_into().unwrap());
            else if current_hash.starts_with(&current_padding){
                self.current_difficulty += 1;
                println!("Miner #{} at difficulty {}/{}, hash: {} for hash:{}",self.id, self.current_difficulty, self.difficulty, self.getCoinHash().await, self.hash_of_preceding_coin);
            }
            i += 1;
        }
    }
}

// HTTP Requests

#[derive(Deserialize)]
struct ResJson {
    coin_id: String,
}

#[derive(Deserialize)]
struct ResJson2 {
    number_of_leading_zeros: i8,
}

async fn get_hash_of_preceding_coin() -> String {
    let url = "http://cpen442coin.ece.ubc.ca/last_coin".to_owned();
    let client = reqwest::Client::new();
    let res = client.post(url).send().await.unwrap().json::<ResJson>().await.unwrap();
    return res.coin_id;
}

async fn get_difficulty() -> i8 {
    let url = "http://cpen442coin.ece.ubc.ca/difficulty".to_owned();
    let client = reqwest::Client::new();
    let res = client.post(url).send().await.unwrap().json::<ResJson2>().await.unwrap();
    return res.number_of_leading_zeros;
}

// Helper functions

async fn hash_Sha256(cleartext: String) ->  String {
    let mut hasher = Sha256::new();
    hasher.update(String::from(cleartext));
    let hash: String = format!("{:X}", hasher.finalize());
    return hash;
}


#[tokio::main]
async fn main() {
    let miners = 4;
    let intervals = u32::max_value() / miners;
    let mut block = Block::new("bush_did_911".to_owned()).await;
    let mut miner = Miner::new(0).await;
}

