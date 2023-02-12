use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn log_init() -> WorkerGuard {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));

    // file appender
    let file_appender = tracing_appender::rolling::hourly("logs", "app.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ),
    );
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_timer(local_time.clone())
        .with_writer(non_blocking_appender);
    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_timer(local_time)
        .pretty();
    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
