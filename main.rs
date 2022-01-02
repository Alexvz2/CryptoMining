use std::time::{Instant, Duration};
use serde::Deserialize;
use futures::executor::block_on;
use std::collections::HashMap;
use hex_literal::hex;
use sha2::{Sha256, Digest};
use base64::encode;


struct Miner {
    id: i8,
    name : String,
    hash_of_preceding_coin : String,
    coin_blob : String,
    id_of_miner : String,
    hash_of_current_coin : String,
    difficulty : i8,
    current_difficulty : i32,
    last_updated : Instant,
}

impl Miner {
    
    async fn new(id: i8, unhashed_id_of_miner: String) -> Miner {
        let id_of_miner = hash_Sha256(unhashed_id_of_miner).await;
        let hash_of_preceding_coin = get_hash_of_preceding_coin().await;
        let difficulty = get_difficulty().await;
        let hash_of_current_coin = "".to_owned();
        let last_updated = Instant::now();

        return Miner {
            id,
            name : String::from("CPEN 442 Coin2021"),
            hash_of_preceding_coin,
            coin_blob : "".to_owned(),
            id_of_miner,
            hash_of_current_coin,
            difficulty,
            current_difficulty : 0,
            last_updated,
        }
    }

    async fn claim_coin(&self) -> u16 {
        let url = "http://cpen442coin.ece.ubc.ca/claim_coin".to_owned();

        let mut map = HashMap::new();
        map.insert("coin_blob", encode(self.coin_blob.to_owned()));
        map.insert("id_of_miner",self.id_of_miner.to_owned());
        map.insert("hash_of_last_coin", self.hash_of_preceding_coin.to_owned());

        let client = reqwest::Client::new();
        let res = client.post(url).json(&map).send().await.unwrap();
    
        println!("response {}", &res.status().as_u16());
        return res.status().as_u16();
    }

    async fn getCoinHash(&self) -> String {
        let mut cleartext = String::from("");
        cleartext.push_str(&self.name);
        cleartext.push_str(&self.hash_of_preceding_coin);
        cleartext.push_str(&self.coin_blob);
        cleartext.push_str(&self.id_of_miner);
        return hash_Sha256(cleartext).await;
    }

    async fn isHashValid(&self, difficulty: i8) -> bool {
        let mut temp_difficulty = difficulty;
        if difficulty == -1 {
            temp_difficulty = self.difficulty;
        }
        let padding = "0".repeat(temp_difficulty.try_into().unwrap());
        return self.getCoinHash().await.starts_with(&padding);
    }

    async fn updateCoinValues(&mut self) -> bool {
        let last_coin = get_hash_of_preceding_coin().await;
        let difficulty = get_difficulty().await;
        self.last_updated = Instant::now();
        if self.hash_of_preceding_coin != last_coin || self.difficulty != difficulty {
            self.hash_of_preceding_coin = last_coin;
            self.difficulty = difficulty;
            self.current_difficulty = 0;
            return true;
        }
        return false;
    }


    async fn mine_block(&mut self) {
        let mut i = 0;

        while i != u32::max_value() {
            self.coin_blob = i.to_string();

            if self.last_updated.elapsed() >= Duration::from_secs(120) {
                if self.updateCoinValues().await {
                    println!("Coin has been updated {}",self.hash_of_preceding_coin);
                    return
                }
            }
            if self.isHashValid(-1).await{
                println!("Miner #{} found coin blob {} with hash {}",self.id, self.coin_blob, self.getCoinHash().await);
                println!("Output :{}", self.claim_coin().await);
            }
            else if self.isHashValid((self.current_difficulty + 1).try_into().unwrap()).await{
                self.current_difficulty += 1;
                println!("Miner #{} at difficulty {}/{}, hash: {} for hash:{}",self.id, self.current_difficulty, self.difficulty, self.getCoinHash().await, self.hash_of_preceding_coin);
            }
            i += 1;
        }
    }

    async fn start(&mut self) {
        loop {
            self.mine_block().await;
            self.updateCoinValues().await;
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
    let mut miner = Miner::new(0, "bush_did_911".to_owned()).await;
    miner.start().await;
}

