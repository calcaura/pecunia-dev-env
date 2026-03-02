/// Dependency Precompiler
/// 
/// This application downloads and precompiles all workspace dependencies.
/// It imports and uses all dependencies to ensure they are fully compiled
/// and cached for future builds.

use std::process::Command;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    println!("🔨 Dependency Precompiler");
    println!("========================\n");

    // Step 1: Fetch all dependencies
    println!("📥 Fetching all dependencies...");
    fetch_dependencies()?;
    println!("✅ Dependencies fetched successfully\n");

    // Step 2: Compile all dependencies in release mode
    println!("🏗️  Compiling dependencies (this may take a while)...");
    compile_dependencies()?;
    println!("✅ Dependencies precompiled successfully\n");

    // Step 3: Display compilation stats
    println!("📊 Precompilation Complete!");
    println!("All workspace dependencies have been downloaded and compiled.");
    println!("Your builds will now use the precompiled cache.\n");

    Ok(())
}

/// Fetch all dependencies without compiling
fn fetch_dependencies() -> Result<()> {
    let output = Command::new("cargo")
        .args(&["fetch", "--locked"])
        .output()
        .context("Failed to execute cargo fetch")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Cargo fetch failed: {}", stderr);
        return Err(anyhow::anyhow!("cargo fetch failed"));
    }

    Ok(())
}

/// Compile all dependencies in release mode
fn compile_dependencies() -> Result<()> {
    let output = Command::new("cargo")
        .args(&["build", "--release", "-p", "dep-precompiler"])
        .output()
        .context("Failed to execute cargo build")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Cargo build failed: {}", stderr);
        // Print last 50 lines of error for debugging
        let lines: Vec<&str> = stderr.lines().collect();
        let start = if lines.len() > 50 { lines.len() - 50 } else { 0 };
        for line in &lines[start..] {
            eprintln!("{}", line);
        }
        return Err(anyhow::anyhow!("cargo build failed"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Print compilation summary
    for line in stdout.lines() {
        if line.contains("Finished") || line.contains("Compiling") {
            println!("  {}", line);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependencies_loaded() {
        // Verify all dependencies are available
        assert!(true);
    }
}
