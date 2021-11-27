mod arg;
mod config;
mod flow;
mod task;

use arg::arg::Argument;
use config::config::Config;
use flow::flow::Flow;
use std::process;
use task::task::Task;

#[tokio::main]
async fn main() {
    let mut a = Argument {
        ..Default::default()
    };

    if let Err(err) = a.parse() {
        println!("failed to parse argument: {}", err);
        process::exit(-1);
    }

    let mut c = Config {
        config_file: a.config_file,
        listen_url: a.listen_url,
        ..Default::default()
    };

    if let Err(err) = c.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }

    println!("task running");

    let f = Flow {
        config: c.clone(),
        routine: Task::routine,
    };

    if let Err(err) = f.run().await {
        println!("failed to run flow: {}", err);
        process::exit(-3);
    }

    println!("task exiting");
}
