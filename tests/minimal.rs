use cucumber::{then, when, World as _};
use test_of_trace;
use tracing::Level;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::format::{self, Format},
    layer::SubscriberExt,
    Layer,
};

#[derive(Debug, Default, cucumber::World)]
#[world(init = Self::new)]
pub struct World;

impl World {
    fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() {
    let _res = World::cucumber()
        .configure_and_init_tracing(
            format::DefaultFields::new(), // a fmt::FormatFields implementation
            Format::default(),            // a pre-configured Event formatter in fmt module
            // an FnOnce consuming a layer::Layered subscriber
            |fmt_layer| tracing_subscriber::registry().with(LevelFilter::TRACE.and_then(fmt_layer)),
        )
        .run_and_exit("tests/features")
        .await;
}

#[when(regex = r"^I call the add function with (?P<left>\d+) and (?P<right>\d+)$")]
fn call_add(_world: &mut World, left: usize, right: usize) {
    test_of_trace::add(left, right);
}

#[allow(unused_variables)]
#[then(regex = r"^a trace about adding (?P<left>\d+) and (?P<right>\d+) is emitted$")]
fn adding_trace_is_emitted(_world: &mut World, left: usize, right: usize) {
    // TODO
    assert!(true);
}

#[allow(unused_variables)]
#[then(
    regex = r"the result (?P<result>\d+) is written in the (?P<level>trace|debug|info|warn|error) trace"
)]
fn trace_result(_world: &mut World, result: usize, level: Level) {
    // TODO
    assert!(true);
}

#[when(regex = r"^I call the multiply function with (?P<left>\d+) and (?P<right>\d+)$")]
fn call_multiply(_world: &mut World, left: usize, right: usize) {
    test_of_trace::multiply(left, right);
}

#[allow(unused_variables)]
#[then(regex = r"^a trace about multiplying (?P<left>\d+) and (?P<right>\d+) is emitted$")]
fn multiplying_trace_is_emitted(_world: &mut World, left: usize, right: usize) {
    // TODO
    assert!(true);
}
