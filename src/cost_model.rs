use crate::mapping::Mapping;
use crate::result::PredictionResult;
use crate::topology::PeArray;
use crate::workload::MatrixMultiply;

pub const IDEAL_COMPUTE_ASSUMPTIONS: &[&str] = &[
    "Every active PE performs one MAC every cycle.",
    "Work is perfectly distributed across active PEs.",
    "Operands are always available when needed.",
    "Output writes introduce no stalls.",
    "Communication is free.",
    "Memory bandwidth is unlimited.",
    "There is no pipeline fill or drain overhead.",
    "There is no synchronization overhead.",
];

#[derive(Debug, PartialEq, Eq)]
pub enum PredictionError {
    NoActivePes,
    ActivePesExceedTopology { active_pes: usize, total_pes: usize },
}

pub fn predict(
    workload: &MatrixMultiply,
    topology: &PeArray,
    mapping: &Mapping,
) -> Result<PredictionResult, PredictionError> {
    let total_macs = workload.total_macs();
    let total_pes = topology.total_pes();
    let peak_macs_per_cycle = topology.peak_macs_per_cycle();
    let active_pes = mapping.active_pes;

    if active_pes == 0 {
        return Err(PredictionError::NoActivePes);
    }

    if active_pes > total_pes {
        return Err(PredictionError::ActivePesExceedTopology {
            active_pes,
            total_pes,
        });
    }
    let effective_macs_per_cycle = active_pes * topology.macs_per_pe_per_cycle;
    let ideal_compute_cycles = total_macs.div_ceil(effective_macs_per_cycle);
    let compute_utilization = active_pes as f64 / total_pes as f64;

    Ok(PredictionResult {
        total_macs,
        total_pes,
        active_pes,
        peak_macs_per_cycle,
        effective_macs_per_cycle,
        ideal_compute_cycles,
        compute_utilization,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_example_produces_expected_prediction() {
        let workload = MatrixMultiply { m: 8, n: 8, k: 8 };

        let topology = PeArray {
            rows: 2,
            columns: 2,
            macs_per_pe_per_cycle: 1,
        };

        let mapping = Mapping { active_pes: 4 };

        let result = predict(&workload, &topology, &mapping)
            .expect("reference example should produce a valid prediction");

        assert_eq!(result.total_macs, 512);
        assert_eq!(result.total_pes, 4);
        assert_eq!(result.active_pes, 4);
        assert_eq!(result.peak_macs_per_cycle, 4);
        assert_eq!(result.effective_macs_per_cycle, 4);
        assert_eq!(result.ideal_compute_cycles, 128);
        assert_eq!(result.compute_utilization, 1.0);
    }

    #[test]
    fn prediction_rejects_zero_active_pes() {
        let workload = MatrixMultiply { m: 8, n: 8, k: 8 };

        let topology = PeArray {
            rows: 2,
            columns: 2,
            macs_per_pe_per_cycle: 1,
        };

        let mapping = Mapping { active_pes: 0 };

        let error = predict(&workload, &topology, &mapping)
            .expect_err("zero active PEs should be rejected");

        assert_eq!(error, PredictionError::NoActivePes);
    }

    #[test]
    fn prediction_rejects_more_active_pes_than_exist() {
        let workload = MatrixMultiply { m: 8, n: 8, k: 8 };

        let topology = PeArray {
            rows: 2,
            columns: 2,
            macs_per_pe_per_cycle: 1,
        };

        let mapping = Mapping { active_pes: 8 };

        let error = predict(&workload, &topology, &mapping)
            .expect_err("mapping cannot use more PEs than the topology contains");

        assert_eq!(
            error,
            PredictionError::ActivePesExceedTopology {
                active_pes: 8,
                total_pes: 4,
            }
        );
    }
}
