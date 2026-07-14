
struct MatrixMultiply {
    m: usize, 
    n: usize,
    k: usize
}

struct PeArray {
    rows: usize,
    columns: usize,
    macs_per_pe_per_cycle: usize,
}

impl MatrixMultiply {
    fn total_macs(&self) -> usize {
        self.m * self.n * self.k
    }
}

impl PeArray {
    fn total_pes(&self) -> usize {
        self.rows * self.columns
    }

    fn peak_macs_per_cycle(&self) -> usize {
        self.total_pes() * self.macs_per_pe_per_cycle
    }
}

fn main() {
    let workload = MatrixMultiply {
        m: 8,
        n: 8,
        k: 8,
    };
    
    let topology = PeArray {
        rows: 2,
        columns: 2,
        macs_per_pe_per_cycle: 1,
    };

    println!(
        "Matrix Multiplication: M={}, N={}, K={}",
        workload.m,
        workload.n,
        workload.k
    );

    println!("Total MACs: {}", workload.total_macs());
    println!("Total PEs: {}", topology.total_pes());
    println!(
        "Peak MACs per cycle: {}",
        topology.peak_macs_per_cycle()
    );
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn matrix_multiply_calculates_total_macs() {
        let workload = MatrixMultiply {
            m: 8,
            n: 8,
            k: 8,
        };
        
        assert_eq!(workload.total_macs(), 512);
    }
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
