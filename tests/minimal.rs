use cucumber::{then, when, World};
use test_of_trace;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {}

impl TestWorld {
    fn new() -> Self {
        Self {}
    }
}

#[when("I call the add function")]
fn call_add(_world: &mut TestWorld) {
    test_of_trace::add(3, 4);
}

#[then("a trace is emitted")]
fn trace_is_emitted(_world: &mut TestWorld) {
    //TODO
    assert!(false);
}

#[tokio::main]
async fn main() {
    TestWorld::run("tests/features").await;
}
