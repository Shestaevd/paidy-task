use lazy_static::lazy_static;
use prometheus::{Gauge, GaugeVec, HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry};
use tokio::sync::Mutex;

pub struct AppMetrics {
    pub registry: Registry,
    pub http_request_duration: HistogramVec,
    pub db_request_duration: HistogramVec,
    pub cpu_load_gauge: GaugeVec,
    pub cpu_load_total_gauge: Gauge,
    pub ram_used_gauge: Gauge,
    pub ram_available_gauge: Gauge,
}

lazy_static! {
    pub static ref METRICS: Mutex<AppMetrics> = Mutex::new({
        let registry = Registry::new();

        let opts = Opts::new("http_request", "Http request");
        let requests = IntCounterVec::new(opts, &["method", "path", "response_code"])
            .expect("Failed to create total counter");
        registry
            .register(Box::new(requests.clone()))
            .expect("Failed to register http_request metric");

        let opts = HistogramOpts::new("http_request_duration", "Http request duration");
        let http_request_duration = HistogramVec::new(opts, &["method", "path"])
            .expect("Failed to create http_request_duration histogram");
        registry
            .register(Box::new(http_request_duration.clone()))
            .expect("Failed to http_request_duration histogram");

        let opts = HistogramOpts::new("db_request_duration", "Db request duration");
        let db_request_duration = HistogramVec::new(opts, &["operation"])
            .expect("Failed to create db_request_duration histogram");
        registry
            .register(Box::new(db_request_duration.clone()))
            .expect("Failed to db_request_duration metric");

        let opts = Opts::new("cpu_load", "Cpu load by cores");
        let cpu_load_gauge: GaugeVec =
            GaugeVec::new(opts, &["core"]).expect("Failed to create cpu_load gauge");
        registry
            .register(Box::new(cpu_load_gauge.clone()))
            .expect("Failed to register cpu_load_gauge metric");

        let cpu_load_total_gauge: Gauge = Gauge::new("cpu_load_total", "Cpu load total")
            .expect("Failed to create cpu_load_total gauge");
        registry
            .register(Box::new(cpu_load_total_gauge.clone()))
            .expect("Failed to register cpu_load_total_gauge metric");

        let ram_used_gauge: Gauge = Gauge::new("ram_used_gauge", "Ram used")
            .expect("Failed to create cpu_load_total gauge");
        registry
            .register(Box::new(ram_used_gauge.clone()))
            .expect("Failed to register cpu_load_total_gauge metric");

        let ram_available_gauge: Gauge = Gauge::new("ram_available_gauge", "Ram availabe")
            .expect("Failed to create ram_available_gauge gauge");
        registry
            .register(Box::new(ram_available_gauge.clone()))
            .expect("Failed to register ram_available_gauge metric");

        AppMetrics {
            registry,
            http_request_duration,
            db_request_duration,
            cpu_load_gauge,
            cpu_load_total_gauge,
            ram_used_gauge,
            ram_available_gauge,
        }
    });
}
