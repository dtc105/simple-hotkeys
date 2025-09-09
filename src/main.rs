mod parser;
mod runner;

use runner::Runner;

fn main() {
    let mut runner = Runner::new();
    runner.run();
}
