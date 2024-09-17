mod time;

use anyhow::Ok;
use clap::Parser;
use nu_ansi_term::Color::Red;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(required = true, index = 1, help = "Command to execute")]
    cmd: String,
}

fn main() {
    let args = Args::parse();
    let cmd = &args.cmd.as_str();

    let result = time::get_time(cmd);
    if let Err(err) = result {
        eprintln!("{}", Red.paint(err.to_string()));
        std::process::exit(1);
    }
    
    Ok(()).expect("Failed to execute command");
}
