use std::thread;
use std::time::Duration;
use std::time::{Duration, SystemTime};


struct Miner {
    id: i8;
    nonce_interval = u64;
    name = String;
    hash_of_preceding_coin = String;
    coin_blob = String;
    id_of_miner = String;
    hash_of_current_coin = String;
    difficulty = i8;
    current_difficulty = i8;
    last_updated = f64;
}

impl    

fn initiate_miner(id: i8, nonce_interval: i64, id_of_miner: String) -> Miner {
    Miner {
        id,
        nonce_interval,
        name = String::from("CPEN 442 Coin2021"),
        hash_of_preceding_coin = getLastCoin();
        coin_blob = "";
        id_of_miner,
        hash_of_current_coin = "";
        difficulty = getDifficulty();
        current_difficulty = 0;
        last_updated = SystemTime::now();

    }
}

fn update_miner(){

}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}