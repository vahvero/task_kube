use clap::Parser;
use log::info;
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Number of tasks to create
    #[arg(short, long, default_value_t = 124)]
    count: usize,
    /// Which URL to target
    #[arg(short, long, default_value = "localhost:30000")]
    address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let address = args.address;
    let target = format!("http://{}/api/task", address);
    info!("Targeting {}", address);
    let client = reqwest::Client::new();
    let tasks = (1..args.count + 1).map(|id| create_new_task(id, &client, &target));
    for task in tasks {
        let _ = task.await;
    }
}

async fn create_new_task(id: usize, client: &Client, target: &str) {
    let resp = client.post(target).send().await;
    match resp {
        Ok(response) => println!("{} Status: {}", id, response.status()),
        Err(e) => println!("id={} Error: {}", id, e),
    }
}
