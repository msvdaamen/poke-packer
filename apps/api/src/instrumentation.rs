use opentelemetry::global;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use opentelemetry_stdout::{LogExporter, MetricExporter, SpanExporter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Instrumentation {
    pub is_production: bool,
    pub tracer_provider: SdkTracerProvider,
    pub meter_provider: SdkMeterProvider,
    pub logger_provider: SdkLoggerProvider,
}

impl Instrumentation {
    pub fn new(is_production: bool) -> Self {
        Self {
            is_production,
            tracer_provider: init_tracer_provider(),
            meter_provider: init_meter_provider(),
            logger_provider: init_logger_provider(),
        }
    }

    pub fn start(&self) {
        global::set_tracer_provider(self.tracer_provider.clone());
        global::set_meter_provider(self.meter_provider.clone());
        if self.is_production {
            let otel_layer = OpenTelemetryTracingBridge::new(&self.logger_provider);
            tracing_subscriber::registry().with(otel_layer).init();
        } else {
            let format = tracing_subscriber::fmt::format().compact();
            let subscriber = tracing_subscriber::fmt()
                .event_format(format)
                // ... add configuration
                .finish();

            let _ = tracing::subscriber::set_global_default(subscriber)
                .map_err(|_err| eprintln!("Unable to set global default subscriber"));
        }
    }

    pub fn stop(&self) {
        // Flush and shutdown all providers before exit
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("Error shutting down tracer provider: {err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("Error shutting down meter provider: {err:?}");
        }
        if let Err(err) = self.logger_provider.shutdown() {
            eprintln!("Error shutting down logger provider: {err:?}");
        }
    }
}

// --- Provider initialization ---
fn init_tracer_provider() -> SdkTracerProvider {
    SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build()
}

fn init_meter_provider() -> SdkMeterProvider {
    SdkMeterProvider::builder()
        .with_periodic_exporter(MetricExporter::default())
        .build()
}

fn init_logger_provider() -> SdkLoggerProvider {
    SdkLoggerProvider::builder()
        .with_simple_exporter(LogExporter::default())
        .build()
}
