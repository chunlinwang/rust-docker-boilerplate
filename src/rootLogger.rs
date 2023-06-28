use crate::vars;
use std::fs::OpenOptions;
use slog_json::Json;
use slog::{o, PushFnValue};
use slog::FnValue;
use slog::{Drain, Never, SendSyncRefUnwindSafeDrain, Logger};
use std::sync::Arc;
use chrono::{Local, SecondsFormat};

pub fn build_logger_root() -> Logger<Arc<dyn SendSyncRefUnwindSafeDrain<Ok=(), Err=Never>>> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(vars::log_path())
        .unwrap();

    let drain = Json::new(file) // stdout example: std::io::stdout()
        .add_key_value(o!(
        "@timestamp" => PushFnValue(move |_, ser| {
            ser.emit(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true))
        }),
        "loglevel" => FnValue(move |rinfo| {
            rinfo.level().as_str()
        }),
        "msg" => PushFnValue(move |record, ser| {
            ser.emit(record.msg())
        }),
        ))
        .build()
        .fuse();

    let drain = slog_async::Async::new(drain).build().fuse();

    // generic root-logger you can use for the whole application
    slog::Logger::root(
        drain,
        o!(
        "version" => env!("CARGO_PKG_VERSION"),
        "service" => vars::service_name(),
        "log_type" => "application",
        "application_type" => "service",
        "module" => FnValue(move |info| { info.module().to_string() })
        ),
    )
}