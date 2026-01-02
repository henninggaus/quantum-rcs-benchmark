//! RCS Simulator Binary
//! 
//! Usage: rcs_sim <depth> <n_qubits> [samples]
//! Output: JSON result to stdout

use quantum_rcs::run_benchmark;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <depth> <n_qubits> [samples]", args[0]);
        eprintln!("Example: {} 7 10", args[0]);
        std::process::exit(1);
    }
    
    let depth: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: depth must be a positive integer");
        std::process::exit(1);
    });
    
    let n_qubits: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Error: n_qubits must be a positive integer");
        std::process::exit(1);
    });
    
    let samples: usize = args.get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1024);
    
    // Validate inputs
    if depth == 0 || depth > 50 {
        eprintln!("Error: depth must be between 1 and 50");
        std::process::exit(1);
    }
    
    if !(2..=20).contains(&n_qubits) {
        eprintln!("Error: n_qubits must be between 2 and 20");
        std::process::exit(1);
    }
    
    eprintln!("🔬 Running RCS Benchmark");
    eprintln!("   Depth: {}", depth);
    eprintln!("   Qubits: {}", n_qubits);
    eprintln!("   Samples: {}", samples);
    eprintln!();
    
    // Run benchmark
    let result = run_benchmark(depth, n_qubits, samples);
    
    eprintln!("✅ Complete!");
    eprintln!("   XEB Score: {:.4}", result.xeb_score);
    eprintln!("   Runtime: {}ms", result.runtime_ms);
    
    // Output JSON
    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", json);
    
    // Also save to results directory if it exists
    let results_dir = Path::new("results");
    if results_dir.exists() {
        let filename = format!("results/{}.json", 
            chrono::Utc::now().format("%Y%m%d"));
        if let Err(e) = fs::write(&filename, &json) {
            eprintln!("Warning: Could not write to {}: {}", filename, e);
        } else {
            eprintln!("   Saved to: {}", filename);
        }
    }
}
