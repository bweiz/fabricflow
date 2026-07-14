
struct MatrixMultiply {
    m: usize, 
    n: usize,
    k: usize
}

impl MatrixMultiply {
    fn total_macs(&self) -> usize {
        self.m * self.n * self.k
    }
}



fn main() {
    let workload = MatrixMultiply {
        m: 8,
        n: 8,
        k: 8,
    };

    println!(
        "Matrix Multiplication: M={}, N={}, K={}",
        workload.m,
        workload.n,
        workload.k
    );

    println!("Total MACs: {}", workload.total_macs());
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
}
