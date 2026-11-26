use std::env::args;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write; 
use std::path::Path;
use std::process::Command;

fn main(){
    let argv: Vec<String> = args().collect();
    let argc = argv.len();

    if argc < 2{
        println!("No input file to compile provided!");
        return;
    }

    //Handles input file path and extracts file name.
    let prog_path = Path::new(&argv[1]);
    let prog_file = match prog_path.file_name(){
        Some(f) => f,
        None => {
            println!("Failed to extract file name from input path {}", prog_path.display());
            return;
        },
    };
    
    //Extracts program name from given program file.
    let prog_file_str = prog_file.display().to_string();
    let program_name: String = prog_file_str.split(".").next().unwrap().to_string();
    println!("GAVE ME PROGRAM: {} from file {}", program_name, prog_file_str);

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

    //Turns ll file into o file using llc
    let stat = Command::new("llc")
        .args(["-filetype=obj", "bogus.ll", "-o", "bogus.o"])
        .status()
        .expect("FAILED TO RUN llc COMMAND");
    if !stat.success(){
        println!("Failed to turn to machine code because {}", stat);
    }

    //Turns ll file into o file using llc
    let stat2 = Command::new("clang")
        .args(["bogus.o", "-o", "bogus"])
        .status()
        .expect("FAILED TO RUN clang LINKING COMMAND");
    if !stat2.success(){
        println!("Failed to turn to machine code because {}", stat2);
    }
}
