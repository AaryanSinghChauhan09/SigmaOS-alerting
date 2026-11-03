use anyhow::Result;
use clap::Parser;
use alerting::alerting;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = alerting::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = alerting::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-alerting running");
    Ok(())
}
