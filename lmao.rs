//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.2.2

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fmt;

#[derive(PartialEq, Eq)]
enum IntSigned{
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntSize(isize)
}

impl fmt::Display for IntSigned{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            IntSigned::Int8(n) => write!(f, "i8 {}", n),
            IntSigned::Int16(n) => write!(f, "i16 {}", n),
            IntSigned::Int32(n) => write!(f, "i32 {}", n),
            IntSigned::Int64(n) => write!(f, "i64 {}", n),
            IntSigned::Int128(n) => write!(f, "i128 {}", n),
            IntSigned::IntSize(n) => write!(f, "isize {}", n),
        }
    }
}

#[derive(PartialEq, Eq)]
enum IntUnsigned{
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    UIntSize(usize)
}

impl fmt::Display for IntUnsigned{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            IntUnsigned::UInt8(n) => write!(f, "u8 {}", n),
            IntUnsigned::UInt16(n) => write!(f, "u16 {}", n),
            IntUnsigned::UInt32(n) => write!(f, "u32 {}", n),
            IntUnsigned::UInt64(n) => write!(f, "u64 {}", n),
            IntUnsigned::UInt128(n) => write!(f, "u128 {}", n),
            IntUnsigned::UIntSize(n) => write!(f, "usize {}", n),
        }
    }
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
    MiscBox(usize),
    NULLBox,
}

impl PartialEq for Value{
    fn eq(&self, other: &Self) -> bool{
        match(self, other){
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::UInt(a), Value::UInt(b)) => a == b,
            (Value::Float32(a), Value::Float32(b)) => a == b,
            (Value::Float64(a), Value::Float64(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::StringBox(a), Value::StringBox(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::ListBox(a), Value::ListBox(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (Value::ObjectBox(a), Value::ObjectBox(b)) => a == b,
            (Value::MiscBox(a), Value::MiscBox(b)) => a == b,
            (Value::NULLBox, Value::NULLBox) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl fmt::Display for Value{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Value::Int(int) => write!(f, "{}", int),
            Value::UInt(uint) => write!(f, "{}", uint),
            Value::Float32(flt32) => write!(f, "f32 {}", flt32),
            Value::Float64(flt64) => write!(f, "f64 {}", flt64),
            Value::Char(c) => write!(f, "Char \'{}\'", c.escape_default().collect::<String>()),
            Value::Boolean(b) => write!(f, "Boolean {}", b),
            Value::String(s) => write!(f, "String \"{}\"", s),
            Value::StringBox(sb) => write!(f, "StringBox {}", sb),
            Value::List(_) => write!(f, "LIST [INSERT_CONTENTS_HERE]"), //Do some kind of actual printing here later!
            Value::ListBox(lb) => write!(f, "ListBox {}", lb),
            Value::Object(_) => write!(f, "Object OBJ"), //Do some kind of actual printing here later!
            Value::ObjectBox(ob) => write!(f, "ObjectBox {}", ob),
            Value::MiscBox(bn) => write!(f, "MiscBox {}", bn),
            Value::NULLBox => write!(f, "Box NULL"),
        }
    }
}

//Can either be a value to push to the stack or 
// a command to run an operator or something like that.
#[derive(PartialEq, Eq)]
enum Token{
    V(Value),
    Word(String)
}

impl Default for Token{
    fn default() -> Self{
        Token::V(Value::NULLBox)
    }
}

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Token::V(val) => write!(f, "{}", val),
            Token::Word(w) => write!(f, "Word {}", w),
        }
    }
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

impl Default for ASTNode{
    fn default() -> Self{
        ASTNode::Terminal(Token::V(Value::NULLBox))
    }
}

impl fmt::Display for ASTNode{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            ASTNode::Terminal(t) => write!(f, "{}", t),
            ASTNode::If{if_true, if_false} => write!(f, "If [true_branch: {}, false_branch: {}]", if_true, if_false),
            ASTNode::While(body) => write!(f, "While [{}]", body),
            ASTNode::Expression(vec) => {
                let strs: Vec<String> = vec.iter().map(|n| format!("{}", n)).collect();
                write!(f, "Expression [{}]", strs.join(", "))
            },
            ASTNode::Function{func_cmd: cmd, func_name: name, func_bod: body} => {
                write!(f, "Function [cmd: {}, name: {}, body: {}]", cmd, name, body)
            },
            ASTNode::Variable{var_name: name, var_cmd: cmd} => write!(f, "Variable [name: {}, cmd: {}]", name, cmd),
            ASTNode::LocVar{name: nm, cmd: c} => write!(f, "Local Variable [name: {}, cmd: {}]", nm, c),
            ASTNode::BoxOp(op) => write!(f, "BoxOp {}", op),
        }
    }
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

impl State{
    //Creates a new state.
    fn new() -> Self{
        State {
            stack: Vec::new(),
            fns: HashMap::new(),
            vars: HashMap::new(),
            frames: vec![HashMap::new()],
            heap: Vec::new(),
            free_list: Vec::new() 
        }
    }

