pub struct PeArray {
    pub rows: usize,
    pub columns: usize,
    pub macs_per_pe_per_cycle: usize,
}

impl PeArray {
    pub fn total_pes(&self) -> usize {
        self.rows * self.columns
    }

    pub fn peak_macs_per_cycle(&self) -> usize {
        self.total_pes() * self.macs_per_pe_per_cycle
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn pe_array_calculates_total_pes() {
        let topology = PeArray {
            rows: 2,
            columns: 2,
            macs_per_pe_per_cycle: 1,
        };

        assert_eq!(topology.total_pes(), 4);
    }

    #[test]
    fn pe_array_calculates_peak_macs_per_cycle() {
        let topology = PeArray {
            rows: 2,
            columns: 2,
            macs_per_pe_per_cycle: 1,
        };

        assert_eq!(topology.peak_macs_per_cycle(), 4);
    }
}
