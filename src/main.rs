use anyhow::Result;
use env_logger::Env;
use log::{error, info};
use once_cell::sync::OnceCell;
use tokio::select;
use tokio::task::JoinSet;

use crate::config::{Config, read_config};
use crate::server::start_http;

mod server;
mod ipset;
mod config;
mod endpoints;

pub(crate) static CONFIG: OnceCell<Config> = OnceCell::new();

async fn execute_tasks_until_cancelled(mut tasks: JoinSet<Result<String>>) -> Result<()> {
    loop {
        select! {
            _ = tokio::signal::ctrl_c() => {
                    info!("CTRL + C received. Shutting down all tasks.");
                    tasks.shutdown().await;
                    return Ok(())
            },
            res = tasks.join_next() => {
                let res = res.unwrap()?;
                match res {
                    Ok(val) => info!("{val}"),
                    Err(why) => {
                        error!("{why}");
                        return Err(why);
                    }
                }
                if tasks.is_empty() {
                    info!("all tasks finished");
                    return Ok(())
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    CONFIG.set(read_config()?).unwrap();
    let mut tasks = JoinSet::new();
    tasks.spawn(start_http());
    execute_tasks_until_cancelled(tasks).await
}

