use tracing::{info};
use tracing_subscriber::{self, layer::SubscriberExt};

#[tracing::instrument]
fn blah() {
    for i in 0..10 {
        info!("{}", i);
    }
}

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_etw::EtwLayer::new(true)),
    )
    .expect("setup the subscriber");

    blah();
}
