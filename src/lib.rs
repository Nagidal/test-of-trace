pub mod logger;
use tracing::{info, trace};

pub fn add(left: usize, right: usize) -> usize {
    trace!("{}", format!("I'm adding {left} and {right}"));
    let result = left + right;
    info!("The result is {}", result);
    result
}

pub fn multiply(left: usize, right: usize) -> usize {
    trace!("{}", format!("I'm multiplying {left} and {right}"));
    let result = left * right;
    info!("The result is {}", result);
    result
}
