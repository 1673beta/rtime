use std::time::Instant;
use std::process::Command;

use anyhow::{Ok, Result};
use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

pub fn get_time(cmd: &str, args: &[&str]) -> Result<()> {
    let start = Instant::now();

    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    let pid = std::process::id();

    let current_mem = PEAK_ALLOC.current_usage_as_kb();

    println!("{}", String::from_utf8_lossy(&output.stdout));

    println!("========================");
    println!("Program        : {}", cmd);
    println!("PID            : {}", pid);
    println!("Memory Usage   : {}KB", current_mem);
    println!("Elapsed Time   : {:?}s", duration.as_secs_f64());
    println!("========================");

    Ok(())
}