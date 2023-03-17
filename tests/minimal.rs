use cucumber::{then, when, World};

use tracing::{instrument, Level};
use tracing_subscriber::fmt;

use test_of_trace;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {}

impl TestWorld {
    fn new() -> Self {
        test_of_trace::logger::clear();
        Self {}
    }
}

#[when(regex = r"^I call the add function with (?P<left>\d+) and (?P<right>\d+)$")]
fn call_add(_world: &mut TestWorld, left: usize, right: usize) {
    test_of_trace::add(left, right);
}

#[then(regex = r"^a trace about adding (?P<left>\d+) and (?P<right>\d+) is emitted$")]
fn adding_trace_is_emitted(_world: &mut TestWorld, left: usize, right: usize) {
    match test_of_trace::logger::level_contains(
        &Level::TRACE,
        &format!("adding {left} and {right}"),
    ) {
        Ok(val) => assert!(val),
        Err(_) => assert!(false),
    }
}

#[then(
    regex = r"the result (?P<result>\d+) is written in the (?P<level>trace|debug|info|warn|error) trace"
)]
fn trace_result(_world: &mut TestWorld, result: usize, level: Level) {
    match test_of_trace::logger::level_contains(&level, &format!("result is {result}")) {
        Ok(val) => assert!(val),
        Err(_) => assert!(false),
    }
}

#[when(regex = r"^I call the multiply function with (?P<left>\d+) and (?P<right>\d+)$")]
fn call_multiply(_world: &mut TestWorld, left: usize, right: usize) {
    test_of_trace::multiply(left, right);
}

#[then(regex = r"^a trace about multiplying (?P<left>\d+) and (?P<right>\d+) is emitted$")]
fn multiplying_trace_is_emitted(_world: &mut TestWorld, left: usize, right: usize) {
    match test_of_trace::logger::level_contains(
        &Level::TRACE,
        &format!("multiplying {left} and {right}"),
    ) {
        Ok(val) => assert!(val),
        Err(_) => assert!(false),
    }
}

fn install_tracing() {
    fmt()
        .without_time()
        .with_line_number(true)
        .with_max_level(Level::TRACE)
        .json()
        .with_writer(test_of_trace::logger::Logger::new)
        .init();
}

#[tokio::main]
#[instrument]
async fn main() {
    install_tracing();
    TestWorld::run("tests/features").await;
    //TestWorld::cucumber()
    //.max_concurrent_scenarios(1)
    //.run("tests/features")
    //.await;
}
