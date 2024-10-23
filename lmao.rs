//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.3.20

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

impl Copy for IntSigned {}

impl Clone for IntSigned{
    fn clone(&self) -> IntSigned{
        *self
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

impl Copy for IntUnsigned {}

impl Clone for IntUnsigned{
    fn clone(&self) -> IntUnsigned{
        *self
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

impl Clone for Value{
    fn clone(&self) -> Value{
        match self{
            Value::Int(i) => Value::Int(*i),
            Value::UInt(i) => Value::UInt(*i),
            Value::Float32(f) => Value::Float32(*f),
            Value::Float64(f) => Value::Float64(*f),
            Value::Char(c) => Value::Char(*c),
            Value::Boolean(b) => Value::Boolean(*b),
            Value::String(st) => Value::String((st).clone()),
            Value::StringBox(sb) => Value::StringBox(*sb),
            Value::List(l) => Value::List((l).clone()),
            Value::ListBox(bn) => Value::ListBox(*bn),
            Value::Object(o) => Value::Object(o.clone()),
            Value::ObjectBox(bn) => Value::ObjectBox(*bn),
            Value::MiscBox(bn) => Value::MiscBox(*bn),
            Value::NULLBox => Value::NULLBox,
        }
    }
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
            Value::Float32(flt32) => {
                if flt32.abs() > 9999999999999999.0{
                    write!(f, "f32 {:e}", flt32)
                }else{
                    write!(f, "f32 {}", flt32)
                }
            },
            Value::Float64(flt64) => {
                if flt64.abs() > 9999999999999999.0{
                    write!(f, "f64 {:e}", flt64)
                }else{
                    write!(f, "f64 {}", flt64)
                }
            },
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

type OpFunc = fn(&mut State) -> Result<(), String>;

//Main mutable state
struct State{
    stack: Vec<Value>,
    fns: HashMap<String, ASTNode>,
    vars: HashMap<String, Value>,
    frames: Vec<HashMap<String, Value>>,
    heap: Vec<(Value, bool)>,
    free_list: Vec<usize>,
    ops: HashMap<String, OpFunc>
}

fn type_to_string(v: &Value) -> String{
    let type_str: &str = match v{
        Value::Int(IntSigned::Int8(_)) => "i8",
        Value::Int(IntSigned::Int16(_)) => "i16",
        Value::Int(IntSigned::Int32(_)) => "i32",
        Value::Int(IntSigned::Int64(_)) => "i64",
        Value::Int(IntSigned::Int128(_)) => "i128",
        Value::Int(IntSigned::IntSize(_)) => "isize",
        
        Value::UInt(IntUnsigned::UInt8(_)) => "u8",
        Value::UInt(IntUnsigned::UInt16(_)) => "u16",
        Value::UInt(IntUnsigned::UInt32(_)) => "u32",
        Value::UInt(IntUnsigned::UInt64(_)) => "u64",
        Value::UInt(IntUnsigned::UInt128(_)) => "u128",
        Value::UInt(IntUnsigned::UIntSize(_)) => "usize",

        Value::Float32(_) => "f32",
        Value::Float64(_) => "f64",

        Value::Char(_) => "Char",
        Value::Boolean(_) => "Boolean",
        Value::String(_) => "String",
        Value::StringBox(_) => "StringBox",
        Value::List(_) => "List",
        Value::ListBox(_) => "ListBox",
        Value::Object(_) => "Object",
        Value::ObjectBox(_) => "ObjectBox",
        Value::MiscBox(_) => "MiscBox",
        Value::NULLBox => "NULLBox",
    };

    type_str.to_string()
}

//Used for numerical operators like +, -, *, etc.
fn numerical_type_error_string(op_type: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Operand types must match and be numeric types! Attempted values: {} and {}", op_type, v1, v2)
}

fn needs_n_args_only_n_provided(op_type: &str, args_needed: &str, args_provided: &str) -> String{
    let plural_s: &str;
    if args_needed == "One"{
        plural_s = "";
    }else{
        plural_s = "s";
    }
    format!("Operator ({}) error! {} operand{} required on stack; {} provided!", op_type, args_needed, plural_s, args_provided)
}

fn should_never_get_here_for_func(func: &str) -> String{
    format!("Should never get here for {} function!", func)
}

//Adds two values of matching numerical types together, pusing the result to the stack.
fn add(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Int(IntSigned::IntSize(a.wrapping_add(b))))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::UInt(IntUnsigned::UIntSize(a.wrapping_add(b))))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Int(IntSigned::Int8(a.wrapping_add(b))))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Int(IntSigned::Int16(a.wrapping_add(b))))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Int(IntSigned::Int32(a.wrapping_add(b))))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Int(IntSigned::Int64(a.wrapping_add(b))))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Int(IntSigned::Int128(a.wrapping_add(b))))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt8(a.wrapping_add(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt16(a.wrapping_add(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt32(a.wrapping_add(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt64(a.wrapping_add(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt128(a.wrapping_add(b))))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Float32(a + b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Float64(a + b))
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("+", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("+", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("+", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("add")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Subtracts two values of matching numerical types, pusing the result to the stack.
fn sub(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Int(IntSigned::IntSize(a.wrapping_sub(b))))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::UInt(IntUnsigned::UIntSize(a.wrapping_sub(b))))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Int(IntSigned::Int8(a.wrapping_sub(b))))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Int(IntSigned::Int16(a.wrapping_sub(b))))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Int(IntSigned::Int32(a.wrapping_sub(b))))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Int(IntSigned::Int64(a.wrapping_sub(b))))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Int(IntSigned::Int128(a.wrapping_sub(b))))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt8(a.wrapping_sub(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt16(a.wrapping_sub(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt32(a.wrapping_sub(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt64(a.wrapping_sub(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt128(a.wrapping_sub(b))))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Float32(a - b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Float64(a - b))
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("-", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("-", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("-", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("sub")),

    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Pops two items from top of stack and multiplies them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands.
fn mult(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Int(IntSigned::IntSize(a.wrapping_mul(b))))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::UInt(IntUnsigned::UIntSize(a.wrapping_mul(b))))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Int(IntSigned::Int8(a.wrapping_mul(b))))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Int(IntSigned::Int16(a.wrapping_mul(b))))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Int(IntSigned::Int32(a.wrapping_mul(b))))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Int(IntSigned::Int64(a.wrapping_mul(b))))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Int(IntSigned::Int128(a.wrapping_mul(b))))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt8(a.wrapping_mul(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt16(a.wrapping_mul(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt32(a.wrapping_mul(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt64(a.wrapping_mul(b))))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::UInt(IntUnsigned::UInt128(a.wrapping_mul(b))))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Float32(a * b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Float64(a * b))
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("*", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("*", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("*", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("mult")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

fn division_by_zero_error(t: &str) -> String{
    format!("Operator (/) error! Division by zero occuring between two operands of type {}!", t)
}

//Pops two items from top of stack and divides them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as division by zero.
fn div(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::IntSize(a / b)))
            }else{
                Err(division_by_zero_error("isize"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UIntSize(a / b)))
            }else{
                Err(division_by_zero_error("usize"))
            }
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int8(a / b)))
            }else{
                Err(division_by_zero_error("i8"))
            }
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int16(a / b)))
            }else{
                Err(division_by_zero_error("i16"))
            }
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int32(a / b)))
            }else{
                Err(division_by_zero_error("i32"))
            }
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int64(a / b)))
            }else{
                Err(division_by_zero_error("i64"))
            }
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int128(a / b)))
            }else{
                Err(division_by_zero_error("i128"))
            }
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt8(a / b)))
            }else{
                Err(division_by_zero_error("u8"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt16(a / b)))
            }else{
                Err(division_by_zero_error("u16"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt32(a / b)))
            }else{
                Err(division_by_zero_error("u32"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt64(a / b)))
            }else{
                Err(division_by_zero_error("u64"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt128(a / b)))
            }else{
                Err(division_by_zero_error("u128"))
            }
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            if b != 0.0{
                Ok(Value::Float32(a / b))
            }else{
                Err(division_by_zero_error("f32"))
            }
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            if b != 0.0{
                Ok(Value::Float64(a / b))
            }else{
                Err(division_by_zero_error("f64"))
            }
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("/", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("/", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("/", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("div")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Creates string for error return in modulo function.
fn modulo_by_zero_error(t: &str) -> String{
    format!("Operator (%) error! Modulo by zero occuring between two operands of type {}!", t)
}

//Pops two items from top of stack and modulos them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as modulo by zero.
fn modulo(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::IntSize(a % b)))
            }else{
                Err(modulo_by_zero_error("isize"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UIntSize(a % b)))
            }else{
                Err(modulo_by_zero_error("usize"))
            }
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int8(a % b)))
            }else{
                Err(modulo_by_zero_error("i8"))
            }
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int16(a % b)))
            }else{
                Err(modulo_by_zero_error("i16"))
            }
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int32(a % b)))
            }else{
                Err(modulo_by_zero_error("i32"))
            }
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int64(a % b)))
            }else{
                Err(modulo_by_zero_error("i64"))
            }
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            if b != 0{
                Ok(Value::Int(IntSigned::Int128(a % b)))
            }else{
                Err(modulo_by_zero_error("i128"))
            }
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt8(a % b)))
            }else{
                Err(modulo_by_zero_error("u8"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt16(a % b)))
            }else{
                Err(modulo_by_zero_error("u16"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt32(a % b)))
            }else{
                Err(modulo_by_zero_error("u32"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt64(a % b)))
            }else{
                Err(modulo_by_zero_error("u64"))
            }
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            if b != 0{
                Ok(Value::UInt(IntUnsigned::UInt128(a % b)))
            }else{
                Err(modulo_by_zero_error("u128"))
            }
        },

        (Some(a), Some(b)) => {
            Err(format!("Operator (%) error! Modulo operation requires two operands with \
                a singular matching type that is an integer type! Attempted values: {} and {}", a, b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("%", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("%", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("modulo")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Adds two values of matching numerical types together, pusing the result to the stack.
fn power(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Float32(a.powf(b)))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Float64(a.powf(b)))
        },

        (Some(a), Some(b)) => {
            Err(format!("Operator (pow) error. Exponential operation requires two operands with a singular \
                matching type that is either f32 or f64! Attempted values: {} and {}", a, b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("pow", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("pow", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("power")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Swaps the top two items on the stack, errors out of inusfficient items exist.
fn swap(s: &mut State) -> Result<(), String>{
    match s.pop2(){
        (Some(a), Some(b)) => {
            s.push(b);
            s.push(a);
            Ok(())
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("swap", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("swap", "Two", "none")),
        _ => Err(should_never_get_here_for_func("swap")),
    }
}

//Removes the top item from the stack 
// or errors out if stack is empty.
fn drop(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(_) => Ok(()),
        None => Err(needs_n_args_only_n_provided("drop", "One", "none")),
    }
}

//Clears existing stack to be empty. 
// This can be useful if you want a clean stack without doing a ton of drops.
fn drop_stack(s: &mut State) -> Result<(), String>{
    s.stack.clear();
    Ok(())
}

//Rotates top three items on stack, 
// putting the top item below the previous two.
fn rot(s: &mut State) -> Result<(), String>{
    match s.pop3(){
        (Some(a), Some(b), Some(c)) => {
            s.push(c);
            s.push(a);
            s.push(b);

            Ok(())
        },
        (None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided("rot", "Three", "only two")),
        (None, None, Some(_)) => Err(needs_n_args_only_n_provided("rot", "Three", "only one")),
        (None, None, None) => Err(needs_n_args_only_n_provided("rot", "Three", "none")),
        _ => Err(should_never_get_here_for_func("rot")),
    }
}

//Very literally just copies the top element of the stack and pushes it. 
// If it's a box, the box itself is copied, not the data it contains.
fn dup(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(v) => {
            s.push(v.clone());
            s.push(v.clone());
            Ok(())
        },
        None => Err(needs_n_args_only_n_provided("dup", "One", "none")),
    }
}

//Creates an error string used in the deep_dup function to avoid some code duplication.
fn error_for_deep_dup_due_to_bad_box(box_type: &str, disp_value: Value) -> String{
    format!("Operator (deepDup) error. Deep duplication of {} failed \
        because it's an invalid {} number due to being been free'd!", disp_value, box_type)
}

//Works like dup but duplicates the data held by box types 
// and creates a new box to hold the duplicated data.
fn deep_dup(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                let dupped_string = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_string);
                s.push(Value::StringBox(bn));
                s.push(Value::StringBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("StringBox", Value::StringBox(bn)))
            }
        },
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                let dupped_list = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_list);
                s.push(Value::ListBox(bn));
                s.push(Value::ListBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("ListBox", Value::ListBox(bn)))
            }
        },
        Some(Value::ObjectBox(bn)) => {
            if s.validate_box(bn){
                let dupped_obj = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_obj);
                s.push(Value::ObjectBox(bn));
                s.push(Value::ObjectBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("ObjectBox", Value::ObjectBox(bn)))
            }
        },
        Some(Value::MiscBox(bn)) => {
            if s.validate_box(bn){
                let dupped_data = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_data);
                s.push(Value::MiscBox(bn));
                s.push(Value::MiscBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("MiscBox", Value::MiscBox(bn)))
            }
        },
        Some(v) => {
            s.push(v.clone());
            s.push(v.clone());
            Ok(())
        },
        None => Err(needs_n_args_only_n_provided("dup", "One", "none")),
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