    //Inserts an item into the heap, 
    // returning an index to where it was inserted in the heap.
    fn insert_to_heap(&mut self, ins_val: Value) -> usize{
        if self.free_list.len() > 0{
            let free_cell_num = self.free_list.pop().unwrap();
            self.heap[free_cell_num] = (ins_val, true);
            return free_cell_num;
        }else{
            self.heap.push((ins_val, true));
            return self.heap.len() - 1;
        }
    }

    //Pushes a value to the stack and accounts for if the value 
    // is a non-primitive type, allocating it on the heap if necessary.
    fn push(&mut self, ins_val: Value){
        match ins_val{
            Value::String(_) => {
                let box_num = self.insert_to_heap(ins_val);
                self.stack.push(Value::StringBox(box_num));
            },
            Value::List(_) => {
                let box_num = self.insert_to_heap(ins_val);
                self.stack.push(Value::ListBox(box_num));
            },
            Value::Object(_) => {
                let box_num = self.insert_to_heap(ins_val);
                self.stack.push(Value::ObjectBox(box_num));
            },
            anything => self.stack.push(anything),
        }
    }

}

//DELETE THIS LATER
// //This enum is used to contain all the possible data types of Lmao.
// enum Value{
//     //Specific signed integers found from type declarations. (coming soonTM)
//     Int(IntSigned),
//     //Speficic unsigned integers found from type declarations.
//     UInt(IntUnsigned),
//     //Specified float types
//     Float32(f32),
//     Float64(f64),
//     Char(char),
//     Boolean(bool),
//     //String and its equivalent box to live on the stack.
//     String(String),
//     StringBox(usize),
//     List(Vec<Value>),
//     ListBox(usize),
//     Object(HashMap<String, Value>),
//     ObjectBox(usize),
//     MiscBox(usize),
//     NULLBox,
// }

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
                let mut iter = tok[1..].chars();
                let mut captured: char = iter.nth(0).unwrap();
                if captured == '\\'{
                    captured = match iter.nth(0).unwrap(){
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '0' => '\0',
                        _ => captured,
                    };
                    println!("{}", captured);

                }
                lexed.push(Token::V(Value::Char(captured)));
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
            //Explicit integer cases for both signed and unsigned.
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
            //Type inference for float.
            ref t if t.contains(".") 
                    && (t.chars().next().unwrap() == '-' 
                    || (t.chars().next().unwrap() >= '0' 
                        && t.chars().next().unwrap() <= '9')) 
                    => {
                match tok.parse::<f32>(){
                    Ok(parsed) => lexed.push(Token::V(Value::Float32(parsed))),
                    Err(_) => throw_parse_error("f32", &tok),
                }
            },
            //Type inference for integer.
            ref t if t.chars().next().unwrap() == '-' 
                    || (t.chars().next().unwrap() >= '0' 
                        && t.chars().next().unwrap() <= '9') 
                    => {
                match tok.parse::<isize>(){
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

//This function does the heavy-lifting of recursively building the AST.
fn make_ast_prime(
    mut already_parsed: Vec<ASTNode>, 
    tokens: Vec<Token>, 
    token_index: usize, 
    terminators: Vec<Token>
) -> (Vec<ASTNode>, Vec<Token>, usize, Option<usize>){
    //If out of tokens to parse, end or throw error if there were terminators to look for.
    if token_index >= tokens.len(){
        if terminators.len() == 0{
            return (already_parsed, tokens, token_index, None)
        }else{
            let mut terms = String::new();
            for t in terminators.iter(){
                terms.push_str(&format!("{}, ", t));
            }
            panic!("Ended expression without finding one of: {}", terms);
        }
    //If still tokens to parse, converts the tokens into an ASTNode.
    }else{
        match tokens[token_index]{
            //Stop on terminator case.
            ref tok if terminators.contains(tok) => (already_parsed, tokens, token_index + 1, Some(token_index)),
            //Parse if statement case.
            Token::Word(ref cmd) if cmd == "if" => {
                let (true_branch, false_branch, tokens_prime, token_index_prime) = parse_if(tokens, token_index + 1);
                already_parsed.push(ASTNode::If{if_true : Box::new(true_branch), if_false : Box::new(false_branch)});
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, terminators) 
            },
            //While loop parsing case.
            Token::Word(ref cmd) if cmd == "while" => {
                let (loop_body, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, vec![Token::Word(";".to_string())]);
                already_parsed.push(ASTNode::While(Box::new(ASTNode::Expression(loop_body))));
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, terminators)
            },
            //Function case.
            Token::Word(ref cmd) if cmd == "func" => {
                //Makes sure there's enough stuff to look to parse the function.
                if token_index + 2 > tokens.len(){
                    panic!("Insufficient tokens left for function to be parsed!");
                }

                let mut toks = tokens;
                let command = std::mem::take(&mut toks[token_index + 1]);
                let name = std::mem::take(&mut toks[token_index + 2]);
                let (command_str, name_str) = match (command, name){
                    (Token::Word(c), Token::Word(n)) => (c, n),
                    (_, _) => panic!("SHOULD NEVER GET HERE!!!"),
                };

                let (fbod, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), toks, token_index + 3, vec![Token::Word(";".to_string())]);

                let fbod_ast = Box::new(ASTNode::Expression(fbod));

                already_parsed.push(
                    ASTNode::Function{func_cmd: command_str, func_name: name_str, func_bod: fbod_ast});
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, terminators)

            },
            //Var command parsing case.
            Token::Word(ref cmd) if cmd == "var" => {
                let (mut var_data, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, vec![Token::Word(";".to_string())]);
                if var_data.len() >= 2{
                    let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
                        (ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c, n),
                        (_, _) => {panic!("Malformed variable command Error! \
                            Insufficient parameters given for variable command!")},
                    };

                    already_parsed.push(ASTNode::Variable{var_name: name, var_cmd: cmd});
                    make_ast_prime(already_parsed, tokens_prime, token_index_prime, terminators)

                }else{
                    panic!("Malformed variable command Error! \
                        Insufficient parameters given for variable command!");
                }
            },
            //Loc command parsing case.
            Token::Word(ref cmd) if cmd == "loc" => {
                let (mut var_data, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, vec![Token::Word(";".to_string())]);
                if var_data.len() >= 2{
                    let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
                        (ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c, n),
                        (_, _) => {panic!("Malformed local variable command Error! \
                            Insufficient parameters given for local variable command!")},
                    };

                    already_parsed.push(ASTNode::LocVar{name: name, cmd: cmd});
                    make_ast_prime(already_parsed, tokens_prime, token_index_prime, terminators)

                }else{
                    panic!("Malformed local variable command Error! \
                        Insufficient parameters given for local variable command!");
                }
            },
            //Box command case.
            Token::Word(ref cmd) if cmd == "box" => {
                let (mut box_data, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, vec![Token::Word(";".to_string())]);
                if box_data.len() >= 1{
                    let box_cmd = match std::mem::take(&mut box_data[0]){
                        ASTNode::Terminal(Token::Word(c)) => c,
                        _ => panic!("Malformed box command!"),
                    };

                    already_parsed.push(ASTNode::BoxOp(box_cmd));
                    make_ast_prime(already_parsed, tokens_prime, token_index_prime, terminators)
                }else{
                    panic!("Malformed box command! No box command token given!")
                }
            },
            _ => {
                let mut toks = tokens;
                already_parsed.push(ASTNode::Terminal(std::mem::take(&mut toks[token_index])));
                make_ast_prime(already_parsed, toks, token_index + 1, terminators)
            },
        }
    }

}

