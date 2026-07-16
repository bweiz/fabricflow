#[derive(Debug)]
pub struct PredictionResult {
    pub total_macs: usize,
    pub total_pes: usize,
    pub active_pes: usize,
    pub peak_macs_per_cycle: usize,
    pub effective_macs_per_cycle: usize,
    pub ideal_compute_cycles: usize,
    pub compute_utilization: f64,
}