//Used in == and != operators to generate an error.
fn equality_error(op_type: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Comparisons of equality and inequality \
        must have matching types! Attempted values: {} and {}", op_type, v1, v2)
}

//Checks for equality between two data types. For boxes it checks to see 
// if the box numbers are equal and for NULL box it checks for self-equality.
//Consumes both items from stack and pushes resulting boolean based on their comparison.
fn is_equal(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::Char(a)), Some(Value::Char(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::StringBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::StringBox(_))) => Ok(Value::Boolean(false)),

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::ListBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::ListBox(_))) => Ok(Value::Boolean(false)),

        (Some(Value::ObjectBox(a)), Some(Value::ObjectBox(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::ObjectBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::ObjectBox(_))) => Ok(Value::Boolean(false)),

        (Some(Value::MiscBox(a)), Some(Value::MiscBox(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::MiscBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::MiscBox(_))) => Ok(Value::Boolean(false)),

        (Some(Value::NULLBox), Some(Value::NULLBox)) => Ok(Value::Boolean(true)),

        (Some(a), Some(b)) => {
            Err(equality_error("==", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("==", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("==", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("is_equal")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Checks for inequality between two data types. For boxes it checks to see 
// if the box numbers are equal and for NULL box it checks for self-inequality.
//Consumes both items from stack and pushes resulting boolean based on their comparison.
fn is_not_equal(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::Char(a)), Some(Value::Char(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::StringBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::StringBox(_))) => Ok(Value::Boolean(true)),

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::ListBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::ListBox(_))) => Ok(Value::Boolean(true)),

        (Some(Value::ObjectBox(a)), Some(Value::ObjectBox(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::ObjectBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::ObjectBox(_))) => Ok(Value::Boolean(true)),

        (Some(Value::MiscBox(a)), Some(Value::MiscBox(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::MiscBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::MiscBox(_))) => Ok(Value::Boolean(true)),

        (Some(Value::NULLBox), Some(Value::NULLBox)) => Ok(Value::Boolean(false)),

        (Some(a), Some(b)) => {
            Err(equality_error("!=", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("!=", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("!=", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("is_not_equal")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

fn comparison_error(op_type: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Non-equality comparison operators need matching \
        non-null types to function! Attempted values: {} and {}", op_type, v1, v2)
}

//Compares two values on stack to see if the second to top is greater than the top.
fn is_greater_than(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::Char(a)), Some(Value::Char(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::ObjectBox(a)), Some(Value::ObjectBox(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::MiscBox(a)), Some(Value::MiscBox(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(a), Some(b)) => {
            Err(comparison_error(">", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided(">", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided(">", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("is_greater_than")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Compares two values on stack to see if the second to top is less than the top.
fn is_less_than(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::Char(a)), Some(Value::Char(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::ObjectBox(a)), Some(Value::ObjectBox(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::MiscBox(a)), Some(Value::MiscBox(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(a), Some(b)) => {
            Err(comparison_error("<", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("<", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("<", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("is_less_than")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Compares two values on stack to see if the second to top is greater than or equal to the top.
fn is_greater_than_equal_to(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::Char(a)), Some(Value::Char(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::ObjectBox(a)), Some(Value::ObjectBox(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::MiscBox(a)), Some(Value::MiscBox(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(a), Some(b)) => {
            Err(comparison_error(">=", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided(">=", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided(">=", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("is_greater_than_equal_to")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

//Compares two values on stack to see if the second to top is less than or equal to the top.
fn is_less_than_equal_to(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Int(IntSigned::IntSize(a))), Some(Value::Int(IntSigned::IntSize(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt(IntUnsigned::UIntSize(a))), Some(Value::UInt(IntUnsigned::UIntSize(b)))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::Int(IntSigned::Int8(a))), Some(Value::Int(IntSigned::Int8(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int(IntSigned::Int16(a))), Some(Value::Int(IntSigned::Int16(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int(IntSigned::Int32(a))), Some(Value::Int(IntSigned::Int32(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int(IntSigned::Int64(a))), Some(Value::Int(IntSigned::Int64(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int(IntSigned::Int128(a))), Some(Value::Int(IntSigned::Int128(b)))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::UInt(IntUnsigned::UInt8(a))), Some(Value::UInt(IntUnsigned::UInt8(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt16(a))), Some(Value::UInt(IntUnsigned::UInt16(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt32(a))), Some(Value::UInt(IntUnsigned::UInt32(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt64(a))), Some(Value::UInt(IntUnsigned::UInt64(b)))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt(IntUnsigned::UInt128(a))), Some(Value::UInt(IntUnsigned::UInt128(b)))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::Float32(a)), Some(Value::Float32(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Float64(a)), Some(Value::Float64(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::Char(a)), Some(Value::Char(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::ObjectBox(a)), Some(Value::ObjectBox(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::MiscBox(a)), Some(Value::MiscBox(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(a), Some(b)) => {
            Err(comparison_error("<=", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("<=", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("<=", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("is_less_than_equal_to")),
    };

    match res {
        Ok(v) => {
            s.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
    
}

impl State{
    //Creates a new state.
    fn new() -> Self{
        //Creates lookup table for operator functions.
        let mut ops_map: HashMap<String, OpFunc> = HashMap::new();
        //Basic math operators.
        ops_map.insert("+".to_string(), add);
        ops_map.insert("-".to_string(), sub);
        ops_map.insert("*".to_string(), mult);
        ops_map.insert("/".to_string(), div);
        ops_map.insert("%".to_string(), modulo);
        ops_map.insert("pow".to_string(), power);

        //Stack operators.
        ops_map.insert("swap".to_string(), swap);
        ops_map.insert("drop".to_string(), drop);
        ops_map.insert("dropStack".to_string(), drop_stack);
        ops_map.insert("rot".to_string(), rot);
        ops_map.insert("dup".to_string(), dup);
        ops_map.insert("deepDup".to_string(), deep_dup);

        //Comparison operators.
        ops_map.insert("==".to_string(), is_equal);
        ops_map.insert("!=".to_string(), is_not_equal);
        ops_map.insert(">".to_string(), is_greater_than);
        ops_map.insert("<".to_string(), is_less_than);
        ops_map.insert(">=".to_string(), is_greater_than_equal_to);
        ops_map.insert("<=".to_string(), is_less_than_equal_to);

        State {
            stack: Vec::new(),
            fns: HashMap::new(),
            vars: HashMap::new(),
            frames: vec![HashMap::new()],
            heap: Vec::new(),
            free_list: Vec::new(),
            ops: ops_map 
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

    //Returns a boolean based on whether or not the desired box number is valid.
    fn validate_box(&self, box_num: usize) -> bool{
        box_num >= 0 && box_num < self.heap.len() && self.heap[box_num].1
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

    fn pop(&mut self) -> Option<Value>{
        self.stack.pop()
    }

    fn pop2(&mut self) -> (Option<Value>, Option<Value>){
        let top = self.pop();
        let second_to_top = self.pop();
        (second_to_top, top)
    }

    fn pop3(&mut self) -> (Option<Value>, Option<Value>, Option<Value>){
        let top = self.pop();
        let second_to_top = self.pop();
        let third_to_top = self.pop();
        (third_to_top, second_to_top, top)
    }

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
            ref t if (t.chars().next().unwrap() == '-' && t.len() > 1) 
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

//DELETE LATER
// //The various types of nodes that are part of the Abstract Syntax Tree
// enum ASTNode{
//     Terminal(Token),
//     If {if_true: Box<ASTNode>, if_false: Box<ASTNode>},
//     While(Box<ASTNode>),
//     Expression(Vec<ASTNode>),
//     Function{func_cmd: String, func_name: String, func_bod: Box<ASTNode>},
//     Variable{var_name: String, var_cmd: String},
//     LocVar{name: String, cmd: String},
//     BoxOp(String)
// }

//Iterates recursively through the AST and effectively runs the program doing so.
fn run_program(ast: &ASTNode, state: &mut State) -> Result<(), String>{
    match ast{
        ASTNode::Expression(nodes) => {
            for node in nodes.iter(){
                match node{
                    ASTNode::Terminal(Token::V(v)) => state.push((*v).clone()),
                    ASTNode::Terminal(Token::Word(ref op)) => {
                        match state.ops.get(op){
                            Some(func) => {
                                match func(state){
                                    Ok(_) => {},
                                    Err(e) => return Err(e),
                                }
                            },
                            None => {
                                return Err(format!("Unrecognized Operator: {}", op));
                            },
                        } 
                    },
                    _ => {},
                }
            }
        },
        _ => {return Err("Should never get to this point!".to_string());},
    }

    Ok(())
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

    println!("{}\n\n\n\n", ast);

    let mut state = State::new();

    let result = run_program(&ast, &mut state);

    match result{
        Ok(_) => println!("The program completed successfully!"),
        Err(e) => println!("The program failed with error: {}", e),
    }

    //TEMPORARY DEBUG STACK PRINTING FOR DEVELOPMENT PURPOSES. WILL BE DELETED LATER
    println!("STACK START");
    for el in state.stack.iter(){
        println!("{}", el);
    }
    println!("STACK END");

}
