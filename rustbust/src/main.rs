use std::fs::read_to_string;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{

    let app = Command::new("RustBuster")
        .author("numonce")
        .version("1.0.0")
        .about("A simple dirb clone in Async Rust!")
        .arg(
            Arg::new("wordlist")
                    .takes_value(true)
                    .required(true)
                    .long("wordlist")
                    .short('w')
                    .help("Path to wordlist to be used"),
         )
        .arg(
            Arg::new("url")
                .takes_value(true)
                .required(true)
                .long("url")
                .short('u')
                .help("Url to be used")
                )
        .get_matches();
        
    let urlbuff = app.value_of("url").unwrap().to_owned();
    let buff = read_to_string(app.value_of("wordlist").unwrap()).unwrap();
    let wordlist = buff.split_whitespace();
    let tasks: Vec<_> = wordlist
        .map(|buff| {
        let url = urlbuff.clone();
        let cli = reqwest::Client::new();
        let word = buff.to_string();
        tokio::spawn(async move {
        request(url, word, cli).await.unwrap();
        })
        })
    .collect();
for task in tasks {
task.await?;
}
Ok(())
}

async fn request(url: String, word: String, client: reqwest::Client) -> Result<(),Box<dyn std::error::Error>>{
let url = format!("{}/{}",url,word);
let res = client.get(&url)
    .send()
    .await?;
    let status = res.status();
    if status != 404 {
    println!("Testing {} and the status is {}",url,status);
    }
    Ok(())
}
