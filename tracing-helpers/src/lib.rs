use anyhow::Context;
use opentelemetry::global;

use opentelemetry::sdk::trace::Tracer;

use opentelemetry::trace::TraceError;

use opentelemetry_otlp::WithExportConfig;
use std::collections::HashMap;

use tracing_subscriber::prelude::*;

fn init_tracer() -> Result<Tracer, TraceError> {
    dotenv_flow::dotenv_flow().ok();
    let key = std::env::var("HONEY_COMB_TEAM_KEY")
        .context("Failed to get team key")
        .expect("HoneyComb Team Key should be provided");
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint("https://api.honeycomb.io/v1/traces")
                .with_http_client(reqwest::Client::default())
                .with_headers(HashMap::from([("x-honeycomb-team".into(), key)]))
                .with_timeout(std::time::Duration::from_secs(2)),
        )
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
}

pub fn config_telemetry() {
    // Needed to forward ordinary log statements to our tracing subscriber.
    tracing_log::LogTracer::init().expect("Failed to initialize log tracer");
    let tracer = init_tracer().expect("Failed to initialize tracer");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub fn shutdown_global_tracer_provider() {
    global::shutdown_tracer_provider();
}
