mod cmd;
mod exec;
mod runner;
mod statetest;
use cmd::Error;
use structopt::StructOpt;
mod cli_env;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let cmd = cmd::MainCmd::from_args();
    cmd.run().await
}
