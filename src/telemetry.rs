use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
/// type of the returned subscriber, which is indeed quite complex.
pub fn get_subscriber<Sink>(
    name: String, // Name of the subscriber (e.g. "areum-backend")
    env_filter: String, // Which log level to use (e.g. info, debug, warn, error)
    sink: Sink, // Sink is the destination of the logs (e.g. stdout, file, etc.)
) -> impl Subscriber + Send + Sync
where
    // HRTB: Sink must implement MakeWriter for any lifetime 'a
    // 'static: Sink must live for the entire duration of the program
    // Send + Sync: Sink must be thread-safe
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        sink,
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer) 
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
} 