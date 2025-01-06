use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Default)]
pub struct Logger {
    store: Vec<Log>,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info(&mut self, log: impl Into<Cow<'static, str>>) {
        self.store.push(Log::info(log));
    }

    pub fn error(&mut self, log: impl Into<Cow<'static, str>>) {
        self.store.push(Log::error(log));
    }

    pub fn warn(&mut self, log: impl Into<Cow<'static, str>>) {
        self.store.push(Log::warn(log));
    }

    pub fn take_all(&mut self) -> Vec<Log> {
        self.store.drain(..).collect()
    }
}

#[derive(Serialize)]
pub struct Log {
    level: LogLevel,
    time: u64,
    content: Cow<'static, str>,
}

impl Log {
    fn new(level: LogLevel, content: impl Into<Cow<'static, str>>) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            level,
            time,
            content: content.into(),
        }
    }

    fn info(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(LogLevel::Info, content)
    }

    fn error(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(LogLevel::Error, content)
    }

    fn warn(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(LogLevel::Warn, content)
    }
}

#[derive(Serialize)]
enum LogLevel {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "info")]
    Info,
}