fn parse_if(tokens: Vec<Token>, token_index: usize) -> (ASTNode, ASTNode, Vec<Token>, usize){
    let (true_branch, tokens_prime, token_index_prime, terminator_index) = 
        make_ast_prime(
            Vec::new(), 
            tokens, 
            token_index, 
            vec![Token::Word("else".to_string()), Token::Word(";".to_string())]
        );
    match terminator_index{
        Some(i) => {
            match tokens_prime[i]{
                Token::Word(ref cmd) if cmd == "else" => {
                    let (false_branch, tokens_prime_prime, token_index_prime_prime) = 
                        parse_else(tokens_prime, token_index_prime);
                    (ASTNode::Expression(true_branch), false_branch, 
                        tokens_prime_prime, token_index_prime_prime)
                },
                _ => (ASTNode::Expression(true_branch), ASTNode::Expression(vec![]), tokens_prime, token_index_prime),  
            }
        },
        _ => panic!("SHOULD NEVER GET HERE!!!"),
    }
}

fn parse_else(tokens: Vec<Token>, token_index: usize) -> (ASTNode, Vec<Token>, usize){
    let (if_false, tokens_prime, token_index_prime, _) = 
        make_ast_prime(
            Vec::new(),
            tokens, 
            token_index,
            vec![Token::Word(";".to_string())]
        );
    (ASTNode::Expression(if_false), tokens_prime, token_index_prime)
}

