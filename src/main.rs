mod cost_model;
mod mapping;
mod result;
mod topology;
mod workload;

use cost_model::{IDEAL_COMPUTE_ASSUMPTIONS, predict};
use mapping::Mapping;
use topology::PeArray;
use workload::MatrixMultiply;

fn main() {
    let workload = MatrixMultiply { m: 8, n: 8, k: 8 };

    let topology = PeArray {
        rows: 2,
        columns: 2,
        macs_per_pe_per_cycle: 1,
    };

    let mapping = Mapping { active_pes: 4 };

    match predict(&workload, &topology, &mapping) {
        Ok(result) => {
            println!("FabricFlow v0 Prediction");
            println!("------------------------");
            println!("Total MAC operations: {}", result.total_macs);
            println!("Total available PEs: {}", result.total_pes);
            println!("Active PEs: {}", result.active_pes);
            println!(
                "Theoretical peak MACs/cycle: {}",
                result.peak_macs_per_cycle
            );
            println!("Effective MACs/cycle: {}", result.effective_macs_per_cycle);
            println!("Ideal compute cycles: {}", result.ideal_compute_cycles);
            println!(
                "Compute utilization: {:.1}%",
                result.compute_utilization * 100.0
            );
            println!();
            println!("Model assumptions:");

            for assumption in IDEAL_COMPUTE_ASSUMPTIONS {
                println!("- {assumption}");
            }

            println!();
            println!("This prediction is an ideal compute lower bound.");
        }

        Err(error) => {
            eprintln!("Prediction failed: {error:?}");
        }
    }
}
