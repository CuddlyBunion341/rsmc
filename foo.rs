use crate::AsTrace;
pub use log::SetLoggerError;
use tracing_core::dispatch;

#[derive(Debug)]
pub struct LogTracer {
    ignore_crates: Box<[String]>,
}

#[derive(Debug)]
pub struct Builder {
    ignore_crates: Vec<String>,
    filter: log::LevelFilter,
}

impl LogTracer {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn new() -> Self {
        Self {
            ignore_crates: Vec::new().into_boxed_slice(),
        }
    }

    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn init_with_filter(level: log::LevelFilter) -> Result<(), SetLoggerError> {
        Self::builder().with_max_level(level).init()
    }

    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn init() -> Result<(), SetLoggerError> {
        Self::builder().init()
    }
}

impl Default for LogTracer {
    fn default() -> Self {
        Self::new()
    }
}

impl log::Log for LogTracer {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        if metadata.level().as_trace() > tracing_core::LevelFilter::current() {
            return false;
        }

        if !self.ignore_crates.is_empty() {
            let target = metadata.target();
            for ignored in &self.ignore_crates[..] {
                if target.starts_with(ignored) {
                    return false;
                }
            }
        }

        dispatch::get_default(|dispatch| dispatch.enabled(&metadata.as_trace()))
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            crate::dispatch_record(record);
        }
    }

    fn flush(&self) {}
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_level(self, filter: impl Into<log::LevelFilter>) -> Self {
        let filter = filter.into();
        Self { filter, ..self }
    }

    pub fn ignore_crate(mut self, name: impl Into<String>) -> Self {
        self.ignore_crates.push(name.into());
        self
    }

    pub fn ignore_all<I>(self, crates: impl IntoIterator<Item = I>) -> Self
    where
        I: Into<String>,
    {
        crates.into_iter().fold(self, Self::ignore_crate)
    }

    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn init(self) -> Result<(), SetLoggerError> {
        let ignore_crates = self.ignore_crates.into_boxed_slice();
        let logger = Box::new(LogTracer { ignore_crates });
        log::set_boxed_logger(logger)?;
        log::set_max_level(self.filter);
        Ok(())
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            ignore_crates: Vec::new(),
            filter: log::LevelFilter::max(),
        }
    }
}
