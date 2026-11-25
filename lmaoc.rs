use std::env::args;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write; 
use std::path::Path;

fn main(){
    let argv: Vec<String> = args().collect();
    let argc = argv.len();

    let bogus = "
define i32 @main() {
    entry:
        ret i32 42
}
";

    let bogus_path = Path::new("./bogus.ll");
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(bogus_path)
    {
        Ok(f) => f,
        Err(e) => {
            println!("Stuff's borked!");
            return; 
        },
    };

    match file.write_all(bogus.as_bytes()){
        Ok(_) => (),
        Err(_) => {
            println!("FAILED TO WRITE!");
            return;
        }
    } 
}
