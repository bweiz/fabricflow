pub struct MatrixMultiply {
    pub m: usize,
    pub n: usize,
    pub k: usize,
}

impl MatrixMultiply {
    pub fn total_macs(&self) -> usize {
        self.m * self.n * self.k
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn matrix_multiply_calculates_total_macs() {
        let workload = MatrixMultiply { m: 8, n: 8, k: 8 };

        assert_eq!(workload.total_macs(), 512);
    }
}
