use anyhow::{Result, anyhow};
use std::env;
use tokio::net;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    #[expect(clippy::indexing_slicing)]
    let query = args[1].as_str();

    #[expect(clippy::print_stdout)]
    {
        println!("{query:?}");
    }

    let addr = net::lookup_host(query)
        .await?
        .next()
        .ok_or(anyhow!("No existe el dispositivo"))?;
    #[expect(clippy::print_stdout)]
    {
        println!("socket address is {addr}");
    }

    Ok(())
}