//Consumes a vec of tokens and generates an Abstract Syntax Tree (AST) from it,
// returning it for the program to then run.
fn make_ast(tokens: Vec<Token>) -> ASTNode{
    ASTNode::Expression(make_ast_prime(Vec::new(), tokens, 0, Vec::new()).0)
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
        print!("{} | ", tok);
    }
    println!("\n\n\n\n\n");

    let lexed = lex_tokens(tokens);

    for lxt in lexed.iter(){
        println!("{}", lxt);
    }
    println!("\n\n\n\n\n");

    let ast: ASTNode = make_ast(lexed);

    println!("{}", ast);

//DELETE THIS LATER
// //This enum is used to contain all the possible data types of Lmao.
// enum Value{
//     //Specific signed integers found from type declarations. (coming soonTM)
//     Int(IntSigned),
//     //Speficic unsigned integers found from type declarations.
//     UInt(IntUnsigned),
//     //Specified float types
//     Float32(f32),
//     Float64(f64),
//     Char(char),
//     Boolean(bool),
//     //String and its equivalent box to live on the stack.
//     String(String),
//     StringBox(usize),
//     List(Vec<Value>),
//     ListBox(usize),
//     Object(HashMap<String, Value>),
//     ObjectBox(usize),
//     MiscBox(usize),
//     NULLBox,
// }

    let mut state = State::new();
    state.push(Value::Int(IntSigned::Int16(999)));
    state.push(Value::String("Your mum gay lmao".to_string()));
    state.push(Value::Object(HashMap::new()));
    state.push(Value::List(Vec::new()));

    println!("STACK START");
    for el in state.stack.iter(){
        println!("{}", el);
    }
    println!("STACK END");

}
