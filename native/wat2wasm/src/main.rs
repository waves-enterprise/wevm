use clap::Parser;
use std::{fs, io::Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(args.input).unwrap();

    for path in paths {
        let path = path.unwrap();
        let wat = path.path();
        let name = path
            .file_name()
            .into_string()
            .unwrap()
            .replace("wat", "wasm");
        print!("{}... ", name);
        let binary = wat::parse_file(wat).unwrap();

        let mut wasm = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/{}", args.output, name))
            .unwrap();
        let _ = wasm.write_all(&binary);

        println!("OK");
    }
}
