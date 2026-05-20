mod parser;
mod runner;

use runner::Runner;

fn main() {
    let mut runner = Runner::new();
    env_logger::init();
    runner.run();
}
