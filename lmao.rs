//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.1.4

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

enum IntSigned{
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntSize(isize)
}

enum IntUnsigned{
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    UIntSize(usize)
}

//This enum is used to contain all the possible data types of Lmao.
enum Value{
    //General integer type from type inference
    Integer(isize),
    //Specific signed integers found from type declarations. (coming soonTM)
    Int(IntSigned),
    //Speficic unsigned integers found from type declarations.
    UInt(IntUnsigned),
    //Inferred float type.
    Float(f32),
    //Specified float types
    Float32(f32),
    Float64(f64),
    Char(char),
    Boolean(bool),
    //String and its equivalent box to live on the stack.
    String(String),
    StringBox(usize),
    List(Vec<Value>),
    ListBox(usize),
    Object(HashMap<String, Value>),
    ObjectBox(usize),
    MiscBox(usize)
}

//Can either be a value to push to the stack or 
// a command to run an operator or something like that.
enum Token{
    V(Value),
    Cmd(String)
}

//The various types of nodes that are part of the Abstract Syntax Tree
enum ASTNode{
    Terminal(Token),
    If {if_true: Box<ASTNode>, if_false: Box<ASTNode>},
    While(Box<ASTNode>),
    Expression(Vec<ASTNode>),
    Function{func_cmd: String, func_name: String, func_bod: Box<ASTNode>},
    Variable{var_name: String, var_cmd: String},
    LocVar{name: String, cmd: String},
    BoxOp(String)
}

//Main mutable state
struct State{
    stack: Vec<Value>,
    fns: HashMap<String, ASTNode>,
    vars: HashMap<String, Value>,
    frames: Vec<HashMap<String, Value>>,
    heap: Vec<(Value, bool)>,
    free_list: Vec<usize>
}

fn main(){
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if argc < 2{
        panic!("Error! No program given in arguments for Lmao to run!");
    }

    let file_path = Path::new(&argv[1]);
    let file_name = file_path.display();

    let mut code_file = match File::open(&file_path){
        Ok(f) => f,
        Err(reason) => panic!("Unable to open Lmao file {} for parsing because {}", file_name, reason),
    };

    let mut file_string = String::new();
    match code_file.read_to_string(&mut file_string){
        Ok(_) => {},
        Err(reason) => panic!("Unable to read Lmao file {} because {}", file_name, reason),
    }

    println!("READ IN FILE CONTENTS:\n\n{}", file_string);

    let file_chars: Vec<char> = file_string.chars().collect();

    println!("LEN {}", file_chars.len());

    for c in file_chars.iter(){
        print!("{} ", c);
    }

}
