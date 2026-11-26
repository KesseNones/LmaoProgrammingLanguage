use std::env::args;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write; 
use std::path::Path;
use std::process::Command;
mod parser;
use parser::*;

//Takes in an AST and emits the LLVM instructions 
//by appending to instructions reference.
fn traverse_ast_and_emit(
ast: &ASTNode, instructions: &mut Vec<String>, 
pc: &mut u32, stack: &mut Vec<u32>)
{
    match ast{
        //Necessary since outermost node is a file.
        ASTNode::File(data) => {
            traverse_ast_and_emit(&data.program, instructions, pc, stack)
        },
        ASTNode::Expression(nodes) => {
            for node in nodes.iter(){
                match node{
                    ASTNode::Val(Value::Int32(i)) => {
                        let alloc = format!("%sp{} = alloca i32, align 4", pc);
                        let store = format!("store i32 {}, ptr %sp{}, align 4", i, pc);
                        instructions.push(alloc);
                        instructions.push(store);
                        stack.push(*pc);
                    },
                    ASTNode::Sop(StackOperator::Add) => {
                        let top = stack.pop().unwrap();
                        let snd = stack.pop().unwrap();

                        let top_str = format!("%sp{}", top);
                        let snd_str = format!("%sp{}", snd);

                        let middle = " load i32, ptr ";
                        let end = ", align 4";

                        let load_a = format!("%a{} = {}{}{}", pc, middle, top_str, end);
                        let load_b = format!("%b{} = {}{}{}", pc, middle, snd_str, end);
                        instructions.push(load_a);
                        instructions.push(load_b);

                        let sum = format!("%res{} = add i32 %a{}, %b{}", pc, pc, pc);
                        instructions.push(sum);
                        
                        let alloc = format!("%sp{} = alloca i32, align 4", *pc + 1);
                        let store = 
                            format!("store i32 {}{}, ptr %sp{}, align 4", 
                                "%res", pc, *pc + 1);
                        instructions.push(alloc);
                        instructions.push(store);
                        stack.push(*pc + 1);
                        *pc += 1;
                    },
                    _ => (),
                } 
                *pc += 1;
            }
        },
        _ => (),
    }   
}

//Takes in an input file path from args and tries to compile the input program.
// Returns an error string if there's an error and None if it's all fine.
fn compile(argv: &Vec<String>, prog_path_str: &str) -> Option<String>{
    //Handles input file path and extracts file name.
    let prog_path = Path::new(prog_path_str);
    let prog_file = match prog_path.file_name(){
        Some(f) => f,
        None => {
            return Some(format!("Failed to extract file name \
                from input path {}", prog_path.display()));
        },
    };
    
    //Extracts program name from given program file.
    let prog_file_str = prog_file.display().to_string();
    let program_name: String = prog_file_str.split(".").next().unwrap().to_string();

    let mut input_file = match OpenOptions::new()
        .read(true)
        .open(prog_path)
    {
        Ok(f) => f,
        Err(e) => {
            return Some(format!("Unable to open {} because {}", prog_file_str, e));
        }
    };
    let mut prog_str: String = String::new();
    if let Err(e) = input_file.read_to_string(&mut prog_str){
        return Some(format!("Unable to open {} because {}", prog_file_str, e));
    }

    //Tries to parse program string into ast.
    //Errors if it failed.
    let ast = match parse_string_to_ast(argv, prog_str){
        Ok(ast) => ast,
        Err(e) => return Some(format!("{}", e)),
    };

    let mut instructions: Vec<String> = Vec::new();
    let mut pc: u32 = 0;
    let mut stack: Vec<u32> = Vec::new();
    traverse_ast_and_emit(&ast, &mut instructions, &mut pc, &mut stack);

    //Start and end of program IR.
    let start = "define i32 @main() {\nentry:\n";

    if stack.len() > 0{
        instructions.push(format!("%end = load i32, ptr %sp{}, align 4", 
                stack.pop().unwrap()));
        instructions.push(format!("ret i32 %end"));
    }else{
        instructions.push(format!("ret i32 0"));
    }

    let end = "\n}\n";

    //Builds program string.
    let prog_str = format!("{}{}{}", start, instructions.join("\n"), end);
    println!("{}", prog_str);

    let llvm_name = format!("{}.ll", program_name);
    let llvm_path = Path::new(&llvm_name);
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(llvm_path)
    {
        Ok(f) => f,
        Err(e) => {
            return Some(format!("Failed to open {} for writing because {}", 
                    llvm_name, e)); 
        },
    };

    match file.write_all(prog_str.as_bytes()){
        Ok(_) => (),
        Err(e) => {
            println!("FAILED TO WRITE!");
            return Some(format!("Failed to write {} because {}", llvm_name, e));
        }
    } 

    //Turns ll file into o file using llc
    let obj_name = format!("{}.o", program_name);
    let res = Command::new("llc")
        .args(["-filetype=obj", &llvm_name, "-o", &obj_name])
        .status();
    let stat = match res{
        Ok(r) => r,
        Err(e) => {
            return Some(format!("Failed to run llc command because {}", e));
        },
    };
    if !stat.success(){
        return Some(format!("Failed to turn to machine code because {}", stat));
    }

    //Turns ll file into o file using llc
    let res2 = Command::new("clang")
        .args([&obj_name, "-o", &program_name])
        .status();
    let stat2 = match res2{
        Ok(r) => r,
        Err(e) => {
            return Some(format!("Failed to run clang command because {}", e));
        },
    };
    if !stat2.success(){
        return Some(format!("Failed to turn to machine code because {}", stat2));
    }
     
    None
}

fn main(){
    let argv: Vec<String> = args().collect();
    let argc = argv.len();

    if argc < 2{
        println!("No input file to compile provided!");
        return;
    }

    match compile(&argv, &argv[1]){
        Some(e) => println!("{}", e),
        None => ()
    }
}
