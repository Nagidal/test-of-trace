use tracing::{info, trace};

pub fn add(left: usize, right: usize) -> usize {
    trace!("I'm adding");
    let result = left + right;
    info!("The result is {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
