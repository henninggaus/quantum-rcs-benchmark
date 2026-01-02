# 🔮 Daily Quantum RCS Benchmark

![Daily RCS](https://github.com/henninggaus/quantum-rcs-benchmark/actions/workflows/daily-rcs.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## About

Täglicher Quantum Benchmark mit Rust. **Random Circuit Sampling (RCS)** simuliert 
NISQ-Circuits ähnlich Google's Sycamore Supremacy-Experiment. Der **XEB-Score** 
(Cross-Entropy Benchmarking) misst die Schaltkreis-Qualität — Basis für VQAs und QML.

### Key Features

- 🚀 Pure Rust state-vector simulation with `nalgebra`
- 📈 Daily automated benchmarks via GitHub Actions
- 🎯 XEB scoring following Google's methodology
- 🔄 Variable depth circuits (5-15 layers)

## Latest Result

| Metric | Value |
|--------|-------|
| Date | 2026-01-02 |
| Qubits | 10 |
| Depth | 7 |
| **XEB Score** | **1.0000** |
| Samples | 1024 |
| Runtime | 3ms |

## Benchmark History

| Date | Depth | Qubits | XEB Score | Samples | Runtime |
|------|-------|--------|-----------|---------|--------|
| 2026-01-02 | 7 | 10 | 1.0000 | 1024 | 3ms |

## Usage

### Build

```bash
cargo build --release
```

### Run Benchmark

```bash
# Default: depth=7, qubits=10, samples=1024
cargo run --release --bin rcs_sim -- 7 10

# Custom parameters
cargo run --release --bin rcs_sim -- 12 8 2048
```

### Update README

```bash
cargo run --release --bin readme_gen
```

## Technical Details

### Circuit Structure

Each random circuit consists of:
1. Initial Hadamard layer on all qubits
2. `depth` layers of:
   - Random single-qubit gates (√X, √Y, √W)
   - CZ gates in alternating patterns

### XEB Score

The Cross-Entropy Benchmark score is calculated as:

```
XEB = 2^n × ⟨p_ideal(x)⟩ - 1
```

Where `⟨p_ideal(x)⟩` is the mean ideal probability of sampled bitstrings.

- **XEB = 1.0**: Perfect fidelity
- **XEB = 0.0**: Random noise
- **XEB < 0**: Worse than random

## References

- [Google Quantum AI: Quantum Supremacy](https://www.nature.com/articles/s41586-019-1666-5)
- [Cross-Entropy Benchmarking](https://arxiv.org/abs/1608.00263)

## License

MIT License - See [LICENSE](LICENSE) for details.
