use std::fs;
use clap::Parser;
use walkdir::WalkDir;

/// Collect (Rust) source files into one text file for easy upload to LLM
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// root directory of source files
    #[arg(short, long)]
    source_dir: String,
    /// path of output file
    #[arg(short, long)]
    output_file: String,
}
fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let mut collected_source = String::new();
    for entry in WalkDir::new(args.source_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.display().to_string().ends_with(".rs") {
            println!("reading file: {:?}", path);
            let source= match fs::read_to_string(&path) {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to open file: {}\t error: {:?}", path.display(), e);
                    continue
                }
            };
            let mut path_as_comment = String::from("// ");
            path_as_comment.push_str(&path.display().to_string());
            path_as_comment.push_str("\n");
            collected_source.push_str(path_as_comment.as_str());
            collected_source.push_str(&source);
        }
    }

    fs::write(args.output_file, collected_source.as_str()).expect("Failed to write source code");
}
