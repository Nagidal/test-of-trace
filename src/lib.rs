use tracing::{info, trace};

pub fn add(left: usize, right: usize) -> usize {
    trace!("I'm adding");
    let result = left + right;
    info!("The result is {}", result);
    result
}
