# LEARNING

## 2026-07-07

### What I did
- Read the Eyeriss paper (ISCA 2016), sections I–III
- Focused on understanding existing spatial accelerator designs before writing any code

### What I learned
- The paper frames spatial accelerators as hardware that spends extra compute to reduce data movement, because moving data (especially from DRAM) can cost more energy than the MACs themselves.
- Instead of a cache-heavy CPU/GPU, Eyeriss uses a global buffer plus an array of Processing Elements (PEs) that communicate through local buffers/FIFOs – a good mental model for an FPGA PE array.
- For CNNs, the big lever is where weights, activations, and partial sums live in the memory hierarchy; that choice dominates bandwidth and energy cost more than the total number of MACs.
- Row-stationary dataflow explicitly maximizes reuse of all three (weights, activations, psums) at once, instead of over-optimizing one and paying for extra movement of the others, which is why it wins on energy across many layers.
