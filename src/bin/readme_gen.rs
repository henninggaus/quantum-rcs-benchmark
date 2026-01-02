//! README Generator Binary
//! 
//! Reads all JSON results from results/ directory and generates
//! an updated README.md with a table and ASCII trend chart.

use quantum_rcs::RcsResult;
use std::fs;
use std::path::Path;

fn main() {
    let results_dir = Path::new("results");
    
    if !results_dir.exists() {
        eprintln!("Error: results/ directory not found");
        std::process::exit(1);
    }
    
    // Read all JSON files
    let mut results: Vec<RcsResult> = Vec::new();
    
    if let Ok(entries) = fs::read_dir(results_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(result) = serde_json::from_str::<RcsResult>(&content) {
                        results.push(result);
                    }
                }
            }
        }
    }
    
    // Sort by date
    results.sort_by(|a, b| a.date.cmp(&b.date));
    
    eprintln!("📊 Found {} benchmark results", results.len());
    
    // Generate README content
    let readme = generate_readme(&results);
    
    // Write README.md
    if let Err(e) = fs::write("README.md", &readme) {
        eprintln!("Error writing README.md: {}", e);
        std::process::exit(1);
    }
    
    eprintln!("✅ README.md updated successfully");
}

fn generate_readme(results: &[RcsResult]) -> String {
    let mut md = String::new();
    
    // Header
    md.push_str("# 🔮 Daily Quantum RCS Benchmark\n\n");
    md.push_str("![Daily RCS](https://github.com/yourusername/quantum-rcs-benchmark/actions/workflows/daily-rcs.yml/badge.svg)\n");
    md.push_str("![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)\n");
    md.push_str("![License](https://img.shields.io/badge/license-MIT-blue.svg)\n\n");
    
    // Description
    md.push_str("## About\n\n");
    md.push_str("Täglicher Quantum Benchmark mit Rust. **Random Circuit Sampling (RCS)** simuliert \n");
    md.push_str("NISQ-Circuits ähnlich Google's Sycamore Supremacy-Experiment. Der **XEB-Score** \n");
    md.push_str("(Cross-Entropy Benchmarking) misst die Schaltkreis-Qualität — Basis für VQAs und QML.\n\n");
    
    md.push_str("### Key Features\n\n");
    md.push_str("- 🚀 Pure Rust state-vector simulation with `nalgebra`\n");
    md.push_str("- 📈 Daily automated benchmarks via GitHub Actions\n");
    md.push_str("- 🎯 XEB scoring following Google's methodology\n");
    md.push_str("- 🔄 Variable depth circuits (5-15 layers)\n\n");
    
    // Latest result highlight
    if let Some(latest) = results.last() {
        md.push_str("## Latest Result\n\n");
        md.push_str(&format!("| Metric | Value |\n"));
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Date | {} |\n", latest.date));
        md.push_str(&format!("| Qubits | {} |\n", latest.qubits));
        md.push_str(&format!("| Depth | {} |\n", latest.depth));
        md.push_str(&format!("| **XEB Score** | **{:.4}** |\n", latest.xeb_score));
        md.push_str(&format!("| Samples | {} |\n", latest.samples));
        md.push_str(&format!("| Runtime | {}ms |\n\n", latest.runtime_ms));
    }
    
    // Results table
    md.push_str("## Benchmark History\n\n");
    md.push_str("| Date | Depth | Qubits | XEB Score | Samples | Runtime |\n");
    md.push_str("|------|-------|--------|-----------|---------|--------|\n");
    
    // Show last 30 results
    let display_results: Vec<_> = results.iter().rev().take(30).collect();
    for r in display_results.iter().rev() {
        md.push_str(&format!(
            "| {} | {} | {} | {:.4} | {} | {}ms |\n",
            r.date, r.depth, r.qubits, r.xeb_score, r.samples, r.runtime_ms
        ));
    }
    md.push('\n');
    
    // ASCII Chart - last 7 days
    if results.len() >= 2 {
        md.push_str("## XEB Trend (Recent)\n\n");
        md.push_str("```\n");
        md.push_str(&generate_ascii_chart(results));
        md.push_str("```\n\n");
    }
    
    // Usage
    md.push_str("## Usage\n\n");
    md.push_str("### Build\n\n");
    md.push_str("```bash\n");
    md.push_str("cargo build --release\n");
    md.push_str("```\n\n");
    
    md.push_str("### Run Benchmark\n\n");
    md.push_str("```bash\n");
    md.push_str("# Default: depth=7, qubits=10, samples=1024\n");
    md.push_str("cargo run --release --bin rcs_sim -- 7 10\n\n");
    md.push_str("# Custom parameters\n");
    md.push_str("cargo run --release --bin rcs_sim -- 12 8 2048\n");
    md.push_str("```\n\n");
    
    md.push_str("### Update README\n\n");
    md.push_str("```bash\n");
    md.push_str("cargo run --release --bin readme_gen\n");
    md.push_str("```\n\n");
    
    // Technical details
    md.push_str("## Technical Details\n\n");
    md.push_str("### Circuit Structure\n\n");
    md.push_str("Each random circuit consists of:\n");
    md.push_str("1. Initial Hadamard layer on all qubits\n");
    md.push_str("2. `depth` layers of:\n");
    md.push_str("   - Random single-qubit gates (√X, √Y, √W)\n");
    md.push_str("   - CZ gates in alternating patterns\n\n");
    
    md.push_str("### XEB Score\n\n");
    md.push_str("The Cross-Entropy Benchmark score is calculated as:\n\n");
    md.push_str("```\n");
    md.push_str("XEB = 2^n × ⟨p_ideal(x)⟩ - 1\n");
    md.push_str("```\n\n");
    md.push_str("Where `⟨p_ideal(x)⟩` is the mean ideal probability of sampled bitstrings.\n\n");
    md.push_str("- **XEB = 1.0**: Perfect fidelity\n");
    md.push_str("- **XEB = 0.0**: Random noise\n");
    md.push_str("- **XEB < 0**: Worse than random\n\n");
    
    // References
    md.push_str("## References\n\n");
    md.push_str("- [Google Quantum AI: Quantum Supremacy](https://www.nature.com/articles/s41586-019-1666-5)\n");
    md.push_str("- [Cross-Entropy Benchmarking](https://arxiv.org/abs/1608.00263)\n\n");
    
    // License
    md.push_str("## License\n\n");
    md.push_str("MIT License - See [LICENSE](LICENSE) for details.\n");
    
    md
}

