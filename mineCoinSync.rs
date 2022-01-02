use std::time::{Duration, Instant};



struct Miner {
    id: i8;
    name : String;
    hash_of_preceding_coin : String;
    coin_blob : String;
    id_of_miner : String;
    hash_of_current_coin : String;
    difficulty : i8;
    current_difficulty : i8;
    last_updated : Instant;
}

impl Miner {
    
    fn new(id: i8, unhashed_id_of_miner: String) -> Person {
        let id_of_miner = String::from(input_id_of_miner);

        let hash_of_preceding_coin = hash_of_preceding_coin();
        let hash_of_current_coin = "".to_owned();
        let current_difficulty = "".to_owned();
        let last_updated = Instant::now();

        return Miner(
            id,
            name : String::from("CPEN 442 Coin2021"),
            hash_of_preceding_coin,
            coin_blob : 0,
            id_of_miner,
            hash_of_current_coin,
            difficulty,
            current_difficulty : 0,
            last_updated)
    }
}

fn hash_of_preceding_coin(){

}

fn 
fn update_miner(){

}

fn main() {
    
}