use std::time::Instant;
use std::process::Command;
use std::str;

use anyhow::{Ok, Result};
use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

pub fn get_time(cmd: &str) -> Result<()> {
    let start = Instant::now();

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    let pid = std::process::id();

    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    let cpu_usage = get_cpu_usage(pid)?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    println!("========================");
    println!("Program        : {}", cmd);
    println!("PID            : {}", pid);
    println!("CPU Usage      : {:.2}%", cpu_usage);
    println!("Memory Usage   : {}KB", current_mem);
    println!("Elapsed Time   : {:?}s", duration.as_secs_f64());
    println!("========================");

    Ok(())
}

pub fn get_cpu_usage(pid: u32) -> Result<f32> {
    let output = if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(format!("ps -p {} -o %cpu", pid))
            .output()?
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg(format!("ps -p {} -o %cpu | awk 'NR>1 {{print $1}}'", pid))
            .output()?
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS").into());
    };

    let cpu_usage = str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);

    Ok(cpu_usage)
}