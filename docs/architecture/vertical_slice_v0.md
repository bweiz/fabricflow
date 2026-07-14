# FabricFlow Vertical Slice v0

## Question
What is the smallest, simplest version of this project that will produce a prediction?
## Reference Example

### Workload

Matrix multiplication:

C[M, N] = A[M, K] × B[K, N]

For v0:

- M = 8
- N = 8
- K = 8

Total operations:

512 MAC operations.

### Hardware

A 2 × 2 processing-element array:

- 4 processing elements
- 1 MAC per PE per cycle
- 4 theoretical MACs per cycle

### Mapping

For v0, the mapping is explicitly provided rather than discovered.

The model assumes all four PEs can participate in the computation.

### Expected Ideal Result

512 MACs / 4 MACs per cycle = 128 ideal compute cycles.

## Vertical Slice

## 1. Workload

The workload describes what computation must be performed.

For v0, FabricFlow supports one workload type: matrix multiplication.

Required information:

- M dimension
- N dimension
- K dimension

Derived information:

- total MAC operations = M × N × K

## 2. Hardware Topology

The topology describes the compute resources available to execute the workload.

For v0, the hardware is modeled as a rectangular PE array.

Required information:

- number of PE rows
- number of PE columns
- MAC operations per PE per cycle

Derived information:

- total processing elements
- theoretical peak MAC throughput

## 3. Mapping

The mapping describes how the workload is assigned to the available hardware.

For v0, mapping is intentionally simplified.

The mapping specifies:

- number of active processing elements

The mapping does not yet describe:

- loop ordering
- tiling
- spatial assignment of individual loop dimensions
- tensor reuse
- memory placement
- communication

This simplified mapping allows the vertical slice to exercise the complete
workload → hardware → mapping → prediction pipeline before introducing a
full accelerator mapping representation.

## 5. Result

The v0 result contains:

- total MAC operations
- total available PEs
- active PEs
- theoretical peak MACs per cycle
- effective MACs per cycle
- ideal compute cycles
- compute utilization

The result must also expose the assumptions under which the prediction was made.

## Assumptions and Limitations

Vertical Slice v0 is an idealized compute-only model.

It assumes:

- every active PE performs one MAC every cycle
- work can be perfectly distributed across active PEs
- operands are always available when needed
- output writes introduce no stalls
- communication is free
- memory bandwidth is unlimited
- there is no pipeline fill or drain overhead
- there is no synchronization overhead

Therefore, the predicted cycle count represents an ideal compute lower bound,
not expected real hardware execution time.


## End-to-End Execution

Given:

- an 8 × 8 × 8 matrix multiplication workload
- a 2 × 2 PE array
- four active PEs

FabricFlow should:

1. Parse or construct the workload.
2. Calculate 512 total MAC operations.
3. Parse or construct the hardware topology.
4. Calculate four total PEs and four peak MACs per cycle.
5. Apply the explicit mapping using four active PEs.
6. Calculate an effective rate of four MACs per cycle.
7. Predict 128 ideal compute cycles.
8. Report 100% compute utilization.
9. Report the assumptions and limitations of the model.

## Definition of Done

Vertical Slice v0 is complete when:

- [ ] One matrix multiplication workload can be represented.
- [ ] One PE-array topology can be represented.
- [ ] One explicit compute mapping can be represented.
- [ ] FabricFlow calculates total MAC operations.
- [ ] FabricFlow calculates ideal execution cycles.
- [ ] FabricFlow calculates compute utilization.
- [ ] One example executes the complete pipeline.
- [ ] The result includes model assumptions.
- [ ] Automated tests verify the reference example.
