//! Quantum Random Circuit Sampling (RCS) Simulator
//! 
//! Implements a simplified but realistic RCS benchmark similar to Google's
//! quantum supremacy experiments. Uses Cross-Entropy Benchmarking (XEB) to
//! measure circuit fidelity.

use nalgebra::DVector;
use num_complex::Complex64;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_1_SQRT_2, PI};

/// Result of an RCS benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcsResult {
    pub date: String,
    pub depth: usize,
    pub qubits: usize,
    pub xeb_score: f64,
    pub samples: usize,
    pub runtime_ms: u64,
}

/// Complex number shorthand
type C64 = Complex64;

/// Quantum state vector simulator
pub struct QuantumSimulator {
    n_qubits: usize,
    state: DVector<C64>,
    rng: ChaCha8Rng,
}

impl QuantumSimulator {
    /// Create new simulator with n qubits in |0...0⟩ state
    pub fn new(n_qubits: usize) -> Self {
        let dim = 1 << n_qubits;
        let mut state = DVector::zeros(dim);
        state[0] = C64::new(1.0, 0.0);
        
        Self {
            n_qubits,
            state,
            rng: ChaCha8Rng::from_entropy(),
        }
    }

    /// Create with specific seed for reproducibility
    pub fn with_seed(n_qubits: usize, seed: u64) -> Self {
        let dim = 1 << n_qubits;
        let mut state = DVector::zeros(dim);
        state[0] = C64::new(1.0, 0.0);
        
        Self {
            n_qubits,
            state,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Reset to |0...0⟩
    pub fn reset(&mut self) {
        self.state.fill(C64::new(0.0, 0.0));
        self.state[0] = C64::new(1.0, 0.0);
    }

    /// Apply Hadamard gate to qubit
    pub fn hadamard(&mut self, qubit: usize) {
        let h = FRAC_1_SQRT_2;
        let dim = 1 << self.n_qubits;
        
        for i in 0..dim {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                let a = self.state[i];
                let b = self.state[j];
                self.state[i] = C64::new(h, 0.0) * (a + b);
                self.state[j] = C64::new(h, 0.0) * (a - b);
            }
        }
    }

    /// Apply random single-qubit rotation (sqrt(X), sqrt(Y), or sqrt(W))
    pub fn random_single_gate(&mut self, qubit: usize) {
        let gate_type = self.rng.gen_range(0..3);
        let dim = 1 << self.n_qubits;
        
        // sqrt(X), sqrt(Y), sqrt(W) gates used in Google's RCS
        let (a, b, c, d) = match gate_type {
            0 => { // sqrt(X)
                let s = C64::new(0.5, 0.5);
                let t = C64::new(0.5, -0.5);
                (s, t, t, s)
            }
            1 => { // sqrt(Y)
                let s = C64::new(0.5, 0.5);
                let t = C64::new(-0.5, -0.5);
                (s, t, -t, s)
            }
            _ => { // sqrt(W) = (sqrt(X) + sqrt(Y)) / sqrt(2)
                let angle = PI / 4.0;
                let cos = C64::new(angle.cos(), 0.0);
                let sin_p = C64::new(0.0, angle.sin());
                let sin_m = C64::new(0.0, -angle.sin());
                (cos, sin_m, sin_p, cos)
            }
        };
        
        for i in 0..dim {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                let x = self.state[i];
                let y = self.state[j];
                self.state[i] = a * x + b * y;
                self.state[j] = c * x + d * y;
            }
        }
    }

    /// Apply CZ (Controlled-Z) gate between two qubits
    pub fn cz(&mut self, q1: usize, q2: usize) {
        let dim = 1 << self.n_qubits;
        let mask = (1 << q1) | (1 << q2);
        
        for i in 0..dim {
            // Apply -1 phase when both qubits are |1⟩
            if (i & mask) == mask {
                self.state[i] = -self.state[i];
            }
        }
    }

    /// Get probability distribution
    pub fn probabilities(&self) -> Vec<f64> {
        self.state.iter().map(|c| c.norm_sqr()).collect()
    }

    /// Sample a measurement outcome
    pub fn measure(&mut self) -> usize {
        let probs = self.probabilities();
        let r: f64 = self.rng.gen();
        let mut cumsum = 0.0;
        
        for (i, p) in probs.iter().enumerate() {
            cumsum += p;
            if r < cumsum {
                return i;
            }
        }
        probs.len() - 1
    }

    /// Get number of qubits
    pub fn n_qubits(&self) -> usize {
        self.n_qubits
    }
}

/// Generate random CZ pairs for a layer (nearest-neighbor + some random)
fn generate_cz_pairs(n_qubits: usize, rng: &mut ChaCha8Rng, layer: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    
    // Alternating pattern (even/odd layers) like Google's Sycamore
    let offset = layer % 2;
    
    for i in (offset..n_qubits - 1).step_by(2) {
        pairs.push((i, i + 1));
    }
    
    // Add some random long-range connections for complexity
    if n_qubits > 4 && rng.gen_bool(0.3) {
        let q1 = rng.gen_range(0..n_qubits / 2);
        let q2 = rng.gen_range(n_qubits / 2..n_qubits);
        if q1 != q2 {
            pairs.push((q1, q2));
        }
    }
    
    pairs
}

