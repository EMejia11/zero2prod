use tracing::subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing::Subscriber;
use tracing_subscriber::fmt::MakeWriter;



// Using 'impl Subscriber' as return type to avoid having to spell out
// the actual type of the returned subscriber
// We must explicitly call out that the returned subscriber is 'Send'
// and 'Sync' to allow us to pass it to 'init_subscriber' later
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        sink
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}


// Register a subscriber as global default to process span data
// only called once
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}