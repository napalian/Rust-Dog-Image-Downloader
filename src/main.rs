use reqwest;
use tokio::task;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let num_threads = 4;
    let mut tasks = vec![];

    for _ in 1..=num_threads {
        let task = task::spawn(async {
            for _i in 1..=25 {
                let body = reqwest::get("https://dog.ceo/api/breeds/image/random")
                    .await?
                    .json::<serde_json::Value>()
                    .await?;
            }

            Ok::<(), reqwest::Error>(())
        });
        
        tasks.push(task);
    }

    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
