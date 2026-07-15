mod topology;
mod workload;

use topology::PeArray;
use workload::MatrixMultiply;

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