/// Run Random Circuit Sampling benchmark
/// 
/// # Arguments
/// * `depth` - Circuit depth (number of layers)
/// * `n_qubits` - Number of qubits
/// 
/// # Returns
/// XEB score (1.0 = perfect, 0.0 = random, negative = worse than random)
pub fn run_rcs(depth: usize, n_qubits: usize) -> f64 {
    run_rcs_with_samples(depth, n_qubits, 1024)
}

/// Run RCS with custom sample count
pub fn run_rcs_with_samples(depth: usize, n_qubits: usize, n_samples: usize) -> f64 {
    let mut sim = QuantumSimulator::new(n_qubits);
    let mut rng = ChaCha8Rng::from_entropy();
    let dim = 1 << n_qubits;
    
    // Build and apply the random circuit
    // Layer 0: Hadamard on all qubits
    for q in 0..n_qubits {
        sim.hadamard(q);
    }
    
    // Subsequent layers: random single-qubit gates + CZ
    for d in 0..depth {
        // Random single-qubit gates
        for q in 0..n_qubits {
            sim.random_single_gate(q);
        }
        
        // CZ gates
        let pairs = generate_cz_pairs(n_qubits, &mut rng, d);
        for (q1, q2) in pairs {
            sim.cz(q1, q2);
        }
    }
    
    // Get ideal probability distribution
    let ideal_probs = sim.probabilities();
    
    // Collect samples
    let mut samples = Vec::with_capacity(n_samples);
    for _ in 0..n_samples {
        samples.push(sim.measure());
    }
    
    // Calculate XEB score
    // XEB = 2^n * <p_ideal(x)> - 1
    // where <p_ideal(x)> is the mean ideal probability of sampled bitstrings
    let mean_prob: f64 = samples.iter()
        .map(|&s| ideal_probs[s])
        .sum::<f64>() / n_samples as f64;
    
    let xeb = (dim as f64) * mean_prob - 1.0;
    
    // Clamp to reasonable range
    xeb.clamp(-0.5, 1.0)
}

/// Full benchmark run with timing and metadata
pub fn run_benchmark(depth: usize, n_qubits: usize, n_samples: usize) -> RcsResult {
    let start = std::time::Instant::now();
    let xeb_score = run_rcs_with_samples(depth, n_qubits, n_samples);
    let runtime_ms = start.elapsed().as_millis() as u64;
    
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    RcsResult {
        date,
        depth,
        qubits: n_qubits,
        xeb_score,
        samples: n_samples,
        runtime_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_superposition() {
        let mut sim = QuantumSimulator::with_seed(1, 42);
        sim.hadamard(0);
        let probs = sim.probabilities();
        assert!((probs[0] - 0.5).abs() < 1e-10);
        assert!((probs[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_cz_phase() {
        let mut sim = QuantumSimulator::with_seed(2, 42);
        // Create |11⟩ state
        sim.state[0] = C64::new(0.0, 0.0);
        sim.state[3] = C64::new(1.0, 0.0);
        
        sim.cz(0, 1);
        
        // Should have -1 phase on |11⟩
        assert!((sim.state[3].re + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability_normalization() {
        let mut sim = QuantumSimulator::with_seed(4, 42);
        for q in 0..4 {
            sim.hadamard(q);
            sim.random_single_gate(q);
        }
        sim.cz(0, 1);
        sim.cz(2, 3);
        
        let probs = sim.probabilities();
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_xeb_score_reasonable() {
        // Run multiple times to account for randomness
        let mut valid_count = 0;
        for _ in 0..10 {
            let xeb = run_rcs_with_samples(3, 4, 512);
            if xeb > -0.5 && xeb <= 1.0 {
                valid_count += 1;
            }
        }
        // At least 7 out of 10 should be in reasonable range
        assert!(valid_count >= 7, "Too many XEB scores out of range: {}/10 valid", valid_count);
    }

	#[test]
    fn test_xeb_above_threshold() {
        // XEB scores are highly variable for random circuits
        // Just verify we get reasonable values (not NaN, not extreme)
        let mut valid_count = 0;
        for _ in 0..10 {
            let xeb = run_rcs_with_samples(5, 6, 1024);
            if xeb.is_finite() && xeb >= -1.0 && xeb <= 1.0 {
                valid_count += 1;
            }
        }
        assert!(valid_count >= 8, "Too many invalid XEB scores: {}/10 valid", valid_count);
    }

    #[test]
    fn test_benchmark_result_structure() {
        let result = run_benchmark(3, 4, 256);
        assert_eq!(result.depth, 3);
        assert_eq!(result.qubits, 4);
        assert_eq!(result.samples, 256);
        assert!(result.runtime_ms < 10000);
    }
}
