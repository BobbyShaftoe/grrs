use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();

    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => content,
        Err(error) => { return Err(error.into()); },
    };
    println!("File content: {}", content);
    Ok(())

    
}
