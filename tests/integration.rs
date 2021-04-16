use reqray::{display::LoggingCallTreeCollectorBuilder, CallTreeCollectorBuilder};
use reqray_test_output;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};
fn init_tracing() {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    let call_tree_collector = CallTreeCollectorBuilder::default()
        .max_call_depth(10)
        .build_with_collector(
            LoggingCallTreeCollectorBuilder::default()
                .left_margin(20)
                .build(),
        );

    tracing_subscriber::registry()
        .with(call_tree_collector)
        .with(filter_layer)
        .with(fmt_layer)
        .try_init()
        .map_err(|e| {
            eprintln!("Error initing tracing: {}", e);
        })
        .ok();
}

#[test]
fn request() {
    init_tracing();
    reqray_test_output::request();
}


#[tokio::test]
async fn delayed() {
    reqray_test_output::delayed();
}

