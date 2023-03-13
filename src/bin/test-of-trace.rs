use test_of_trace;
use tracing::{instrument, trace};
use tracing_subscriber::{fmt, EnvFilter};

fn install_tracing() {
    fmt()
        .without_time()
        .with_line_number(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("trace"))
                .unwrap(),
        )
        .init();
}

#[instrument]
fn main() {
    install_tracing();
    trace!("Starting {}", env!("CARGO_PKG_NAME"));
    test_of_trace::add(5, 8);
    trace!("Finishing {}", env!("CARGO_PKG_NAME"));
}
