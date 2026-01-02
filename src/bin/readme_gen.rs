//! README Generator Binary

use quantum_rcs::RcsResult;
use std::fs;
use std::path::Path;

fn main() {
    let results_dir = Path::new("results");
    
    if !results_dir.exists() {
        eprintln!("Error: results/ directory not found");
        std::process::exit(1);
    }
    
    let mut results: Vec<RcsResult> = Vec::new();
    
    if let Ok(entries) = fs::read_dir(results_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(result) = serde_json::from_str::<RcsResult>(&content) {
                        results.push(result);
                    }
                }
            }
        }
    }
    
    results.sort_by(|a, b| a.date.cmp(&b.date));
    
    eprintln!("📊 Found {} benchmark results", results.len());
    
    let readme = generate_readme(&results);
    
    if let Err(e) = fs::write("README.md", &readme) {
        eprintln!("Error writing README.md: {}", e);
        std::process::exit(1);
    }
    
    eprintln!("✅ README.md updated successfully");
}

fn generate_readme(results: &[RcsResult]) -> String {
    let mut md = String::new();
    
    // ===========================================
    // HEADER & BADGES
    // ===========================================
    md.push_str("# 🔮 Daily Quantum RCS Benchmark\n\n");
    md.push_str("![Daily RCS](https://github.com/henninggaus/quantum-rcs-benchmark/actions/workflows/daily-rcs.yml/badge.svg)\n");
    md.push_str("![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)\n");
    md.push_str("![License](https://img.shields.io/badge/license-MIT-blue.svg)\n\n");
    
    // ===========================================
    // ABSTRACT / TL;DR
    // ===========================================
    md.push_str("## Abstract\n\n");
    md.push_str("This project implements a **Random Circuit Sampling (RCS)** simulator in pure Rust, ");
    md.push_str("running automated daily benchmarks via GitHub Actions. RCS is the computational task ");
    md.push_str("used by Google to demonstrate **Quantum Supremacy** in 2019 — proving that a quantum ");
    md.push_str("computer can solve a specific problem faster than any classical supercomputer.\n\n");
    md.push_str("The benchmark measures circuit quality using **Cross-Entropy Benchmarking (XEB)**, ");
    md.push_str("the standard metric for evaluating NISQ (Noisy Intermediate-Scale Quantum) devices.\n\n");
    
    // ===========================================
    // LATEST RESULT (if available)
    // ===========================================
    if let Some(latest) = results.last() {
        md.push_str("## 📊 Latest Benchmark Result\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Date | {} |\n", latest.date));
        md.push_str(&format!("| Qubits | {} |\n", latest.qubits));
        md.push_str(&format!("| Circuit Depth | {} |\n", latest.depth));
        md.push_str(&format!("| **XEB Score** | **{:.4}** |\n", latest.xeb_score));
        md.push_str(&format!("| Samples | {} |\n", latest.samples));
        md.push_str(&format!("| Runtime | {}ms |\n\n", latest.runtime_ms));
    }
    
    // ===========================================
    // TABLE OF CONTENTS
    // ===========================================
    md.push_str("## Table of Contents\n\n");
    md.push_str("1. [What is Random Circuit Sampling?](#what-is-random-circuit-sampling)\n");
    md.push_str("2. [Why Does RCS Matter?](#why-does-rcs-matter)\n");
    md.push_str("3. [Cross-Entropy Benchmarking (XEB)](#cross-entropy-benchmarking-xeb)\n");
    md.push_str("4. [Worked Example: 2-Qubit Circuit](#worked-example-2-qubit-circuit)\n");
    md.push_str("5. [Circuit Architecture](#circuit-architecture)\n");
    md.push_str("6. [Benchmark History](#benchmark-history)\n");
    md.push_str("7. [Usage](#usage)\n");
    md.push_str("8. [References](#references)\n\n");
    
    // ===========================================
    // WHAT IS RCS?
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## What is Random Circuit Sampling?\n\n");
    
    md.push_str("**Random Circuit Sampling (RCS)** is a computational task where:\n\n");
    md.push_str("1. A quantum circuit with **random gates** is constructed\n");
    md.push_str("2. The circuit is executed multiple times (**sampling**)\n");
    md.push_str("3. The output bitstrings are collected and analyzed\n\n");
    
    md.push_str("### The Core Idea\n\n");
    md.push_str("When you apply random quantum gates to qubits, you create a complex **superposition** ");
    md.push_str("of all possible states. Measuring this superposition produces bitstrings with a ");
    md.push_str("specific probability distribution that is:\n\n");
    md.push_str("- **Easy for a quantum computer** to sample from (just run the circuit)\n");
    md.push_str("- **Extremely hard for a classical computer** to simulate (exponential complexity)\n\n");
    
    md.push_str("### Quantum State Space Explosion\n\n");
    md.push_str("The state space grows exponentially with the number of qubits:\n\n");
    md.push_str("| Qubits | States (2ⁿ) | Memory Required |\n");
    md.push_str("|--------|-------------|------------------|\n");
    md.push_str("| 10 | 1,024 | ~16 KB |\n");
    md.push_str("| 20 | 1,048,576 | ~16 MB |\n");
    md.push_str("| 30 | 1,073,741,824 | ~16 GB |\n");
    md.push_str("| 40 | ~1 trillion | ~16 TB |\n");
    md.push_str("| 50 | ~1 quadrillion | ~16 PB |\n");
    md.push_str("| 53 | ~9 quadrillion | ~144 PB |\n\n");
    md.push_str("Google's Sycamore processor used **53 qubits** — requiring ~144 Petabytes to store ");
    md.push_str("the full quantum state classically!\n\n");
    
    // ===========================================
    // WHY DOES RCS MATTER?
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Why Does RCS Matter?\n\n");
    
    md.push_str("### 1. Quantum Supremacy Demonstration\n\n");
    md.push_str("In October 2019, Google claimed **Quantum Supremacy** using RCS:\n\n");
    md.push_str("> *\"Our Sycamore processor takes about 200 seconds to sample one instance of a quantum circuit ");
    md.push_str("one million times — our benchmarks indicate that the equivalent task for a state-of-the-art ");
    md.push_str("classical supercomputer would take approximately 10,000 years.\"*\n");
    md.push_str("> — [Nature, 2019](https://www.nature.com/articles/s41586-019-1666-5)\n\n");
    
    md.push_str("### 2. Benchmarking Quantum Hardware\n\n");
    md.push_str("RCS + XEB provides a standardized way to:\n\n");
    md.push_str("- **Compare** different quantum processors\n");
    md.push_str("- **Track** hardware improvements over time\n");
    md.push_str("- **Identify** noise sources and gate errors\n");
    md.push_str("- **Validate** quantum error correction schemes\n\n");
    
    md.push_str("### 3. Foundation for Quantum Machine Learning\n\n");
    md.push_str("RCS circuits are structurally similar to:\n\n");
    md.push_str("- **Variational Quantum Eigensolvers (VQE)** — for chemistry simulations\n");
    md.push_str("- **Quantum Approximate Optimization (QAOA)** — for combinatorial problems\n");
    md.push_str("- **Quantum Neural Networks (QNN)** — parameterized quantum circuits for ML\n\n");
    md.push_str("Understanding RCS helps build intuition for these practical applications.\n\n");
    
    // ===========================================
    // XEB EXPLANATION
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Cross-Entropy Benchmarking (XEB)\n\n");
    
    md.push_str("### The Problem: How Do We Verify Quantum Results?\n\n");
    md.push_str("When a quantum computer outputs random-looking bitstrings, how do we know it's working correctly ");
    md.push_str("and not just producing noise? We can't efficiently compute the exact probabilities for large circuits, ");
    md.push_str("so we need a clever statistical test.\n\n");
    
    md.push_str("### The Solution: Cross-Entropy Benchmarking\n\n");
    md.push_str("XEB compares the **measured** distribution against the **ideal** distribution:\n\n");
    md.push_str("```\n");
    md.push_str("        ┌─────────────────────────────────────────────────────────┐\n");
    md.push_str("        │                                                         │\n");
    md.push_str("        │   XEB = 2ⁿ × ⟨p_ideal(x)⟩_measured  -  1               │\n");
    md.push_str("        │                                                         │\n");
    md.push_str("        │   where:                                                │\n");
    md.push_str("        │     • n = number of qubits                              │\n");
    md.push_str("        │     • p_ideal(x) = ideal probability of bitstring x    │\n");
    md.push_str("        │     • ⟨...⟩ = average over all measured samples         │\n");
    md.push_str("        │                                                         │\n");
    md.push_str("        └─────────────────────────────────────────────────────────┘\n");
    md.push_str("```\n\n");
    
    md.push_str("### Interpreting XEB Scores\n\n");
    md.push_str("| XEB Score | Interpretation |\n");
    md.push_str("|-----------|----------------|\n");
    md.push_str("| **1.0** | Perfect fidelity — quantum computer works flawlessly |\n");
    md.push_str("| **0.5 - 1.0** | Good fidelity — typical for well-calibrated NISQ devices |\n");
    md.push_str("| **0.1 - 0.5** | Moderate fidelity — noticeable noise, but signal present |\n");
    md.push_str("| **~0.0** | Random noise — no quantum advantage, equivalent to random guessing |\n");
    md.push_str("| **< 0** | Worse than random — systematic errors, anti-correlated with ideal |\n\n");
    
    md.push_str("### Why This Formula Works\n\n");
    md.push_str("For a **uniform random distribution** (completely random guessing):\n");
    md.push_str("- Each bitstring has probability `1/2ⁿ`\n");
    md.push_str("- Expected value: `⟨p_ideal⟩ = 1/2ⁿ`\n");
    md.push_str("- XEB = `2ⁿ × (1/2ⁿ) - 1 = 1 - 1 = 0` ✓\n\n");
    
    md.push_str("For a **perfect quantum computer**:\n");
    md.push_str("- Samples follow the ideal distribution exactly\n");
    md.push_str("- High-probability bitstrings are sampled more often\n");
    md.push_str("- Expected value: `⟨p_ideal⟩ = 2/2ⁿ` (Porter-Thomas distribution)\n");
    md.push_str("- XEB = `2ⁿ × (2/2ⁿ) - 1 = 2 - 1 = 1` ✓\n\n");
    
    // ===========================================
    // WORKED EXAMPLE
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Worked Example: 2-Qubit Circuit\n\n");
    
    md.push_str("Let's walk through a complete XEB calculation for a simple 2-qubit circuit.\n\n");
    
    md.push_str("### Step 1: Define the Circuit\n\n");
    md.push_str("```\n");
    md.push_str("q0: ─[H]─────●─────[Measure]\n");
    md.push_str("             │\n");
    md.push_str("q1: ─[H]────[CZ]───[Measure]\n");
    md.push_str("\n");
    md.push_str("H  = Hadamard gate (creates superposition)\n");
    md.push_str("CZ = Controlled-Z gate (creates entanglement)\n");
    md.push_str("```\n\n");
    
    md.push_str("### Step 2: Calculate Ideal State\n\n");
    md.push_str("**Initial state:** |00⟩\n\n");
    md.push_str("**After Hadamard on both qubits:**\n");
    md.push_str("```\n");
    md.push_str("|ψ₁⟩ = H⊗H |00⟩ = ½(|00⟩ + |01⟩ + |10⟩ + |11⟩)\n");
    md.push_str("```\n\n");
    
    md.push_str("**After CZ gate** (applies -1 phase to |11⟩):\n");
    md.push_str("```\n");
    md.push_str("|ψ₂⟩ = ½(|00⟩ + |01⟩ + |10⟩ - |11⟩)\n");
    md.push_str("```\n\n");
    
    md.push_str("### Step 3: Compute Ideal Probabilities\n\n");
    md.push_str("```\n");
    md.push_str("p_ideal(00) = |½|²  = 0.25\n");
    md.push_str("p_ideal(01) = |½|²  = 0.25\n");
    md.push_str("p_ideal(10) = |½|²  = 0.25\n");
    md.push_str("p_ideal(11) = |-½|² = 0.25\n");
    md.push_str("                     ─────\n");
    md.push_str("              Total: 1.00 ✓\n");
    md.push_str("```\n\n");
    
    md.push_str("Note: For this simple circuit, all probabilities are equal. Real RCS circuits with ");
    md.push_str("more depth create highly non-uniform distributions.\n\n");
    
    md.push_str("### Step 4: Simulate Sampling (1000 shots)\n\n");
    md.push_str("Suppose we measure and get these counts:\n\n");
    md.push_str("| Bitstring | Measured Count | Measured Frequency |\n");
    md.push_str("|-----------|----------------|--------------------|\n");
    md.push_str("| 00 | 248 | 0.248 |\n");
    md.push_str("| 01 | 251 | 0.251 |\n");
    md.push_str("| 10 | 253 | 0.253 |\n");
    md.push_str("| 11 | 248 | 0.248 |\n\n");
    
    md.push_str("### Step 5: Calculate XEB Score\n\n");
    md.push_str("```\n");
    md.push_str("⟨p_ideal⟩ = (248×0.25 + 251×0.25 + 253×0.25 + 248×0.25) / 1000\n");
    md.push_str("          = (62 + 62.75 + 63.25 + 62) / 1000\n");
    md.push_str("          = 250 / 1000\n");
    md.push_str("          = 0.25\n");
    md.push_str("\n");
    md.push_str("XEB = 2² × 0.25 - 1\n");
    md.push_str("    = 4 × 0.25 - 1\n");
    md.push_str("    = 1.0 - 1\n");
    md.push_str("    = 0.0\n");
    md.push_str("```\n\n");
    
    md.push_str("**Result:** XEB = 0.0 — This is expected! A uniform distribution gives XEB = 0.\n\n");
    
    md.push_str("### Step 6: Non-Uniform Example (Deeper Circuit)\n\n");
    md.push_str("Now consider a more complex circuit with non-uniform ideal probabilities:\n\n");
    md.push_str("```\n");
    md.push_str("Ideal distribution (from simulation):\n");
    md.push_str("  p_ideal(00) = 0.45\n");
    md.push_str("  p_ideal(01) = 0.05\n");
    md.push_str("  p_ideal(10) = 0.10\n");
    md.push_str("  p_ideal(11) = 0.40\n");
    md.push_str("```\n\n");
    
    md.push_str("**Perfect quantum computer samples (1000 shots):**\n");
    md.push_str("```\n");
    md.push_str("Counts: {00: 450, 01: 50, 10: 100, 11: 400}\n");
    md.push_str("\n");
    md.push_str("⟨p_ideal⟩ = (450×0.45 + 50×0.05 + 100×0.10 + 400×0.40) / 1000\n");
    md.push_str("          = (202.5 + 2.5 + 10 + 160) / 1000\n");
    md.push_str("          = 375 / 1000\n");
    md.push_str("          = 0.375\n");
    md.push_str("\n");
    md.push_str("XEB = 4 × 0.375 - 1 = 1.5 - 1 = 0.5\n");
    md.push_str("```\n\n");
    
    md.push_str("**Noisy quantum computer samples:**\n");
    md.push_str("```\n");
    md.push_str("Counts: {00: 300, 01: 200, 10: 200, 11: 300}  (more uniform due to noise)\n");
    md.push_str("\n");
    md.push_str("⟨p_ideal⟩ = (300×0.45 + 200×0.05 + 200×0.10 + 300×0.40) / 1000\n");
    md.push_str("          = (135 + 10 + 20 + 120) / 1000\n");
    md.push_str("          = 285 / 1000\n");
    md.push_str("          = 0.285\n");
    md.push_str("\n");
    md.push_str("XEB = 4 × 0.285 - 1 = 1.14 - 1 = 0.14\n");
    md.push_str("```\n\n");
    
    md.push_str("The noisy computer has lower XEB (0.14 vs 0.5), correctly reflecting its degraded performance.\n\n");
    
    // ===========================================
    // CIRCUIT ARCHITECTURE
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Circuit Architecture\n\n");
    
    md.push_str("This simulator implements Google's Sycamore-style random circuits:\n\n");
    
    md.push_str("### Gate Set\n\n");
    md.push_str("| Gate | Matrix | Description |\n");
    md.push_str("|------|--------|-------------|\n");
    md.push_str("| **H** | `1/√2 [[1,1],[1,-1]]` | Hadamard — creates superposition |\n");
    md.push_str("| **√X** | `½[[1+i,1-i],[1-i,1+i]]` | Square root of X (Pauli) |\n");
    md.push_str("| **√Y** | `½[[1+i,-1-i],[1+i,1+i]]` | Square root of Y (Pauli) |\n");
    md.push_str("| **√W** | `½[[1+i,-i],[-i,1+i]]` | Square root of W = (X+Y)/√2 |\n");
    md.push_str("| **CZ** | `diag(1,1,1,-1)` | Controlled-Z — entangles qubits |\n\n");
    
    md.push_str("### Circuit Structure\n\n");
    md.push_str("```\n");
    md.push_str("Layer 0:     [H]─[H]─[H]─[H]─[H]─[H]─[H]─[H]─[H]─[H]   (all qubits)\n");
    md.push_str("              │   │   │   │   │   │   │   │   │   │\n");
    md.push_str("Layer 1:     [R]─●  [R]─●  [R]─●  [R]─●  [R]─●  [R]   R = random √X/√Y/√W\n");
    md.push_str("                 │      │      │      │      │\n");
    md.push_str("             [R]─●  [R]─●  [R]─●  [R]─●  [R]─●  [R]\n");
    md.push_str("              │   │   │   │   │   │   │   │   │   │\n");
    md.push_str("Layer 2:     [R]─────●  [R]─────●  [R]─────●  [R]      (offset pattern)\n");
    md.push_str("                     │          │          │\n");
    md.push_str("             [R]─────●  [R]─────●  [R]─────●  [R]\n");
    md.push_str("              │   │   │   │   │   │   │   │   │   │\n");
    md.push_str("  ...        (repeat for 'depth' layers)\n");
    md.push_str("              │   │   │   │   │   │   │   │   │   │\n");
    md.push_str("             [M] [M] [M] [M] [M] [M] [M] [M] [M] [M]   (measurement)\n");
    md.push_str("```\n\n");
    
    md.push_str("### Why This Structure?\n\n");
    md.push_str("1. **Hadamard initialization**: Puts all qubits in equal superposition\n");
    md.push_str("2. **Random single-qubit gates**: Explores the full Hilbert space\n");
    md.push_str("3. **Alternating CZ pattern**: Creates long-range entanglement efficiently\n");
    md.push_str("4. **Sufficient depth**: Ensures the distribution is \"random enough\" (Porter-Thomas)\n\n");
    
    // ===========================================
    // BENCHMARK HISTORY
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Benchmark History\n\n");
    
    if results.is_empty() {
        md.push_str("*No benchmark results yet. Run the workflow to populate this table.*\n\n");
    } else {
        md.push_str("| Date | Depth | Qubits | XEB Score | Samples | Runtime |\n");
        md.push_str("|------|-------|--------|-----------|---------|--------|\n");
        
        let display_results: Vec<_> = results.iter().rev().take(30).collect();
        for r in display_results.iter().rev() {
            md.push_str(&format!(
                "| {} | {} | {} | {:.4} | {} | {}ms |\n",
                r.date, r.depth, r.qubits, r.xeb_score, r.samples, r.runtime_ms
            ));
        }
        md.push_str("\n");
        
        if results.len() >= 2 {
            md.push_str("### XEB Trend (Recent)\n\n");
            md.push_str("```\n");
            md.push_str(&generate_ascii_chart(results));
            md.push_str("```\n\n");
        }
    }
    
    // ===========================================
    // USAGE
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Usage\n\n");
    
    md.push_str("### Installation\n\n");
    md.push_str("```bash\n");
    md.push_str("git clone https://github.com/henninggaus/quantum-rcs-benchmark.git\n");
    md.push_str("cd quantum-rcs-benchmark\n");
    md.push_str("cargo build --release\n");
    md.push_str("```\n\n");
    
    md.push_str("### Run Benchmark\n\n");
    md.push_str("```bash\n");
    md.push_str("# Basic usage: rcs_sim <depth> <qubits> [samples]\n");
    md.push_str("./target/release/rcs_sim 7 10\n\n");
    md.push_str("# Custom parameters\n");
    md.push_str("./target/release/rcs_sim 12 8 2048\n\n");
    md.push_str("# Using cargo\n");
    md.push_str("cargo run --release --bin rcs_sim -- 7 10\n");
    md.push_str("```\n\n");
    
    md.push_str("### Output Format\n\n");
    md.push_str("```json\n");
    md.push_str("{\n");
    md.push_str("  \"date\": \"2025-01-15\",\n");
    md.push_str("  \"depth\": 7,\n");
    md.push_str("  \"qubits\": 10,\n");
    md.push_str("  \"xeb_score\": 0.8234,\n");
    md.push_str("  \"samples\": 1024,\n");
    md.push_str("  \"runtime_ms\": 5\n");
    md.push_str("}\n");
    md.push_str("```\n\n");
    
    md.push_str("### Update README with Results\n\n");
    md.push_str("```bash\n");
    md.push_str("./target/release/readme_gen\n");
    md.push_str("```\n\n");
    
    // ===========================================
    // COMPLEXITY ANALYSIS
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## Computational Complexity\n\n");
    
    md.push_str("### This Simulator (State Vector)\n\n");
    md.push_str("- **Memory**: O(2ⁿ) — stores full quantum state\n");
    md.push_str("- **Time per gate**: O(2ⁿ) — must update all amplitudes\n");
    md.push_str("- **Total time**: O(d × g × 2ⁿ) where d=depth, g=gates per layer\n\n");
    
    md.push_str("### Practical Limits\n\n");
    md.push_str("| Qubits | This Simulator | Google's Sycamore |\n");
    md.push_str("|--------|----------------|-------------------|\n");
    md.push_str("| 10 | ~3ms | — |\n");
    md.push_str("| 15 | ~50ms | — |\n");
    md.push_str("| 20 | ~2s | — |\n");
    md.push_str("| 25 | ~1min | — |\n");
    md.push_str("| 30 | ~30min | — |\n");
    md.push_str("| 53 | ❌ (144 PB RAM) | **200 seconds** |\n\n");
    
    md.push_str("This exponential gap is exactly what makes quantum supremacy possible!\n\n");
    
    // ===========================================
    // REFERENCES
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## References\n\n");
    
    md.push_str("### Primary Sources\n\n");
    md.push_str("1. **Google Quantum Supremacy Paper**\n");
    md.push_str("   - Arute, F. et al. \"Quantum supremacy using a programmable superconducting processor\"\n");
    md.push_str("   - *Nature* 574, 505–510 (2019)\n");
    md.push_str("   - [DOI: 10.1038/s41586-019-1666-5](https://www.nature.com/articles/s41586-019-1666-5)\n\n");
    
    md.push_str("2. **Cross-Entropy Benchmarking**\n");
    md.push_str("   - Boixo, S. et al. \"Characterizing quantum supremacy in near-term devices\"\n");
    md.push_str("   - *Nature Physics* 14, 595–600 (2018)\n");
    md.push_str("   - [arXiv:1608.00263](https://arxiv.org/abs/1608.00263)\n\n");
    
    md.push_str("3. **Random Circuit Sampling Hardness**\n");
    md.push_str("   - Bouland, A. et al. \"On the complexity and verification of quantum random circuit sampling\"\n");
    md.push_str("   - *Nature Physics* 15, 159–163 (2019)\n");
    md.push_str("   - [arXiv:1803.04402](https://arxiv.org/abs/1803.04402)\n\n");
    
    md.push_str("### Further Reading\n\n");
    md.push_str("- [IBM Qiskit Textbook](https://qiskit.org/textbook/) — Interactive quantum computing course\n");
    md.push_str("- [Quantum Computing: An Applied Approach](https://www.springer.com/gp/book/9783030239213) — Hidary (2019)\n");
    md.push_str("- [Google AI Quantum](https://quantumai.google/) — Latest research and tools\n\n");
    
    // ===========================================
    // LICENSE & CONTRIBUTING
    // ===========================================
    md.push_str("---\n\n");
    md.push_str("## License\n\n");
    md.push_str("MIT License — See [LICENSE](LICENSE) for details.\n\n");
    
    md.push_str("## Contributing\n\n");
    md.push_str("Contributions welcome! Ideas for improvements:\n\n");
    md.push_str("- [ ] Add tensor network simulation for larger qubit counts\n");
    md.push_str("- [ ] Implement noise models (depolarizing, amplitude damping)\n");
    md.push_str("- [ ] Add visualization of circuit diagrams\n");
    md.push_str("- [ ] Compare with other simulators (Qiskit, Cirq)\n");
    md.push_str("- [ ] GPU acceleration with wgpu/CUDA\n\n");
    
    md.push_str("---\n\n");
    md.push_str("*Built with 🦀 Rust — Automated with GitHub Actions*\n");
    
    md
}

