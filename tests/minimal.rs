use cucumber::{then, when, writer, World, WriterExt as _};
use std::{
    fs::{read_to_string, File},
    io,
    sync::Mutex,
};

use test_of_trace;
use tracing::{instrument, Level};
use tracing_subscriber::fmt;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {}

impl TestWorld {
    #[instrument]
    fn new() -> Self {
        let log_file = File::create("my_temp_trace.log").unwrap();
        fmt()
            .without_time()
            .with_line_number(true)
            .with_max_level(Level::TRACE)
            .json()
            .with_writer(Mutex::new(log_file))
            .init();
        Self {}
    }
    fn level_contains(&self, level: &Level, pattern: &str) -> Result<bool, std::io::Error> {
        let buffer = read_to_string("my_temp_trace.log")?;
        Ok(buffer
            .lines()
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
    //TODO
    match world.level_contains(&Level::INFO, "result is") {
        Ok(val) => assert!(val),
        Err(_) => assert!(false),
    }
}

#[tokio::main]
async fn main() {
    TestWorld::run("tests/features").await;
    // for debugging output:
    //TestWorld::cucumber()
    //.max_concurrent_scenarios(1)
    //.with_writer(
    //writer::Basic::raw(io::stdout(), writer::Coloring::Never, 0)
    //.summarized()
    //.assert_normalized(),
    //)
    //.run("tests/features")
    //.await;
}