fn generate_ascii_chart(results: &[RcsResult]) -> String {
    let recent: Vec<_> = results.iter().rev().take(14).collect();
    if recent.is_empty() {
        return String::from("No data available\n");
    }
    
    let recent: Vec<_> = recent.into_iter().rev().collect();
    
    // Find min/max for scaling
    let scores: Vec<f64> = recent.iter().map(|r| r.xeb_score).collect();
    let min_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    // Add some padding
    let range = (max_score - min_score).max(0.1);
    let chart_min = (min_score - range * 0.1).max(-0.5);
    let chart_max = max_score + range * 0.1;
    let chart_range = chart_max - chart_min;
    
    let height = 10;
    let width = recent.len().min(14);
    
    let mut chart = String::new();
    
    // Y-axis labels and chart
    for row in (0..height).rev() {
        let y_val = chart_min + (row as f64 / (height - 1) as f64) * chart_range;
        chart.push_str(&format!("{:>6.3} │", y_val));
        
        for (i, result) in recent.iter().enumerate().take(width) {
            let normalized = (result.xeb_score - chart_min) / chart_range;
            let y_pos = (normalized * (height - 1) as f64).round() as usize;
            
            if y_pos == row {
                // Check for trend
                if i > 0 {
                    let prev_score = recent[i - 1].xeb_score;
                    if result.xeb_score > prev_score {
                        chart.push_str(" ◆ ");  // Up
                    } else if result.xeb_score < prev_score {
                        chart.push_str(" ◇ ");  // Down
                    } else {
                        chart.push_str(" ● ");  // Same
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
    
    // X-axis
    chart.push_str("       └");
    for _ in 0..width {
        chart.push_str("───");
    }
    chart.push('\n');
    
    // Date labels (abbreviated)
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
    
    // Legend
    chart.push_str("\n       ◆ = increase   ◇ = decrease   ● = start/same\n");
    
    chart
}
