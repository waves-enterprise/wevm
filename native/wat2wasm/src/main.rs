use std::fs;
use std::io::Write;

fn main() {
    let paths = fs::read_dir("./wat").unwrap();

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
            .open(format!("../../src/test/resources/{}", name))
            .unwrap();

        let _ = wasm.write_all(&binary);
        println!("OK");
    }
}
