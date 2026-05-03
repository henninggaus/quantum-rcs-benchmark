# 🔮 Daily Quantum RCS Benchmark

![Daily RCS](https://github.com/henninggaus/quantum-rcs-benchmark/actions/workflows/daily-rcs.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Abstract

This project implements a **Random Circuit Sampling (RCS)** simulator in pure Rust, running automated daily benchmarks via GitHub Actions. RCS is the computational task used by Google to demonstrate **Quantum Supremacy** in 2019 — proving that a quantum computer can solve a specific problem faster than any classical supercomputer.

The benchmark measures circuit quality using **Cross-Entropy Benchmarking (XEB)**, the standard metric for evaluating NISQ (Noisy Intermediate-Scale Quantum) devices.

## 📊 Latest Benchmark Result

| Metric | Value |
|--------|-------|
| Date | 2026-05-03 |
| Qubits | 10 |
| Circuit Depth | 8 |
| **XEB Score** | **1.0000** |
| Samples | 1024 |
| Runtime | 2ms |

## Table of Contents

1. [What is Random Circuit Sampling?](#what-is-random-circuit-sampling)
2. [Why Does RCS Matter?](#why-does-rcs-matter)
3. [Cross-Entropy Benchmarking (XEB)](#cross-entropy-benchmarking-xeb)
4. [Worked Example: 2-Qubit Circuit](#worked-example-2-qubit-circuit)
5. [Circuit Architecture](#circuit-architecture)
6. [Benchmark History](#benchmark-history)
7. [Usage](#usage)
8. [References](#references)

---

## What is Random Circuit Sampling?

**Random Circuit Sampling (RCS)** is a computational task where:

1. A quantum circuit with **random gates** is constructed
2. The circuit is executed multiple times (**sampling**)
3. The output bitstrings are collected and analyzed

### The Core Idea

When you apply random quantum gates to qubits, you create a complex **superposition** of all possible states. Measuring this superposition produces bitstrings with a specific probability distribution that is:

- **Easy for a quantum computer** to sample from (just run the circuit)
- **Extremely hard for a classical computer** to simulate (exponential complexity)

### Quantum State Space Explosion

The state space grows exponentially with the number of qubits:

| Qubits | States (2ⁿ) | Memory Required |
|--------|-------------|------------------|
| 10 | 1,024 | ~16 KB |
| 20 | 1,048,576 | ~16 MB |
| 30 | 1,073,741,824 | ~16 GB |
| 40 | ~1 trillion | ~16 TB |
| 50 | ~1 quadrillion | ~16 PB |
| 53 | ~9 quadrillion | ~144 PB |

Google's Sycamore processor used **53 qubits** — requiring ~144 Petabytes to store the full quantum state classically!

---

## Why Does RCS Matter?

### 1. Quantum Supremacy Demonstration

In October 2019, Google claimed **Quantum Supremacy** using RCS:

> *"Our Sycamore processor takes about 200 seconds to sample one instance of a quantum circuit one million times — our benchmarks indicate that the equivalent task for a state-of-the-art classical supercomputer would take approximately 10,000 years."*
> — [Nature, 2019](https://www.nature.com/articles/s41586-019-1666-5)

### 2. Benchmarking Quantum Hardware

RCS + XEB provides a standardized way to:

- **Compare** different quantum processors
- **Track** hardware improvements over time
- **Identify** noise sources and gate errors
- **Validate** quantum error correction schemes

### 3. Foundation for Quantum Machine Learning

RCS circuits are structurally similar to:

- **Variational Quantum Eigensolvers (VQE)** — for chemistry simulations
- **Quantum Approximate Optimization (QAOA)** — for combinatorial problems
- **Quantum Neural Networks (QNN)** — parameterized quantum circuits for ML

Understanding RCS helps build intuition for these practical applications.

---

## Cross-Entropy Benchmarking (XEB)

### The Problem: How Do We Verify Quantum Results?

When a quantum computer outputs random-looking bitstrings, how do we know it's working correctly and not just producing noise? We can't efficiently compute the exact probabilities for large circuits, so we need a clever statistical test.

### The Solution: Cross-Entropy Benchmarking

XEB compares the **measured** distribution against the **ideal** distribution:

```
        ┌─────────────────────────────────────────────────────────┐
        │                                                         │
        │   XEB = 2ⁿ × ⟨p_ideal(x)⟩_measured  -  1               │
        │                                                         │
        │   where:                                                │
        │     • n = number of qubits                              │
        │     • p_ideal(x) = ideal probability of bitstring x    │
        │     • ⟨...⟩ = average over all measured samples         │
        │                                                         │
        └─────────────────────────────────────────────────────────┘
```

### Interpreting XEB Scores

| XEB Score | Interpretation |
|-----------|----------------|
| **1.0** | Perfect fidelity — quantum computer works flawlessly |
| **0.5 - 1.0** | Good fidelity — typical for well-calibrated NISQ devices |
| **0.1 - 0.5** | Moderate fidelity — noticeable noise, but signal present |
| **~0.0** | Random noise — no quantum advantage, equivalent to random guessing |
| **< 0** | Worse than random — systematic errors, anti-correlated with ideal |

### Why This Formula Works

For a **uniform random distribution** (completely random guessing):
- Each bitstring has probability `1/2ⁿ`
- Expected value: `⟨p_ideal⟩ = 1/2ⁿ`
- XEB = `2ⁿ × (1/2ⁿ) - 1 = 1 - 1 = 0` ✓

For a **perfect quantum computer**:
- Samples follow the ideal distribution exactly
- High-probability bitstrings are sampled more often
- Expected value: `⟨p_ideal⟩ = 2/2ⁿ` (Porter-Thomas distribution)
- XEB = `2ⁿ × (2/2ⁿ) - 1 = 2 - 1 = 1` ✓

---

## Worked Example: 2-Qubit Circuit

Let's walk through a complete XEB calculation for a simple 2-qubit circuit.

### Step 1: Define the Circuit

```
q0: ─[H]─────●─────[Measure]
             │
q1: ─[H]────[CZ]───[Measure]

H  = Hadamard gate (creates superposition)
CZ = Controlled-Z gate (creates entanglement)
```

### Step 2: Calculate Ideal State

**Initial state:** |00⟩

**After Hadamard on both qubits:**
```
|ψ₁⟩ = H⊗H |00⟩ = ½(|00⟩ + |01⟩ + |10⟩ + |11⟩)
```

**After CZ gate** (applies -1 phase to |11⟩):
```
|ψ₂⟩ = ½(|00⟩ + |01⟩ + |10⟩ - |11⟩)
```

### Step 3: Compute Ideal Probabilities

```
p_ideal(00) = |½|²  = 0.25
p_ideal(01) = |½|²  = 0.25
p_ideal(10) = |½|²  = 0.25
p_ideal(11) = |-½|² = 0.25
                     ─────
              Total: 1.00 ✓
```

Note: For this simple circuit, all probabilities are equal. Real RCS circuits with more depth create highly non-uniform distributions.

### Step 4: Simulate Sampling (1000 shots)

Suppose we measure and get these counts:

| Bitstring | Measured Count | Measured Frequency |
|-----------|----------------|--------------------|
| 00 | 248 | 0.248 |
| 01 | 251 | 0.251 |
| 10 | 253 | 0.253 |
| 11 | 248 | 0.248 |

### Step 5: Calculate XEB Score

```
⟨p_ideal⟩ = (248×0.25 + 251×0.25 + 253×0.25 + 248×0.25) / 1000
          = (62 + 62.75 + 63.25 + 62) / 1000
          = 250 / 1000
          = 0.25

XEB = 2² × 0.25 - 1
    = 4 × 0.25 - 1
    = 1.0 - 1
    = 0.0
```

**Result:** XEB = 0.0 — This is expected! A uniform distribution gives XEB = 0.

### Step 6: Non-Uniform Example (Deeper Circuit)

Now consider a more complex circuit with non-uniform ideal probabilities:

```
Ideal distribution (from simulation):
  p_ideal(00) = 0.45
  p_ideal(01) = 0.05
  p_ideal(10) = 0.10
  p_ideal(11) = 0.40
```

**Perfect quantum computer samples (1000 shots):**
```
Counts: {00: 450, 01: 50, 10: 100, 11: 400}

⟨p_ideal⟩ = (450×0.45 + 50×0.05 + 100×0.10 + 400×0.40) / 1000
          = (202.5 + 2.5 + 10 + 160) / 1000
          = 375 / 1000
          = 0.375

XEB = 4 × 0.375 - 1 = 1.5 - 1 = 0.5
```

**Noisy quantum computer samples:**
```
Counts: {00: 300, 01: 200, 10: 200, 11: 300}  (more uniform due to noise)

⟨p_ideal⟩ = (300×0.45 + 200×0.05 + 200×0.10 + 300×0.40) / 1000
          = (135 + 10 + 20 + 120) / 1000
          = 285 / 1000
          = 0.285

XEB = 4 × 0.285 - 1 = 1.14 - 1 = 0.14
```

The noisy computer has lower XEB (0.14 vs 0.5), correctly reflecting its degraded performance.

---

## Circuit Architecture

This simulator implements Google's Sycamore-style random circuits:

### Gate Set

| Gate | Matrix | Description |
|------|--------|-------------|
| **H** | `1/√2 [[1,1],[1,-1]]` | Hadamard — creates superposition |
| **√X** | `½[[1+i,1-i],[1-i,1+i]]` | Square root of X (Pauli) |
| **√Y** | `½[[1+i,-1-i],[1+i,1+i]]` | Square root of Y (Pauli) |
| **√W** | `½[[1+i,-i],[-i,1+i]]` | Square root of W = (X+Y)/√2 |
| **CZ** | `diag(1,1,1,-1)` | Controlled-Z — entangles qubits |

### Circuit Structure

```
Layer 0:     [H]─[H]─[H]─[H]─[H]─[H]─[H]─[H]─[H]─[H]   (all qubits)
              │   │   │   │   │   │   │   │   │   │
Layer 1:     [R]─●  [R]─●  [R]─●  [R]─●  [R]─●  [R]   R = random √X/√Y/√W
                 │      │      │      │      │
             [R]─●  [R]─●  [R]─●  [R]─●  [R]─●  [R]
              │   │   │   │   │   │   │   │   │   │
Layer 2:     [R]─────●  [R]─────●  [R]─────●  [R]      (offset pattern)
                     │          │          │
             [R]─────●  [R]─────●  [R]─────●  [R]
              │   │   │   │   │   │   │   │   │   │
  ...        (repeat for 'depth' layers)
              │   │   │   │   │   │   │   │   │   │
             [M] [M] [M] [M] [M] [M] [M] [M] [M] [M]   (measurement)
```

### Why This Structure?

1. **Hadamard initialization**: Puts all qubits in equal superposition
2. **Random single-qubit gates**: Explores the full Hilbert space
3. **Alternating CZ pattern**: Creates long-range entanglement efficiently
4. **Sufficient depth**: Ensures the distribution is "random enough" (Porter-Thomas)

---

## Benchmark History

| Date | Depth | Qubits | XEB Score | Samples | Runtime |
|------|-------|--------|-----------|---------|--------|
| 2026-03-13 | 13 | 10 | -0.5000 | 1024 | 4ms |
| 2026-03-14 | 14 | 10 | -0.5000 | 1024 | 4ms |
| 2026-03-15 | 5 | 10 | -0.5000 | 1024 | 3ms |
| 2026-03-16 | 6 | 10 | 1.0000 | 1024 | 3ms |
| 2026-03-17 | 7 | 10 | -0.5000 | 1024 | 3ms |
| 2026-03-18 | 8 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-10 | 5 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-11 | 6 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-12 | 7 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-13 | 8 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-14 | 9 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-15 | 10 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-16 | 11 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-17 | 12 | 10 | 1.0000 | 1024 | 3ms |
| 2026-04-18 | 13 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-19 | 14 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-20 | 5 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-21 | 6 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-22 | 7 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-23 | 8 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-24 | 9 | 10 | 1.0000 | 1024 | 2ms |
| 2026-04-25 | 10 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-26 | 11 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-27 | 12 | 10 | 1.0000 | 1024 | 3ms |
| 2026-04-28 | 13 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-29 | 14 | 10 | -0.5000 | 1024 | 3ms |
| 2026-04-30 | 5 | 10 | -0.5000 | 1024 | 3ms |
| 2026-05-01 | 6 | 10 | 1.0000 | 1024 | 2ms |
| 2026-05-02 | 7 | 10 | -0.5000 | 1024 | 3ms |
| 2026-05-03 | 8 | 10 | 1.0000 | 1024 | 2ms |

### XEB Trend (Recent)

```
 1.150 │                                          
 0.967 │ ●  ●     ◆  ◇        ◆           ◆     ◆ 
 0.783 │                                          
 0.600 │                                          
 0.417 │                                          
 0.233 │                                          
 0.050 │                                          
-0.133 │                                          
-0.317 │                                          
-0.500 │────── ◇ ────── ◇  ● ─── ◇  ●  ● ─── ◇ ───
       └──────────────────────────────────────────
        20       23       26       29       02 03 

       ◆ = increase   ◇ = decrease   ● = start/same
```

---

## Usage

### Installation

```bash
git clone https://github.com/henninggaus/quantum-rcs-benchmark.git
cd quantum-rcs-benchmark
cargo build --release
```

### Run Benchmark

```bash
# Basic usage: rcs_sim <depth> <qubits> [samples]
./target/release/rcs_sim 7 10

# Custom parameters
./target/release/rcs_sim 12 8 2048

# Using cargo
cargo run --release --bin rcs_sim -- 7 10
```

### Output Format

```json
{
  "date": "2025-01-15",
  "depth": 7,
  "qubits": 10,
  "xeb_score": 0.8234,
  "samples": 1024,
  "runtime_ms": 5
}
```

### Update README with Results

```bash
./target/release/readme_gen
```

---

## Computational Complexity

### This Simulator (State Vector)

- **Memory**: O(2ⁿ) — stores full quantum state
- **Time per gate**: O(2ⁿ) — must update all amplitudes
- **Total time**: O(d × g × 2ⁿ) where d=depth, g=gates per layer

### Practical Limits

| Qubits | This Simulator | Google's Sycamore |
|--------|----------------|-------------------|
| 10 | ~3ms | — |
| 15 | ~50ms | — |
| 20 | ~2s | — |
| 25 | ~1min | — |
| 30 | ~30min | — |
| 53 | ❌ (144 PB RAM) | **200 seconds** |

This exponential gap is exactly what makes quantum supremacy possible!

---

## References

### Primary Sources

1. **Google Quantum Supremacy Paper**
   - Arute, F. et al. "Quantum supremacy using a programmable superconducting processor"
   - *Nature* 574, 505–510 (2019)
   - [DOI: 10.1038/s41586-019-1666-5](https://www.nature.com/articles/s41586-019-1666-5)

2. **Cross-Entropy Benchmarking**
   - Boixo, S. et al. "Characterizing quantum supremacy in near-term devices"
   - *Nature Physics* 14, 595–600 (2018)
   - [arXiv:1608.00263](https://arxiv.org/abs/1608.00263)

3. **Random Circuit Sampling Hardness**
   - Bouland, A. et al. "On the complexity and verification of quantum random circuit sampling"
   - *Nature Physics* 15, 159–163 (2019)
   - [arXiv:1803.04402](https://arxiv.org/abs/1803.04402)

### Further Reading

- [IBM Qiskit Textbook](https://qiskit.org/textbook/) — Interactive quantum computing course
- [Quantum Computing: An Applied Approach](https://www.springer.com/gp/book/9783030239213) — Hidary (2019)
- [Google AI Quantum](https://quantumai.google/) — Latest research and tools

---

## License

MIT License — See [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Ideas for improvements:

- [ ] Add tensor network simulation for larger qubit counts
- [ ] Implement noise models (depolarizing, amplitude damping)
- [ ] Add visualization of circuit diagrams
- [ ] Compare with other simulators (Qiskit, Cirq)
- [ ] GPU acceleration with wgpu/CUDA

---

*Built with 🦀 Rust — Automated with GitHub Actions*
