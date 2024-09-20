//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.1.10

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
    //Specific signed integers found from type declarations. (coming soonTM)
    Int(IntSigned),
    //Speficic unsigned integers found from type declarations.
    UInt(IntUnsigned),
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
    Word(String)
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

//Tokenizes list of chars into list of strings.
fn tokenize(chars: &Vec<char>) -> Vec<String>{
    let mut tokens: Vec<String> = Vec::new();
    let mut curr_token: Vec<char> = Vec::new();

    let mut in_string = false;
    let mut in_comment = false;

    let mut i: usize = 0;
    while i < chars.len(){
        match (chars[i], in_string, in_comment){
            //Char tokenization
            ('\'', false, false) => {
                if ((i + 2) < chars.len()) && (chars[i + 2] == '\''){
                    tokens.push(String::from(format!("\'{}\'", chars[i + 1])));
                    i += 3;
                }else if ((i + 3) < chars.len()) && (chars[i + 1] == '\\') && (chars[i + 3] == '\'') {
                    tokens.push(String::from(format!("\'\\{}\'", chars[i + 2])));
                    i += 4;
                }else{
                    panic!("Parse error! Char missing closing apostraphie!");
                }
            },
            //Start of string case.
            ('\"', false, false) => {
                curr_token.push(chars[i]);
                in_string = true;
                i += 1;

            },
            //Makes it so strings can have double quotes inside them, as long as they are escaped.
            ('\\', true, false) => {
                if ((i + 1) < chars.len()) && (chars[i + 1] == '\"'){
                    curr_token.push('\\');
                    curr_token.push('\"');
                    i += 2;
                }else{
                    curr_token.push('\\');
                    i += 1;
                }
            },
            //End of string case.
            ('\"', true, false) => {
                curr_token.push(chars[i]);
                tokens.push(curr_token.iter().collect());
                curr_token.clear();
                in_string = false;
                i += 1;
            },
            //In string case.
            (_, true, false) => {
                curr_token.push(chars[i]);
                i += 1;
            },
            //Comment entry case.
            ('/', false, false) => {
                if ((i + 1) < chars.len()) && (chars[i + 1] == '/'){
                    in_comment = true;
                    i += 2;
                }else{
                    curr_token.push(chars[i]);
                    i += 1;
                }
            },
            //Exit comment case.
            ('\n', false, true) => {
                in_comment = false;
                i += 1;
            },
            //In comment case.
            (_, false, true) => i += 1,
            //General parsing case.
            (c, false, false) => {
                if !c.is_whitespace(){
                    curr_token.push(c);
                }else{
                    if curr_token.len() > 0{
                        tokens.push(curr_token.iter().collect());
                        curr_token.clear();
                    }
                }

                i += 1;
            },
            _ => panic!("SHOULD NEVER GET HERE!!!!!!!"),
        }
    }

    if in_string{
        panic!("Parse error! String not ended with matching double quotation!");
    }

    tokens

}

fn throw_parse_error(t: &str, attempted_token: &String){
    panic!("Parse error! Incorrectly constructed {}! Tried: {}", t, attempted_token);
}

