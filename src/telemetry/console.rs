

use tracing_subscriber::Layer;
use tracing_subscriber::filter::FilterFn;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::*;
use tracing_appender::non_blocking::NonBlocking;

#[derive(Clone, Copy)]
struct TelemetryStringWriter(&'static str);
impl std::io::Write for TelemetryStringWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s: Box<str> = Box::from(
            std::str::from_utf8(buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        );
        super::log(self.0, s);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.write(buf).map(|_| ())
    }
}
impl<'a> MakeWriter<'a> for TelemetryStringWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        *self
    }
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum TracingSetupError {
    #[error("Failed to set global default tracing subscriber")]
    SetGlobalDefault(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error("Failed to create log file")]
    CreateLogFile(std::io::Error),
}

pub fn setup_tracing_subscriber() -> Result<(), TracingSetupError> {
    let datalog_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_line_number(true)
        .with_file(true)
        .with_target(true)
        .with_level(true)
        .json()
        .with_writer(TelemetryStringWriter("/console"))
        .with_filter(FilterFn::new(|metadata| {
            metadata.level() == &tracing::Level::WARN || metadata.level() == &tracing::Level::ERROR
        }));

    #[cfg(frc_real)]
    let (writer, _guard) = NonBlocking::new(
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("~/frc.log")
            .map_err(TracingSetupError::CreateLogFile)?
    );
    #[cfg(frc_sim)]
    let (writer, _guard) = NonBlocking::new(
        std::io::stdout()
    );

    let filelog_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_line_number(true)
        .with_file(true)
        .with_target(true)
        .with_level(true)
        .compact()
        .with_writer(writer);

    let subscriber = tracing_subscriber::Registry::default()
        .with(datalog_layer)
        .with(filelog_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}