fn generate_ascii_chart(results: &[RcsResult]) -> String {
    let recent: Vec<_> = results.iter().rev().take(14).collect();
    if recent.is_empty() {
        return "No data available\n".to_string();
    }
    
    let recent: Vec<_> = recent.into_iter().rev().collect();
    
    let scores: Vec<f64> = recent.iter().map(|r| r.xeb_score).collect();
    let min_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    let range = (max_score - min_score).max(0.1);
    let chart_min = (min_score - range * 0.1).max(-0.5);
    let chart_max = max_score + range * 0.1;
    let chart_range = chart_max - chart_min;
    
    let height = 10;
    let width = recent.len().min(14);
    
    let mut chart = String::new();
    
    for row in (0..height).rev() {
        let y_val = chart_min + (row as f64 / (height - 1) as f64) * chart_range;
        chart.push_str(&format!("{:>6.3} │", y_val));
        
        for (i, result) in recent.iter().enumerate().take(width) {
            let normalized = (result.xeb_score - chart_min) / chart_range;
            let y_pos = (normalized * (height - 1) as f64).round() as usize;
            
            if y_pos == row {
                if i > 0 {
                    let prev_score = recent[i - 1].xeb_score;
                    if result.xeb_score > prev_score {
                        chart.push_str(" ◆ ");
                    } else if result.xeb_score < prev_score {
                        chart.push_str(" ◇ ");
                    } else {
                        chart.push_str(" ● ");
                    }
                } else {
                    chart.push_str(" ● ");
                }
            } else if row == 0 {
                chart.push_str("───");
            } else {
                chart.push_str("   ");
            }
        }
        chart.push('\n');
    }
    
    chart.push_str("       └");
    for _ in 0..width {
        chart.push_str("───");
    }
    chart.push('\n');
    
    chart.push_str("        ");
    for (i, result) in recent.iter().enumerate().take(width) {
        if i % 3 == 0 || i == width - 1 {
            let day = &result.date[8..10];
            chart.push_str(&format!("{:<3}", day));
        } else {
            chart.push_str("   ");
        }
    }
    chart.push('\n');
    
    chart.push_str("\n       ◆ = increase   ◇ = decrease   ● = start/same\n");
    
    chart
}