//Given reference to list of seperated tokens, 
// differentiates each one as either a value or word.
//WARNING! OWNERSHIP TRANSFERS SO, YOU BETTER WATCH OUT!
fn lex_tokens(tokens: Vec<String>) -> Vec<Token>{
    let mut lexed: Vec<Token> = Vec::new();

    for tok in tokens.into_iter(){
        match tok{
            //Boolean lexing cases.
            ref t if t == "True" || t == "true" => {
                lexed.push(Token::V(
                    Value::Boolean(true))
                );
            },
            ref t if t == "False" || t == "false" => {
                lexed.push(Token::V(
                    Value::Boolean(false))
                );
            },
            //String case.
            ref t if t.starts_with("\"") && t.ends_with("\"") => {
                lexed.push(Token::V(Value::String(tok[1..(tok.len() - 1)].to_string())));
            }, 
            //Char case.
            ref t if t.starts_with("\'") && t.ends_with("\'") => {
                lexed.push(Token::V(Value::Char(
                    tok[1..].chars().next().unwrap()))
                );
            },
            //List case.
            ref t if t == "[]" => lexed.push(Token::V(Value::List(Vec::new()))),
            //Object case.
            ref t if t == "{}" => lexed.push(Token::V(Value::Object(HashMap::new()))),
            //Float cases.
            ref t if t.ends_with("f32") => {
                match tok[0..(tok.len() - 3)].parse::<f32>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Float32(parsed))),
                    Err(_) => throw_parse_error("f32", &tok),
                }
            },
            ref t if t.ends_with("f64") => {
                match tok[0..(tok.len() - 3)].parse::<f64>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Float64(parsed))),
                    Err(_) => throw_parse_error("f64", &tok), 
                }
            },
            //Integer cases.
            ref t if t.ends_with("u8") => {
                match tok[0..(tok.len() - 2)].parse::<u8>(){
                    Ok(parsed) => lexed.push(Token::V(Value::UInt(IntUnsigned::UInt8(parsed)))),
                    Err(_) => throw_parse_error("u8", &tok), 
                }
            },
            ref t if t.ends_with("i8") => {
                match tok[0..(tok.len() - 2)].parse::<i8>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Int(IntSigned::Int8(parsed)))),
                    Err(_) => throw_parse_error("i8", &tok), 
                }
            },
            ref t if t.ends_with("u16") => {
                match tok[0..(tok.len() - 3)].parse::<u16>(){
                    Ok(parsed) => lexed.push(Token::V(Value::UInt(IntUnsigned::UInt16(parsed)))),
                    Err(_) => throw_parse_error("u16", &tok),
                }
            },
            ref t if t.ends_with("i16") => {
                match tok[0..(tok.len() - 3)].parse::<i16>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Int(IntSigned::Int16(parsed)))),
                    Err(_) => throw_parse_error("i16", &tok), 
                }
            },
            ref t if t.ends_with("u32") => {
                match tok[0..(tok.len() - 3)].parse::<u32>(){
                    Ok(parsed) => lexed.push(Token::V(Value::UInt(IntUnsigned::UInt32(parsed)))),
                    Err(_) => throw_parse_error("u32", &tok), 
                }
            },
            ref t if t.ends_with("i32") => {
                match tok[0..(tok.len() - 3)].parse::<i32>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Int(IntSigned::Int32(parsed)))),
                    Err(_) => throw_parse_error("i32", &tok), 
                }
            },
            ref t if t.ends_with("u64") => {
                match tok[0..(tok.len() - 3)].parse::<u64>(){
                    Ok(parsed) => lexed.push(Token::V(Value::UInt(IntUnsigned::UInt64(parsed)))),
                    Err(_) => throw_parse_error("u64", &tok), 
                }
            },
            ref t if t.ends_with("i64") => {
                match tok[0..(tok.len() - 3)].parse::<i64>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Int(IntSigned::Int64(parsed)))),
                    Err(_) => throw_parse_error("i64", &tok), 
                }
            },
            ref t if t.ends_with("u128") => {
                match tok[0..(tok.len() - 4)].parse::<u128>(){
                    Ok(parsed) => lexed.push(Token::V(Value::UInt(IntUnsigned::UInt128(parsed)))),
                    Err(_) => throw_parse_error("u128", &tok), 
                }
            },
            ref t if t.ends_with("i128") => {
                match tok[0..(tok.len() - 4)].parse::<i128>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Int(IntSigned::Int128(parsed)))),
                    Err(_) => throw_parse_error("i128", &tok), 
                }
            },
            ref t if t.ends_with("usize") => {
                match tok[0..(tok.len() - 5)].parse::<usize>(){
                    Ok(parsed) => lexed.push(Token::V(Value::UInt(IntUnsigned::UIntSize(parsed)))),
                    Err(_) => throw_parse_error("usize", &tok), 
                }
            },
            ref t if t.ends_with("isize") => {
                match tok[0..(tok.len() - 5)].parse::<isize>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Int(IntSigned::IntSize(parsed)))),
                    Err(_) => throw_parse_error("isize", &tok), 
                }
            },

            //General catch-all case mostly meant for operators.
            _ => lexed.push(Token::Word(tok)),
        }
    }

    lexed
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

    let file_chars: Vec<char> = file_string.chars().collect();
    let tokens = tokenize(&file_chars);

    for tok in tokens.iter(){
        print!("{} ", tok);
    }
    println!("");

    let lexed = lex_tokens(tokens);

}
