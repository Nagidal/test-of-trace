use cucumber::{then, when, World};

use tracing::{instrument, Level};
use tracing_subscriber::fmt;

use test_of_trace;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {}

impl TestWorld {
    #[instrument]
    fn new() -> Self {
        fmt()
            .without_time()
            .with_line_number(true)
            .with_max_level(Level::TRACE)
            .json()
            .with_writer(test_of_trace::logger::Logger::new)
            .init();
        Self {}
    }
    fn level_contains(&self, level: &Level, pattern: &str) -> Result<bool, std::io::Error> {
        let s = test_of_trace::logger::GLOBAL_STRING.lock().unwrap();
        Ok(s.lines()
            .filter(|l| l.starts_with(&("{\"level\":\"".to_owned() + level.as_str())))
            .any(|l| l.contains(pattern)))
    }
}

#[when("I call the add function")]
fn call_add(_world: &mut TestWorld) {
    test_of_trace::add(3, 4);
}

#[then("a trace is emitted")]
fn trace_is_emitted(world: &mut TestWorld) {
    match world.level_contains(&Level::INFO, "result is") {
        Ok(val) => assert!(val),
        Err(_) => assert!(false),
    }
}

#[tokio::main]
async fn main() {
    TestWorld::run("tests/features").await;
}
