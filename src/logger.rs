use std::borrow::Cow;

use serde::Serialize;

pub(crate) struct Logger {
    store: Vec<Log>,
}

impl Logger {
    pub fn new() -> Self {
        Self { store: Vec::new() }
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
pub(crate) struct Log {
    level: LogLevel,
    content: Cow<'static, str>,
}

impl Log {
    fn new(level: LogLevel, content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            level,
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
