use prometheus::{
    Histogram, HistogramVec, IntCounter, IntCounterVec, IntGauge, IntGaugeVec, Registry,
};

pub struct DAMetrics {
    pub(crate) dispersed_bytes: IntCounter,
}

impl DAMetrics {
    pub fn new(metrics_registry: &Registry) -> Self {
        let dispersed_bytes = Box::new(
            IntCounter::new(
                "dispersed_bytes",
                "Number of bytes dispersed by the DA layer",
            )
            .unwrap(),
        );
        metrics_registry.register(dispersed_bytes.clone()).unwrap();
        Self {
            dispersed_bytes: *dispersed_bytes,
        }
    }
}

pub struct LibObject {
    metrics: DAMetrics,
}

impl LibObject {
    pub fn new(metrics: &Registry) -> Self {
        Self {
            metrics: DAMetrics::new(metrics),
        }
    }

    pub fn callfn(&self) {
        self.metrics.dispersed_bytes.inc();
    }

    pub fn get_call_count(&self) -> u64 {
        self.metrics.dispersed_bytes.get()
    }
}
