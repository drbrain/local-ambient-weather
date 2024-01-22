use crate::{Encoder, Metric};

#[derive(Clone, Debug)]
pub struct Gauge {
    metric: Metric,
    value: f64,
    timestamp: Option<i64>,
}

impl Gauge {
    pub fn new(name: impl Into<String>, labels: impl Into<String>, value: f64) -> Self {
        let metric = Metric::new(name, labels);

        Self {
            metric,
            value,
            timestamp: None,
        }
    }

    pub fn new_with_timestamp(
        name: impl Into<String>,
        labels: impl Into<String>,
        value: f64,
        timestamp: i64,
    ) -> Self {
        let metric = Metric::new(name, labels);

        Self {
            metric,
            value,
            timestamp: Some(timestamp),
        }
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), std::fmt::Error> {
        encoder.encode_gauge(self)
    }

    pub fn metric(&self) -> &Metric {
        &self.metric
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
