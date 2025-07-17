//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.10.4

//LONG TERM: MAKE OPERATOR FUNCTIONS MORE SLICK USING GENERICS!

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write; 
use std::fmt;
use std::cmp::Ordering;
use std::convert::TryInto;
use fmt::Display;
use std::io;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

//This enum is used to contain all the possible data types of Lmao 
// that live everywhere but the Heap.
enum Value{
    //Signed integers.
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntSize(isize),

    //Unsigned integers.
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    UIntSize(usize),

    //Specified float types
    Float32(f32),
    Float64(f64),

    //Char and boolean.
    Char(char),
    Boolean(bool),

    //Used to reference items in the heap.
    StringBox(usize),
    ListBox(usize),
    ObjectBox(usize),
    MiscBox(usize),
    NULLBox,
}

//Used to contain values on the heap only.
enum HeapValue{
    String(String),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Primitive(Value),
}

//Exists in token lists and AST.
enum SuperValue{
    Reg(Value), 
    Heap(HeapValue),
}

impl PartialEq for SuperValue{
    fn eq(&self, other: &Self) -> bool{
        match(self, other){
            (SuperValue::Reg(r1), SuperValue::Reg(r2)) => r1 == r2,
            (SuperValue::Heap(h1), SuperValue::Heap(h2)) => h1 == h2,
            _ => false,
        }
    }
}

impl Eq for SuperValue {}

impl Clone for Value{
    fn clone(&self) -> Value{
        match self{
            Value::Int8(i) => Value::Int8(*i),
            Value::Int16(i) => Value::Int16(*i),
            Value::Int32(i) => Value::Int32(*i),
            Value::Int64(i) => Value::Int64(*i),
            Value::Int128(i) => Value::Int128(*i),
            Value::IntSize(i) => Value::IntSize(*i),

            Value::UInt8(i) => Value::UInt8(*i),
            Value::UInt16(i) => Value::UInt16(*i),
            Value::UInt32(i) => Value::UInt32(*i),
            Value::UInt64(i) => Value::UInt64(*i),
            Value::UInt128(i) => Value::UInt128(*i),
            Value::UIntSize(i) => Value::UIntSize(*i),

            Value::Float32(f) => Value::Float32(*f),
            Value::Float64(f) => Value::Float64(*f),
            Value::Char(c) => Value::Char(*c),
            Value::Boolean(b) => Value::Boolean(*b),
            Value::StringBox(sb) => Value::StringBox(*sb),
            Value::ListBox(bn) => Value::ListBox(*bn),
            Value::ObjectBox(bn) => Value::ObjectBox(*bn),
            Value::MiscBox(bn) => Value::MiscBox(*bn),
            Value::NULLBox => Value::NULLBox,
        }
    }
}

impl Clone for HeapValue{
    fn clone(&self) -> HeapValue{
        match self{
            HeapValue::String(st) => HeapValue::String((st).clone()),
            HeapValue::List(l) => HeapValue::List((l).clone()),
            HeapValue::Object(o) => HeapValue::Object(o.clone()),
            HeapValue::Primitive(p) => HeapValue::Primitive(p.clone()),
        }
    }
}

impl PartialEq for Value{
    fn eq(&self, other: &Self) -> bool{
        match(self, other){
            (Value::Int8(a), Value::Int8(b)) => a == b,
            (Value::Int16(a), Value::Int16(b)) => a == b,
            (Value::Int32(a), Value::Int32(b)) => a == b,
            (Value::Int64(a), Value::Int64(b)) => a == b,
            (Value::Int128(a), Value::Int128(b)) => a == b,
            (Value::IntSize(a), Value::IntSize(b)) => a == b,

            (Value::UInt8(a), Value::UInt8(b)) => a == b,
            (Value::UInt16(a), Value::UInt16(b)) => a == b,
            (Value::UInt32(a), Value::UInt32(b)) => a == b,
            (Value::UInt64(a), Value::UInt64(b)) => a == b,
            (Value::UInt128(a), Value::UInt128(b)) => a == b,
            (Value::UIntSize(a), Value::UIntSize(b)) => a == b,

            (Value::Float32(a), Value::Float32(b)) => a == b,
            (Value::Float64(a), Value::Float64(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::StringBox(a), Value::StringBox(b)) => a == b,
            (Value::ListBox(a), Value::ListBox(b)) => a == b,
            (Value::ObjectBox(a), Value::ObjectBox(b)) => a == b,
            (Value::MiscBox(a), Value::MiscBox(b)) => a == b,
            (Value::NULLBox, Value::NULLBox) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl PartialEq for HeapValue{
    fn eq(&self, other: &Self) -> bool{
        match(self, other){
            (HeapValue::String(a), HeapValue::String(b)) => a == b,
            (HeapValue::List(a), HeapValue::List(b)) => a == b,
            (HeapValue::Object(a), HeapValue::Object(b)) => a == b,
            (HeapValue::Primitive(a), HeapValue::Primitive(b)) => a == b, //MIGHT DESTROY THE UNIVERSE
            _ => false,
        }
    }
}

impl Eq for HeapValue {}

impl fmt::Display for Value{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Value::Int8(n) => write!(f, "i8 {}", n),
            Value::Int16(n) => write!(f, "i16 {}", n),
            Value::Int32(n) => write!(f, "i32 {}", n),
            Value::Int64(n) => write!(f, "i64 {}", n),
            Value::Int128(n) => write!(f, "i128 {}", n),
            Value::IntSize(n) => write!(f, "isize {}", n),

            Value::UInt8(n) => write!(f, "u8 {}", n),
            Value::UInt16(n) => write!(f, "u16 {}", n),
            Value::UInt32(n) => write!(f, "u32 {}", n),
            Value::UInt64(n) => write!(f, "u64 {}", n),
            Value::UInt128(n) => write!(f, "u128 {}", n),
            Value::UIntSize(n) => write!(f, "usize {}", n),

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
            Value::StringBox(sb) => write!(f, "StringBox {}", sb),
            Value::ListBox(lb) => write!(f, "ListBox {}", lb),
            Value::ObjectBox(ob) => write!(f, "ObjectBox {}", ob),
            Value::MiscBox(bn) => write!(f, "MiscBox {}", bn),
            Value::NULLBox => write!(f, "NULLBox"),
        }
    }
}

impl Default for Value{
    fn default() -> Self{
        Value::NULLBox
    }
}

impl fmt::Display for HeapValue{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            HeapValue::String(s) => write!(f, "String {:?}", s),
            HeapValue::List(ls) => {
                let ls_strs: Vec<String> = ls.iter().map(|el| format!("{}", el)).collect();
                write!(f, "List [{}]", ls_strs.join(", "))
            },
            HeapValue::Object(o) => {
                let mut obj_strs: Vec<String> = Vec::new();
                for (key, value) in o.iter(){
                    obj_strs.push(format!("{}: {}", key, value));
                }
                write!(f, "Object {}{}{}", "{", obj_strs.join(", "), "}")
            },
            HeapValue::Primitive(p) => write!(f, "{}", p),
        }
    }
}

impl Default for HeapValue{
    fn default() -> Self{
        HeapValue::Primitive(Value::NULLBox)
    }
}

impl fmt::Display for SuperValue{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            SuperValue::Reg(r) => write!(f, "{}", r),
            SuperValue::Heap(h) => write!(f, "{}", h),
        }
    }
}

//Can either be a value to push to the stack or 
// a command to run an operator or something like that.
#[derive(PartialEq, Eq)]
enum Token{
    V(SuperValue),
    Word((String, usize))
}

impl Default for Token{
    fn default() -> Self{
        Token::V(SuperValue::Reg(Value::NULLBox))
    }
}

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Token::V(val) => write!(f, "{}", val),
            Token::Word((w, _)) => write!(f, "Word {}", w),
        }
    }
}

//The various types of nodes that are part of the Abstract Syntax Tree
enum ASTNode{
    Terminal(Token),
    If {if_true: Box<ASTNode>, if_false: Box<ASTNode>},
    While(Box<ASTNode>),
    Expression(Vec<ASTNode>),
    Function{func_cmd: String, func_name: String, func_bod: Rc<ASTNode>},
    Variable{var_name: String, var_cmd: String, var_num: usize},
    LocVar{name: String, cmd: String, num: usize},
    BoxOp(String),
    AttErr{attempt: Box<ASTNode>, err: Box<ASTNode>},
    Defer(Rc<ASTNode>),
}

impl Default for ASTNode{
    fn default() -> Self{
        ASTNode::Terminal(Token::V(SuperValue::Reg(Value::NULLBox)))
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
            ASTNode::Variable{var_name: name, var_cmd: cmd, var_num: n} => write!(f, "Variable [name: {}, cmd: {}, num: {}]", name, cmd, n),
            ASTNode::LocVar{name: nm, cmd: c, num: n} => write!(f, "Local Variable [name: {}, cmd: {}, num: {}]", nm, c, n),
            ASTNode::BoxOp(op) => write!(f, "BoxOp {}", op),
            ASTNode::AttErr{attempt: att, err: e} => write!(f, "AttErr [attempt: {}, err: {}]", att, e),
            ASTNode::Defer(bod) => write!(f, "Defer [{}]", bod),
        }
    }
}

impl Clone for ASTNode{
    fn clone(&self) -> ASTNode{
        match self{
            ASTNode::Terminal(Token::V(SuperValue::Reg(r))) => ASTNode::Terminal(Token::V(SuperValue::Reg(r.clone()))),
            ASTNode::Terminal(Token::V(SuperValue::Heap(h))) => ASTNode::Terminal(Token::V(SuperValue::Heap(h.clone()))),
            ASTNode::Terminal(Token::Word(w)) => ASTNode::Terminal(Token::Word(w.clone())),
            ASTNode::If{if_true: true_branch, if_false: false_branch} => {
                ASTNode::If{if_true: Box::new(*true_branch.clone()), 
                    if_false: Box::new(*false_branch.clone())}
            },
            ASTNode::While(bod) => ASTNode::While(Box::new(*bod.clone())),
            ASTNode::Expression(nodes) => {
                let new_nodes: Vec<ASTNode> = nodes.iter().map(|n| n.clone()).collect();
                ASTNode::Expression(new_nodes)
            },
            ASTNode::Function{func_cmd: cmd, func_name: name, func_bod: bod} => {
                ASTNode::Function{func_cmd: cmd.clone(), func_name: name.clone(), 
                    func_bod: Rc::clone(&bod)}
            },
            ASTNode::Variable{var_name: name, var_cmd: cmd, var_num: n} => {
                ASTNode::Variable{var_name: name.clone(), var_cmd: cmd.clone(), var_num: *n}
            },
            ASTNode::LocVar{name: nam, cmd: c, num: n} => ASTNode::LocVar{name: nam.clone(), cmd: c.clone(), num: *n},
            ASTNode::BoxOp(op) => ASTNode::BoxOp(op.clone()),
            ASTNode::AttErr{attempt: att, err: e} => ASTNode::AttErr{attempt: att.clone(), err: e.clone()},
            ASTNode::Defer(bod) => ASTNode::Defer(Rc::clone(bod)),
        }
    }
}

type OpFunc = fn(&mut State) -> Result<(), String>;

//Main mutable state
struct State{
    stack: Vec<Value>,
    fns: HashMap<String, Rc<ASTNode>>,
    vars: Vec<(Value, bool)>,
    heap: Vec<(HeapValue, bool)>,
    free_list: Vec<usize>,
    ops: Vec<OpFunc>,
    frames: Vec<(usize, Vec<(Value, bool)>)>,
    curr_frame: usize,
    frame_pool: Vec<Vec<(Value, bool)>>,
    unique_var_name_count: usize,
    leaving_scope: bool,
    buffer: String,
}

//Uses the implemented format traits to build a string for the given Value type. 
// From there, it only consumes the characters that name the actual type.
// This avoids needing a big match statement. 
fn type_to_string(v: &Value) -> String{
    let mut chrs = String::new();
    for c in format!("{}", v).chars(){
        if c == ' '{
            break;
        }else{
            chrs.push(c);
        }
    }
    chrs
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

fn push_val_or_err(r: Result<Value, String>, s: &mut State) -> Result<(), String>{
    match r{
        Ok(v) => {
            s.stack.push(v);
            Ok(())
        },
        Err(e) => Err(e),
    }
}

//Adds two values of matching numerical types together, pusing the result to the stack.
fn add(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::IntSize(a.wrapping_add(b)))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::UIntSize(a.wrapping_add(b)))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Int8(a.wrapping_add(b)))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Int16(a.wrapping_add(b)))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Int32(a.wrapping_add(b)))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Int64(a.wrapping_add(b)))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Int128(a.wrapping_add(b)))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::UInt8(a.wrapping_add(b)))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::UInt16(a.wrapping_add(b)))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::UInt32(a.wrapping_add(b)))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::UInt64(a.wrapping_add(b)))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            Ok(Value::UInt128(a.wrapping_add(b)))
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

    push_val_or_err(res, s)
    
}

//Subtracts two values of matching numerical types, pusing the result to the stack.
fn sub(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::IntSize(a.wrapping_sub(b)))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::UIntSize(a.wrapping_sub(b)))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Int8(a.wrapping_sub(b)))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Int16(a.wrapping_sub(b)))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Int32(a.wrapping_sub(b)))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Int64(a.wrapping_sub(b)))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Int128(a.wrapping_sub(b)))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::UInt8(a.wrapping_sub(b)))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::UInt16(a.wrapping_sub(b)))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::UInt32(a.wrapping_sub(b)))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::UInt64(a.wrapping_sub(b)))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            Ok(Value::UInt128(a.wrapping_sub(b)))
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

    push_val_or_err(res, s)
    
}

//Pops two items from top of stack and multiplies them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands.
fn mult(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::IntSize(a.wrapping_mul(b)))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::UIntSize(a.wrapping_mul(b)))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Int8(a.wrapping_mul(b)))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Int16(a.wrapping_mul(b)))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Int32(a.wrapping_mul(b)))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Int64(a.wrapping_mul(b)))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Int128(a.wrapping_mul(b)))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::UInt8(a.wrapping_mul(b)))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::UInt16(a.wrapping_mul(b)))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::UInt32(a.wrapping_mul(b)))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::UInt64(a.wrapping_mul(b)))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            Ok(Value::UInt128(a.wrapping_mul(b)))
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

    push_val_or_err(res, s)
    
}

fn division_by_zero_error(t: &str) -> String{
    format!("Operator (/) error! Division by zero occuring between two operands of type {}!", t)
}

//Pops two items from top of stack and divides them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as division by zero.
fn div(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            if b != 0{
                Ok(Value::IntSize(a / b))
            }else{
                Err(division_by_zero_error("isize"))
            }
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            if b != 0{
                Ok(Value::UIntSize(a / b))
            }else{
                Err(division_by_zero_error("usize"))
            }
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            if b != 0{
                Ok(Value::Int8(a / b))
            }else{
                Err(division_by_zero_error("i8"))
            }
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            if b != 0{
                Ok(Value::Int16(a / b))
            }else{
                Err(division_by_zero_error("i16"))
            }
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            if b != 0{
                Ok(Value::Int32(a / b))
            }else{
                Err(division_by_zero_error("i32"))
            }
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            if b != 0{
                Ok(Value::Int64(a / b))
            }else{
                Err(division_by_zero_error("i64"))
            }
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            if b != 0{
                Ok(Value::Int128(a / b))
            }else{
                Err(division_by_zero_error("i128"))
            }
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            if b != 0{
                Ok(Value::UInt8(a / b))
            }else{
                Err(division_by_zero_error("u8"))
            }
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            if b != 0{
                Ok(Value::UInt16(a / b))
            }else{
                Err(division_by_zero_error("u16"))
            }
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            if b != 0{
                Ok(Value::UInt32(a / b))
            }else{
                Err(division_by_zero_error("u32"))
            }
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            if b != 0{
                Ok(Value::UInt64(a / b))
            }else{
                Err(division_by_zero_error("u64"))
            }
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            if b != 0{
                Ok(Value::UInt128(a / b))
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

    push_val_or_err(res, s)
    
}

//Creates string for error return in modulo function.
fn modulo_by_zero_error(t: &str) -> String{
    format!("Operator (%) error! Modulo by zero occuring between two operands of type {}!", t)
}

//Pops two items from top of stack and modulos them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as modulo by zero.
fn modulo(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            if b != 0{
                Ok(Value::IntSize(a % b))
            }else{
                Err(modulo_by_zero_error("isize"))
            }
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            if b != 0{
                Ok(Value::UIntSize(a % b))
            }else{
                Err(modulo_by_zero_error("usize"))
            }
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            if b != 0{
                Ok(Value::Int8(a % b))
            }else{
                Err(modulo_by_zero_error("i8"))
            }
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            if b != 0{
                Ok(Value::Int16(a % b))
            }else{
                Err(modulo_by_zero_error("i16"))
            }
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            if b != 0{
                Ok(Value::Int32(a % b))
            }else{
                Err(modulo_by_zero_error("i32"))
            }
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            if b != 0{
                Ok(Value::Int64(a % b))
            }else{
                Err(modulo_by_zero_error("i64"))
            }
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            if b != 0{
                Ok(Value::Int128(a % b))
            }else{
                Err(modulo_by_zero_error("i128"))
            }
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            if b != 0{
                Ok(Value::UInt8(a % b))
            }else{
                Err(modulo_by_zero_error("u8"))
            }
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            if b != 0{
                Ok(Value::UInt16(a % b))
            }else{
                Err(modulo_by_zero_error("u16"))
            }
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            if b != 0{
                Ok(Value::UInt32(a % b))
            }else{
                Err(modulo_by_zero_error("u32"))
            }
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            if b != 0{
                Ok(Value::UInt64(a % b))
            }else{
                Err(modulo_by_zero_error("u64"))
            }
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            if b != 0{
                Ok(Value::UInt128(a % b))
            }else{
                Err(modulo_by_zero_error("u128"))
            }
        },

        (Some(a), Some(b)) => {
            Err(format!("Operator (% [aka mod]) error! Modulo operation requires two operands with \
                a singular matching type that is an integer type! Attempted values: {} and {}", a, b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("% [aka mod]", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("% [aka mod]", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("modulo")),
    };

    push_val_or_err(res, s)
    
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

    push_val_or_err(res, s)
    
}

//Swaps the top two items on the stack, errors out of inusfficient items exist.
fn swap(s: &mut State) -> Result<(), String>{
    match s.pop2(){
        (Some(a), Some(b)) => {
            s.stack.push(b);
            s.stack.push(a);
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
            s.stack.push(c);
            s.stack.push(a);
            s.stack.push(b);

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
            s.stack.push(v.clone());
            s.stack.push(v.clone());
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
                s.stack.push(Value::StringBox(bn));
                s.stack.push(Value::StringBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("StringBox", Value::StringBox(bn)))
            }
        },
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                let dupped_list = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_list);
                s.stack.push(Value::ListBox(bn));
                s.stack.push(Value::ListBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("ListBox", Value::ListBox(bn)))
            }
        },
        Some(Value::ObjectBox(bn)) => {
            if s.validate_box(bn){
                let dupped_obj = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_obj);
                s.stack.push(Value::ObjectBox(bn));
                s.stack.push(Value::ObjectBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("ObjectBox", Value::ObjectBox(bn)))
            }
        },
        Some(Value::MiscBox(bn)) => {
            if s.validate_box(bn){
                let dupped_data = s.heap[bn].0.clone();
                let new_bn = s.insert_to_heap(dupped_data);
                s.stack.push(Value::MiscBox(bn));
                s.stack.push(Value::MiscBox(new_bn));
                Ok(())
            }else{
                Err(error_for_deep_dup_due_to_bad_box("MiscBox", Value::MiscBox(bn)))
            }
        },
        Some(v) => {
            s.stack.push(v.clone());
            s.stack.push(v.clone());
            Ok(())
        },
        None => Err(needs_n_args_only_n_provided("dup", "One", "none")),
    }
}

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
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Boolean(a == b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::Boolean(a == b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
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

    push_val_or_err(res, s)
    
}

//Checks for inequality between two data types. For boxes it checks to see 
// if the box numbers are equal and for NULL box it checks for self-inequality.
//Consumes both items from stack and pushes resulting boolean based on their comparison.
fn is_not_equal(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Boolean(a != b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
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

    push_val_or_err(res, s)
    
}

fn comparison_error(op_type: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Non-equality comparison operators need matching \
        non-null types to function! Attempted values: {} and {}", op_type, v1, v2)
}

//Compares two values on stack to see if the second to top is greater than the top.
fn is_greater_than(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Boolean(a > b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::Boolean(a > b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
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

    push_val_or_err(res, s)
    
}

//Compares two values on stack to see if the second to top is less than the top.
fn is_less_than(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Boolean(a < b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::Boolean(a < b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
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

    push_val_or_err(res, s)
    
}

//Compares two values on stack to see if the second to top is greater than or equal to the top.
fn is_greater_than_equal_to(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Boolean(a >= b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::Boolean(a >= b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
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

    push_val_or_err(res, s)
    
}

//Compares two values on stack to see if the second to top is less than or equal to the top.
fn is_less_than_equal_to(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Boolean(a <= b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::Boolean(a <= b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
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

    push_val_or_err(res, s)
    
}

//Creates error string for when a box involved is invalid.
fn bad_box_error(op_type: &str, box_type1: &str, box_type2: &str, bn1: usize, bn2: usize, is_two_boxes: bool) -> String{
    if !is_two_boxes{
        format!("Operator ({}) error! Box {} of type {} is invalid \
            because it's either out of range of heap or free'd!", op_type, bn1, box_type1)
    }else{
        format!("Operator ({}) error! Box {} of type {} and Box {} of type {} are invalid \
            because they're either out of range of heap or free'd!", op_type, bn1, box_type1, bn2, box_type2)
    }
}

//Concatenates two strings or two lists together.
//DECIDED THAT TOP STRING BOX ISN'T FREED SINCE IT DOESN'T NEED TO BE
fn concat(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            //Only concatenates the strings if both boxes are valid and different!
            if a != b{
                match (s.validate_box(a), s.validate_box(b)){
                    (true, true) => {
                        //Concatenates string from Box B to string in Box A.
                        let mut a_str: HeapValue = std::mem::take(&mut s.heap[a].0);
                        if let (HeapValue::String(ref mut s1), HeapValue::String(ref s2)) = (&mut a_str, &s.heap[b].0){
                            s1.push_str(s2);
                            s.heap[a].0 = a_str;
                            Ok(Value::StringBox(a))
                        }else{
                            s.heap[a].0 = a_str;
                            Err(should_never_get_here_for_func("concat"))
                        }
                    },
                    (true, false) => {
                        Err(bad_box_error("++", "StringBox", "NA", b, usize::MAX, false))
                    },
                    (false, true) => {
                        Err(bad_box_error("++", "StringBox", "NA", a, usize::MAX, false))
                    },
                    (false, false) => {
                        Err(bad_box_error("++", "StringBox", "StringBox", a, b, true))
                    },
                }
            }else{
                Err(format!("Operator (++) error! Concatenation needs two DIFFERENT \
                    string boxes to work! Attempted values: StringBox {} and StringBox {}", a, b))
            }
        },

        (Some(Value::ListBox(a)), Some(Value::ListBox(b))) => {
            //Only concatenates the lists if both boxes are valid and different!
            if a != b{
                match (s.validate_box(a), s.validate_box(b)){
                    (true, true) => {
                        //Concatenates list from Box B to list in Box A.
                        //THIS IS CRINGE, TRY TO MAYBE FIGURE OUT A BETTER WAY LATER
                        let mut list_a: HeapValue = std::mem::take(&mut s.heap[a].0);
                        if let (HeapValue::List(ref mut ls1), HeapValue::List(ref ls2)) = (&mut list_a, &s.heap[b].0){
                            for el in ls2.iter(){
                                ls1.push(el.clone());
                            }
                            s.heap[a].0 = list_a;
                            Ok(Value::ListBox(a))
                        }else{
                            Err(should_never_get_here_for_func("concat"))
                        }
                    },
                    (true, false) => {
                        Err(bad_box_error("++", "ListBox", "NA", b, usize::MAX, false))
                    },
                    (false, true) => {
                        Err(bad_box_error("++", "ListBox", "NA", a, usize::MAX, false))
                    },
                    (false, false) => {
                        Err(bad_box_error("++", "ListBox", "ListBox", a, b, true))
                    },
                }
            }else{
                Err(format!("Operator (++) error! Concatenation needs two DIFFERENT \
                    string boxes to work! Attempted values: StringBox {} and StringBox {}", a, b))
            }
        },

        (Some(a), Some(b)) => {
            Err(format!("Operator (++) error! Concatenation needs top two operands to \
                be matching types of type StringBox or ListBox! Attempted values: {} and {}", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("++", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("++", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("concat")),
    };

    push_val_or_err(res, s)
}

fn logical_operator_type_error(op_type: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Logical operation requires two operands \
        of matching type Boolean! Attempted values: {} and {}", op_type, v1, v2)
}

//Performs logical AND on two operands.
fn and(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a && b))
        },
        (Some(a), Some(b)) => {
            Err(logical_operator_type_error("and/&&", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("and/&&", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("and/&&", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("and")),
    };

    push_val_or_err(res, s)
}

//Performs logical OR on two operands.
fn or(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a || b))
        },
        (Some(a), Some(b)) => {
            Err(logical_operator_type_error("or/||", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("or/||", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("or/||", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("or")),
    };

    push_val_or_err(res, s)
}

//Performs logical XOR on two operands.
fn xor(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
            Ok(Value::Boolean(a != b))
        },
        (Some(a), Some(b)) => {
            Err(logical_operator_type_error("xor", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("xor", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("xor", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("xor")),
    };

    push_val_or_err(res, s)
}

//Performs logical NOT on top of stack if boolean.
fn not(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop(){
        Some(Value::Boolean(x)) => Ok(Value::Boolean(!x)),
        Some(x) => {
            Err(format!("Operator (not/!) error! Logical NOT requires \
                one Boolean type on the stack. Attempted value: {}", x))
        },
        None => Err(needs_n_args_only_n_provided("not/!", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Pushes a value to a list or a character to a string.
fn list_push(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::ListBox(bn)), Some(v)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref mut ls) = &mut s.heap[bn].0{
                    ls.push(v);
                    Ok(Value::ListBox(bn))
                }else{
                    Err(should_never_get_here_for_func("list_push"))
                }
            }else{
                Err(bad_box_error("push/p", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(Value::StringBox(bn)), Some(Value::Char(c))) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref mut st) = &mut s.heap[bn].0{
                    st.push(c);
                    Ok(Value::StringBox(bn))
                }else{
                    Err(should_never_get_here_for_func("list_push"))
                }
            }else{
                Err(bad_box_error("push/p", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(a), Some(b)) => {
            Err(format!("Operator (push/p) error! Push operator requires \
                a ListBox/StringBox second to top on the stack \
                and a Value/Char on top of the stack! Attempted values: {} and {}", a, b))
        },
        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("push/p", "Two", "only one"))
        },
        (None, None) => {
            Err(needs_n_args_only_n_provided("push/p", "Two", "none"))
        },
        _ => Err(should_never_get_here_for_func("list_push")),
    };

    push_val_or_err(res, s)

}

//Generates error string for 0 length errors for pop and fpop error.
fn pop_error(op_type: &str, collection_type: &str, op_detail: &str) -> String{
    format!("Operator ({}) error! {} needs to be greater \
        than length 0 for {} operation to actually pop something!", op_type, collection_type, op_detail)
}

//Pops from the end of a list/string and pushes the popped thing to the stack.
fn list_pop(s: &mut State) -> Result<(), String>{
    let res: Result<(Value, Value), String> = match s.pop(){
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref mut ls) = &mut s.heap[bn].0{
                    match ls.pop(){
                        Some(v) => Ok((Value::ListBox(bn), v)),
                        None => Err(pop_error("pop/po", "List", "pop")),
                    }
                }else{
                    Err(should_never_get_here_for_func("list_pop"))
                }
            }else{
                Err(bad_box_error("pop/po", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref mut st) = &mut s.heap[bn].0{
                    match st.pop(){
                        Some(v) => Ok((Value::StringBox(bn), Value::Char(v))),
                        None => Err(pop_error("pop/po", "String", "pop")),
                    }
                }else{
                    Err(should_never_get_here_for_func("list_pop"))
                }
            }else{
                Err(bad_box_error("pop/po", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => {
            Err(format!("Operator (pop/po) error! Top of stack needs \
                to be of type StringBox or ListBox! Attempted value: {}", v))
        },
        None => {
            Err(needs_n_args_only_n_provided("pop/po", "One", "none"))
        },
    };  

    match res{
        Ok((v1, v2)) => {
            s.stack.push(v1);
            s.stack.push(v2);
            Ok(())
        },
        Err(e) => Err(e),
    }
}

//Pushes a value to the front of a list or a character to the front of a string.
fn list_front_push(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::ListBox(bn)), Some(v)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref mut ls) = &mut s.heap[bn].0{
                    ls.insert(0, v);
                    Ok(Value::ListBox(bn))
                }else{
                    Err(should_never_get_here_for_func("list_front_push"))
                }
            }else{
                Err(bad_box_error("fpush/fp", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(Value::StringBox(bn)), Some(Value::Char(c))) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref mut st) = &mut s.heap[bn].0{
                    st.insert(0, c);
                    Ok(Value::StringBox(bn))
                }else{
                    Err(should_never_get_here_for_func("list_front_push"))
                }
            }else{
                Err(bad_box_error("fpush/fp", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(a), Some(b)) => {
            Err(format!("Operator (fpush/fp) error! Front push operator requires \
                a ListBox/StringBox second to top on the stack \
                and a Value/Char on top of the stack! Attempted values: {} and {}", a, b))
        },
        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("fpush/fp", "Two", "only one"))
        },
        (None, None) => {
            Err(needs_n_args_only_n_provided("fpush/fp", "Two", "none"))
        },
        _ => Err(should_never_get_here_for_func("list_front_push")),
    };

    push_val_or_err(res, s)

}

//Pops from the front of a list/string and pushes the popped thing to the stack.
fn list_front_pop(s: &mut State) -> Result<(), String>{
    let res: Result<(Value, Value), String> = match s.pop(){
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref mut ls) = &mut s.heap[bn].0{
                    if ls.len() > 0{
                        Ok((Value::ListBox(bn), ls.remove(0)))
                    }else{
                        Err(pop_error("fpop/fpo", "List", "front pop"))
                    }
                }else{
                    Err(should_never_get_here_for_func("list_front_pop"))
                }
            }else{
                Err(bad_box_error("fpop/fpo", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref mut st) = &mut s.heap[bn].0{
                    if st.len() > 0{
                        Ok((Value::StringBox(bn), Value::Char(st.remove(0))))
                    }else{
                        Err(pop_error("fpop/fpo", "String", "front pop"))
                    }
                }else{
                    Err(should_never_get_here_for_func("list_front_pop"))
                }
            }else{
                Err(bad_box_error("fpop/fpo", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => {
            Err(format!("Operator (fpop/fpo) error! Top of stack needs \
                to be of type StringBox or ListBox! Attempted value: {}", v))
        },
        None => {
            Err(needs_n_args_only_n_provided("fpop/fpo", "One", "none"))
        },
    };  

    match res{
        Ok((v1, v2)) => {
            s.stack.push(v1);
            s.stack.push(v2);
            Ok(())
        },
        Err(e) => Err(e),
    }
}

//Indexes into a list or string, 
// pushing the indexed item to the stack.
fn index(s: &mut State) -> Result<(), String>{
    let res = match s.pop2(){
        (Some(Value::ListBox(bn)), Some(Value::UIntSize(i))) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref ls) = s.heap[bn].0{
                    if i < ls.len(){
                        Ok(ls[i].clone())
                    }else{
                        Err(format!("Operator (index) error! \
                            Index {} is out of range of List of size {}", i, ls.len()))
                    }
                }else{
                    Err(should_never_get_here_for_func("index"))
                }
            }else{
                Err(bad_box_error("index", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(Value::StringBox(bn)), Some(Value::UIntSize(i))) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref st) = s.heap[bn].0{
                    if i < st.len(){
                        Ok(Value::Char(st.chars().nth(i).unwrap()))
                    }else{
                        Err(format!("Operator (index) error! \
                            Index {} is out of range of String of size {}", i, st.len()))
                    }
                }else{
                    Err(should_never_get_here_for_func("index"))
                }
            }else{
                Err(bad_box_error("index", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(a), Some(b)) => {
            Err(format!("Operator (index) error! Index operator requires second \
                to top of stack to be either a ListBox or a StringBox, \
                and requires the top of the stack to be of type usize! \
                Attempted values: {} and {}", a, b))
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("index", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("index", "Two", "none")),
        _ => Err(should_never_get_here_for_func("index")),
    };

    push_val_or_err(res, s)

}

//Determines length of string or list and pushes it to stack.
fn length(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop(){
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref ls) = s.heap[bn].0{
                    Ok(Value::UIntSize(ls.len()))
                }else{
                    Err(should_never_get_here_for_func("length"))
                }
            }else{
                Err(bad_box_error("length/len", "ListBox", "NA", bn, usize::MAX, false))
            }            
        },
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref st) = s.heap[bn].0{
                    Ok(Value::UIntSize(st.len()))
                }else{
                    Err(should_never_get_here_for_func("length"))
                }
            }else{
                Err(bad_box_error("length/len", "StringBox", "NA", bn, usize::MAX, false))
            }            
        },
        Some(v) => {
            Err(format!("Operator (length/len) error! Top of stack must \
                be of type ListBox or StringBox! Attempted value: {}", v))
        },
        None => Err(needs_n_args_only_n_provided("length/len", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Takes a string/list and pushes a boolean based on whether it's empty or not.
fn is_empty(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop(){
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref ls) = s.heap[bn].0{
                    Ok(Value::Boolean(ls.len() == 0))
                }else{
                    Err(should_never_get_here_for_func("is_empty"))
                }
            }else{
                Err(bad_box_error("isEmpty", "ListBox", "NA", bn, usize::MAX, false))
            }            
        },
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref st) = s.heap[bn].0{
                    Ok(Value::Boolean(st.len() == 0))
                }else{
                    Err(should_never_get_here_for_func("is_empty"))
                }
            }else{
                Err(bad_box_error("isEmpty", "StringBox", "NA", bn, usize::MAX, false))
            }            
        },
        Some(v) => {
            Err(format!("Operator (isEmpty) error! Top of stack must \
                be of type ListBox or StringBox! Attempted value: {}", v))
        },
        None => Err(needs_n_args_only_n_provided("isEmpty", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Clears a list/string to empty.
fn list_clear(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop(){
        Some(Value::ListBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref mut ls) = &mut s.heap[bn].0{
                    ls.clear();
                    Ok(Value::ListBox(bn))
                }else{
                    Err(should_never_get_here_for_func("list_clear"))
                }
            }else{
                Err(bad_box_error("clear", "ListBox", "NA", bn, usize::MAX, false))
            }            
        },
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref mut st) = &mut s.heap[bn].0{
                    st.clear();
                    Ok(Value::StringBox(bn))
                }else{
                    Err(should_never_get_here_for_func("list_clear"))
                }
            }else{
                Err(bad_box_error("clear", "StringBox", "NA", bn, usize::MAX, false))
            }            
        },
        Some(v) => {
            Err(format!("Operator (clear) error! Top of stack must \
                be of type ListBox or StringBox! Attempted value: {}", v))
        },
        None => Err(needs_n_args_only_n_provided("clear", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Consumes a list/object/string box and a value/char and 
// pushes a boolean based on whether or not that value/value/char
// is in that list/object/string box.
fn list_contains(s: &mut State) -> Result<(), String>{
    let res = match s.pop2(){
        (Some(Value::ListBox(bn)), Some(v)) => {
            if s.validate_box(bn){
                if let HeapValue::List(ref ls) = &s.heap[bn].0{
                    Ok(Value::Boolean(ls.contains(&v)))
                }else{
                    Err(should_never_get_here_for_func("list_contains"))
                }
            }else{
                Err(bad_box_error("contains", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    if let (HeapValue::Object(ref o), HeapValue::String(ref s)) = (&s.heap[a].0, &s.heap[b].0){
                        Ok(Value::Boolean(o.contains_key(s)))
                    }else{
                        Err(should_never_get_here_for_func("list_contains"))
                    }
                },
                (true, false) => {
                    Err(bad_box_error("contains", "StringBox", "NA", b, usize::MAX, false))
                },
                (false, true) => {
                    Err(bad_box_error("contains", "ObjectBox", "NA", a, usize::MAX, false))
                },
                (false, false) => {
                    Err(bad_box_error("contains", "ObjectBox", "StringBox", a, b, true))
                },
            }
        },
        (Some(Value::StringBox(bn)), Some(Value::Char(c))) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref st) = &s.heap[bn].0{
                    Ok(Value::Boolean(st.contains(c)))
                }else{
                    Err(should_never_get_here_for_func("list_contains"))
                }
            }else{
                Err(bad_box_error("contains", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(a), Some(b)) => {
            Err(format!("Operator (contains) error! Second to top \
                of stack must be type ListBox/ObjectBox/StringBox and top \
                of stack must be Value/StringBox/Char respectably! \
                Attempted values: {} and {}", a, b))
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("contains", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("contains", "Two", "none")),
        _ => Err(should_never_get_here_for_func("list_contains")),
    };


    push_val_or_err(res, s)
}

//Alters an item in a list at a particular index to something else.
//MAYBE ADD ABILITY TO CHANGE CHARS IN STRING IN THE FUTURE
fn change_item_at(s: &mut State) -> Result<(), String>{
    let res = match s.pop3(){
        (Some(Value::ListBox(bn)), Some(Value::UIntSize(i)), Some(v)) => {
            //Changes item in list to new value at 
            // index i assuming list is valid and index is in range.
            if s.validate_box(bn){
                if let HeapValue::List(ref mut ls) = &mut s.heap[bn].0{
                    if i < ls.len(){
                        ls[i] = v;
                        Ok(Value::ListBox(bn))
                    }else{
                        Err(format!("Operator (changeItemAt) error! \
                            Index {} is out of range of List of size {}", i, ls.len()))
                    }
                }else{
                    Err(should_never_get_here_for_func("change_item_at"))
                }
            }else{
                Err(bad_box_error("changeItemAt", "ListBox", "NA", bn, usize::MAX, false))
            }
        },
        (Some(a), Some(b), Some(c)) => {
            Err(format!("Operator (changeItemAt) error! \
                Third to top of stack must be type ListBox, \
                second to top of stack must be type usize, \
                and top of stack must by type Value! Attempted values: {}, {}, and {}", a, b, c))
        },
        (None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided("changeItemAt", "Three", "only two")),
        (None, None, Some(_)) => Err(needs_n_args_only_n_provided("changeItemAt", "Three", "only one")),
        (None, None, None) => Err(needs_n_args_only_n_provided("changeItemAt", "Three", "none")),
        _ => Err(should_never_get_here_for_func("change_item_at")),
    };

    push_val_or_err(res, s)

}

//Creates an error string for the three char operators below.
fn non_char_error(op_type: &str, v: &Value) -> String{
    format!("Operator ({}) error! Top of stack must \
        be of type Char! Attempted value: {}", op_type, v)
}

//Conumes a character and pushes a boolean saying whether or not it's whitespace.
fn whitespace_detect(s: &mut State) -> Result<(), String>{
    let res = match s.pop(){
        Some(Value::Char(c)) => {
            Ok(Value::Boolean(c.is_whitespace()))
        },
        Some(v) => {
            Err(non_char_error("isWhitespaceChar", &v))
        },
        None => Err(needs_n_args_only_n_provided("isWhitespaceChar", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Determines if top of stack is an alphabetical char.
fn alpha_char_detect(s: &mut State) -> Result<(), String>{
    let res = match s.pop(){
        Some(Value::Char(c)) => {
            Ok(Value::Boolean(c.is_alphabetic()))
        },
        Some(v) => {
            Err(non_char_error("isAlphaChar", &v))
        },
        None => Err(needs_n_args_only_n_provided("isAlphaChar", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Determines if top of stack is a numeric char.
fn num_char_detect(s: &mut State) -> Result<(), String>{
    let res = match s.pop(){
        Some(Value::Char(c)) => {
            Ok(Value::Boolean(c.is_numeric()))
        },
        Some(v) => {
            Err(non_char_error("isNumChar", &v))
        },
        None => Err(needs_n_args_only_n_provided("isNumChar", "One", "none")),
    };

    push_val_or_err(res, s)
}

fn invalid_types_for_obj_add_or_mut(op_type: &str, v1: &Value, v2: &Value, v3: &Value) -> String{
    format!("Operator ({}) error! Third to top of stack must be of type ObjectBox, \
        second to top must be type StringBox, and top must be type Value! \
        Attempted values: {}, {}, and {}", op_type, v1, v2, v3)
}

//Adds a field to the given object and pushes the mutated object back.
fn add_field(s: &mut State) -> Result<(), String>{
    let res = match s.pop3(){
        (Some(Value::ObjectBox(a)), Some(Value::StringBox(b)), Some(v)) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    let mut obj_to_mut = std::mem::take(&mut s.heap[a].0);
                    if let (HeapValue::Object(ref mut o), HeapValue::String(ref st)) = (&mut obj_to_mut, &s.heap[b].0){
                        if !o.contains_key(st){
                            o.insert(st.clone(), v);
                            s.heap[a].0 = obj_to_mut;
                            Ok(Value::ObjectBox(a))
                        }else{
                            let ret = Err(format!("Operator (objAddField) error! \
                                ObjectBox {} already contains field \"{}\"! \
                                Try removing it first!", a, st));
                            s.heap[a].0 = obj_to_mut;
                            ret
                        }
                    }else{
                        Err(should_never_get_here_for_func("add_field"))
                    }
                },
                (true, false) => Err(bad_box_error("objAddField", "StringBox", "NA", b, usize::MAX, false)),
                (false, true) => Err(bad_box_error("objAddField", "ObjectBox", "NA", a, usize::MAX, false)),
                (false, false) => Err(bad_box_error("objAddField", "ObjectBox", "StringBox", a, b, true)),
            }
        },
        (Some(a), Some(b), Some(c)) => {
            Err(invalid_types_for_obj_add_or_mut("objAddField", &a, &b, &c))
        },
        (None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided("objAddField", "Three", "only two")),
        (None, None, Some(_)) => Err(needs_n_args_only_n_provided("objAddField", "Three", "only one")),
        (None, None, None) => Err(needs_n_args_only_n_provided("objAddField", "Three", "none")),
        _ => Err(should_never_get_here_for_func("add_field")),
    };

    push_val_or_err(res, s)
}

fn invalid_types_for_get_or_rem(op_type: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Second to top of stack must \
        be type ObjectBox and top must be type StringBox! \
        Attempted values: {} and {}", op_type, v1, v2)
}

fn field_not_in_obj_err(op_type: &str, box_num: usize, field_name: &str) -> String{
    format!("Operator ({}) error! Field \"{}\" doesn't exist \
        in ObjectBox {} ! Try adding it!", op_type, field_name, box_num) 
}

//Given an object and string box, conumes the boxes 
// and pushes the value at that key if it exists.
fn get_field(s: &mut State) -> Result<(), String>{
    let res = match s.pop2(){
        (Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    if let (HeapValue::Object(ref o), HeapValue::String(ref st)) = (&s.heap[a].0, &s.heap[b].0){
                        match o.get(st){
                            Some(v) => Ok(v.clone()),
                            None => Err(field_not_in_obj_err("objGetField", a, st))
                        }
                    }else{
                        Err(should_never_get_here_for_func("get_field"))
                    }
                },
                (true, false) => Err(bad_box_error("objGetField", "StringBox", "NA", b, usize::MAX, false)),
                (false, true) => Err(bad_box_error("objGetField", "ObjectBox", "NA", a, usize::MAX, false)),
                (false, false) => Err(bad_box_error("objGetField", "ObjectBox", "StringBox", a, b, true)),
            }
        },
        (Some(a), Some(b)) => {
            Err(invalid_types_for_get_or_rem("objGetField", &a, &b))
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("objGetField", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("objGetField", "Two", "none")),
        _ => Err(should_never_get_here_for_func("get_field")),
    };

    push_val_or_err(res, s)
}

//Determines if a translation from one value type to another is valid. 
// Typically the types have to match unless it's nullbox to box stuff.
fn is_valid_mutation(a: &Value, b: &Value) -> bool{
    match (a, b) {
        (Value::IntSize(_), Value::IntSize(_)) => true,
        (Value::UIntSize(_), Value::UIntSize(_)) => true,

        (Value::Int8(_), Value::Int8(_)) => true,
        (Value::Int16(_), Value::Int16(_)) => true,
        (Value::Int32(_), Value::Int32(_)) => true,
        (Value::Int64(_), Value::Int64(_)) => true,
        (Value::Int128(_), Value::Int128(_)) => true,

        (Value::UInt8(_), Value::UInt8(_)) => true,
        (Value::UInt16(_), Value::UInt16(_)) => true,
        (Value::UInt32(_), Value::UInt32(_)) => true,
        (Value::UInt64(_), Value::UInt64(_)) => true,
        (Value::UInt128(_), Value::UInt128(_)) => true,

        (Value::Float32(_), Value::Float32(_)) => true,
        (Value::Float64(_), Value::Float64(_)) => true,

        (Value::Char(_), Value::Char(_)) => true,
        (Value::Boolean(_), Value::Boolean(_)) => true,

        (Value::StringBox(_), Value::StringBox(_)) => true,
        (Value::ListBox(_), Value::ListBox(_)) => true,
        (Value::MiscBox(_), Value::MiscBox(_)) => true,

        (Value::NULLBox, Value::StringBox(_)) => true,
        (Value::StringBox(_), Value::NULLBox) => true,
        (Value::NULLBox, Value::ListBox(_)) => true,
        (Value::ListBox(_), Value::NULLBox) => true,
        (Value::NULLBox, Value::ObjectBox(_)) => true,
        (Value::ObjectBox(_), Value::NULLBox) => true,
        (Value::NULLBox, Value::MiscBox(_)) => true,
        (Value::MiscBox(_), Value::NULLBox) => true,
        (Value::NULLBox, Value::NULLBox) => true,

        _ => false,

    }
}

fn invalid_mutation_error(op_type: &str, thing_being_mutated: &str, thing_name: &str, v1: &Value, v2: &Value) -> String{
    format!("Operator ({}) error! Invalid mutation of {} {} ! \
        Unable to mutate {} to {}", op_type, thing_being_mutated, thing_name, v1, v2)
}

//Mutates the field to a new value in an object if it exists and it's a valid mutation.
fn mut_field(s: &mut State) -> Result<(), String>{
    let res = match s.pop3(){
        (Some(Value::ObjectBox(a)), Some(Value::StringBox(b)), Some(v)) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    let mut obj_to_mut = std::mem::take(&mut s.heap[a].0);
                    if let (HeapValue::Object(ref mut o), HeapValue::String(ref st)) = (&mut obj_to_mut, &s.heap[b].0){
                        match o.get_mut(st){
                            Some(old_v) => {
                                if is_valid_mutation(old_v, &v){
                                    *old_v = v;
                                    s.heap[a].0 = obj_to_mut;
                                    Ok(Value::ObjectBox(a))
                                }else{
                                    let ret = Err(invalid_mutation_error("objMutField", "Object field", st, &old_v, &v));
                                    s.heap[a].0 = obj_to_mut;
                                    ret
                                }
                            },
                            None => {
                                let ret = Err(field_not_in_obj_err("objMutField", a, st));
                                s.heap[a].0 = obj_to_mut; 
                                ret
                            },
                        }
                    }else{
                        s.heap[a].0 = obj_to_mut;
                        Err(should_never_get_here_for_func("mut_field"))
                    }
                },
                (true, false) => Err(bad_box_error("objMutField", "StringBox", "NA", b, usize::MAX, false)),
                (false, true) => Err(bad_box_error("objMutField", "ObjectBox", "NA", a, usize::MAX, false)),
                (false, false) => Err(bad_box_error("objMutField", "ObjectBox", "StringBox", a, b, true)),
            }
        },
        (Some(a), Some(b), Some(c)) => {
            Err(invalid_types_for_obj_add_or_mut("objMutField", &a, &b, &c))
        },
        (None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided("objMutField", "Three", "only two")),
        (None, None, Some(_)) => Err(needs_n_args_only_n_provided("objMutField", "Three", "only one")),
        (None, None, None) => Err(needs_n_args_only_n_provided("objMutField", "Three", "none")),
        _ => Err(should_never_get_here_for_func("mut_field")),
    };

    push_val_or_err(res, s)
}

//Removes a field from an object at the desired key held in the string box.
fn remove_field(s: &mut State) -> Result<(), String>{
    let res = match s.pop2(){
        (Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    let mut obj_to_mut = std::mem::take(&mut s.heap[a].0);
                    if let (HeapValue::Object(ref mut o), HeapValue::String(ref st)) = (&mut obj_to_mut, &s.heap[b].0){
                        match o.remove(st){
                            Some(_) => {
                                s.heap[a].0 = obj_to_mut;
                                Ok(Value::ObjectBox(a))
                            },
                            None => {
                                let ret = Err(field_not_in_obj_err("objRemField", a, st));
                                s.heap[a].0 = obj_to_mut;
                                ret
                            },
                        }
                    }else{
                        s.heap[a].0 = obj_to_mut;
                        Err(should_never_get_here_for_func("remove_field"))
                    }
                },
                (true, false) => Err(bad_box_error("objRemField", "StringBox", "NA", b, usize::MAX, false)),
                (false, true) => Err(bad_box_error("objRemField", "ObjectBox", "NA", a, usize::MAX, false)),
                (false, false) => Err(bad_box_error("objRemField", "ObjectBox", "StringBox", a, b, true)),
            }
        },
        (Some(a), Some(b)) => {
            Err(invalid_types_for_get_or_rem("objRemField", &a, &b))
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("objRemField", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("objRemField", "Two", "none")),
        _ => Err(should_never_get_here_for_func("remove_field")),
    };

    push_val_or_err(res, s)
}

//Acts like C's strcmp, eating two string boxes and pushing 
// an integer to indicate the result of the comparison between their contents.
// If the second to top is less than the top, a negative one is pushed
// If the second to top is equal to the top, a zero is pushed
// If the second to top is greater than the top, a one is pushed
fn string_compare(s: &mut State) -> Result<(), String>{
    let res = match s.pop2(){
        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    if let (HeapValue::String(ref str_a), HeapValue::String(ref str_b)) = (&s.heap[a].0, &s.heap[b].0){
                        let comp_res: isize = match str_a.cmp(str_b){
                            Ordering::Less => -1,
                            Ordering::Equal => 0,
                            Ordering::Greater => 1,
                        };
                        Ok(Value::IntSize(comp_res))
                    }else{
                        Err(should_never_get_here_for_func("string_compare"))
                    }
                },
                (true, false) => Err(bad_box_error("stringCompare", "StringBox", "NA", b, usize::MAX, false)),
                (false, true) => Err(bad_box_error("stringCompare", "StringBox", "NA", a, usize::MAX, false)),
                (false, false) => Err(bad_box_error("stringCompare", "StringBox", "StringBox", a, b, true)),
            }
        },
        (Some(a), Some(b)) => {
            Err(format!("Operator (stringCompare) error! String comparison \
                requires two items of type StringBox on the stack! \
                Attempted values: {} and {}", &a, &b))
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("stringCompare", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("stringCompare", "Two", "none")),
        _ => Err(should_never_get_here_for_func("string_compare")),
    };

    push_val_or_err(res, s)
}

//Performs bitwise OR between two integers.
fn bit_or(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::IntSize(a | b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::UIntSize(a | b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Int8(a | b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Int16(a | b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Int32(a | b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Int64(a | b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Int128(a | b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::UInt8(a | b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::UInt16(a | b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::UInt32(a | b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::UInt64(a | b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            Ok(Value::UInt128(a | b))
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("bitOr/|", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("bitOr/|", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("bitOr/|", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("bit_or")),
    };

    push_val_or_err(res, s)
}

//Performs bitwise AND between two matching integer types.
fn bit_and(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::IntSize(a & b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::UIntSize(a & b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Int8(a & b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Int16(a & b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Int32(a & b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Int64(a & b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Int128(a & b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::UInt8(a & b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::UInt16(a & b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::UInt32(a & b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::UInt64(a & b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            Ok(Value::UInt128(a & b))
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("bitAnd/&", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("bitAnd/&", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("bitAnd/&", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("bit_and")),
    };

    push_val_or_err(res, s)
}

//Performs bitwise XOR between two matching integer types.
fn bit_xor(s: &mut State) -> Result<(), String>{
    let res: Result<Value, String> = match s.pop2(){
        (Some(Value::IntSize(a)), Some(Value::IntSize(b))) => {
            Ok(Value::IntSize(a ^ b))
        },
        (Some(Value::UIntSize(a)), Some(Value::UIntSize(b))) => {
            Ok(Value::UIntSize(a ^ b))
        },

        (Some(Value::Int8(a)), Some(Value::Int8(b))) => {
            Ok(Value::Int8(a ^ b))
        },
        (Some(Value::Int16(a)), Some(Value::Int16(b))) => {
            Ok(Value::Int16(a ^ b))
        },
        (Some(Value::Int32(a)), Some(Value::Int32(b))) => {
            Ok(Value::Int32(a ^ b))
        },
        (Some(Value::Int64(a)), Some(Value::Int64(b))) => {
            Ok(Value::Int64(a ^ b))
        },
        (Some(Value::Int128(a)), Some(Value::Int128(b))) => {
            Ok(Value::Int128(a ^ b))
        },

        (Some(Value::UInt8(a)), Some(Value::UInt8(b))) => {
            Ok(Value::UInt8(a ^ b))
        },
        (Some(Value::UInt16(a)), Some(Value::UInt16(b))) => {
            Ok(Value::UInt16(a ^ b))
        },
        (Some(Value::UInt32(a)), Some(Value::UInt32(b))) => {
            Ok(Value::UInt32(a ^ b))
        },
        (Some(Value::UInt64(a)), Some(Value::UInt64(b))) => {
            Ok(Value::UInt64(a ^ b))
        },
        (Some(Value::UInt128(a)), Some(Value::UInt128(b))) => {
            Ok(Value::UInt128(a ^ b))
        },

        (Some(a), Some(b)) => {
            Err(numerical_type_error_string("bitXor/^", &a, &b))
        },

        (None, Some(_)) => {
            Err(needs_n_args_only_n_provided("bitXor/^", "Two", "only one"))
        },

        (None, None) => {
            Err(needs_n_args_only_n_provided("bitXor/^", "Two", "none"))
        },

        _ => Err(should_never_get_here_for_func("bit_xor")),
    };

    push_val_or_err(res, s)
}

//Performs a bitwise not on an integer on the stack.
fn bit_not(s: &mut State) -> Result<(), String>{
    let res = match s.pop(){
        Some(Value::IntSize(n)) => Ok(Value::IntSize(!n)),
        Some(Value::UIntSize(n)) => Ok(Value::UIntSize(!n)),

        Some(Value::Int8(n)) => Ok(Value::Int8(!n)),
        Some(Value::Int16(n)) => Ok(Value::Int16(!n)),
        Some(Value::Int32(n)) => Ok(Value::Int32(!n)),
        Some(Value::Int64(n)) => Ok(Value::Int64(!n)),
        Some(Value::Int128(n)) => Ok(Value::Int128(!n)),

        Some(Value::UInt8(n)) => Ok(Value::UInt8(!n)),
        Some(Value::UInt16(n)) => Ok(Value::UInt16(!n)),
        Some(Value::UInt32(n)) => Ok(Value::UInt32(!n)),
        Some(Value::UInt64(n)) => Ok(Value::UInt64(!n)),
        Some(Value::UInt128(n)) => Ok(Value::UInt128(!n)),

        Some(v) => {
            Err(format!("Operator (bitNot) error! Bitwise not requires \
                top of stack to be an integer numeric type! \
                Attempted value: {}", &v))
        },
        None => Err(needs_n_args_only_n_provided("bitNot", "One", "none")),

    };

    push_val_or_err(res, s)
}

//Performs a bitshift on stuff.
//OVERFLOW NEEDS TO BE HANDLED LESS JANKILY IN THE FUTURE MOST LIKELY!!!
fn shift<T: std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> + Default>(n: T, shift_n: isize) -> T{
    let t_bit_count = std::mem::size_of::<T>() * 8;
    let shift_n_abs = shift_n.abs() as usize;

    if shift_n >= 0{
        if shift_n_abs < t_bit_count{
            n << shift_n_abs
        }else{
            T::default()
        }
    }else{
        if shift_n_abs < t_bit_count{
            n >> shift_n_abs
        }else{
            T::default()
        }
    }
}

//Performs a left or right bitshift by n bits on an integer.
fn bit_shift(s: &mut State) -> Result<(), String>{
    let res = match s.pop2(){
        (Some(Value::IntSize(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::IntSize(shift(n, shift_n)))
        },
        (Some(Value::UIntSize(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::UIntSize(shift(n, shift_n)))
        },

        (Some(Value::Int8(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::Int8(shift(n, shift_n)))
        },
        (Some(Value::Int16(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::Int16(shift(n, shift_n)))
        },
        (Some(Value::Int32(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::Int32(shift(n, shift_n)))
        },
        (Some(Value::Int64(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::Int64(shift(n, shift_n)))
        },
        (Some(Value::Int128(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::Int128(shift(n, shift_n)))
        },

        (Some(Value::UInt8(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::UInt8(shift(n, shift_n)))
        },
        (Some(Value::UInt16(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::UInt16(shift(n, shift_n)))
        },
        (Some(Value::UInt32(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::UInt32(shift(n, shift_n)))
        },
        (Some(Value::UInt64(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::UInt64(shift(n, shift_n)))
        },
        (Some(Value::UInt128(n)), Some(Value::IntSize(shift_n))) => {
            Ok(Value::UInt128(shift(n, shift_n)))
        },

        (Some(a), Some(b)) => {
            Err(format!("Operator (bitShift) error! Second to top must \
                be numeric integer type and top must be type isize! \
                Attempted values: {} and {}", &a, &b))
        },

        (None, Some(_)) => Err(needs_n_args_only_n_provided("bitShift", "Two", "only one")),

        (None, None) => Err(needs_n_args_only_n_provided("bitShift", "Two", "none")),

        _ => Err(should_never_get_here_for_func("bit_shift")),
    };

    push_val_or_err(res, s)

}

//Pushes maximum value for isize datatype to stack.
fn max_isize(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::IntSize(isize::MAX));
    Ok(())
}

//Pushes maximum value for usize datatype to stack.
fn max_usize(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::UIntSize(usize::MAX));
    Ok(())
}

//Pushes maximum value for i8 datatype to stack.
fn max_i8(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::Int8(i8::MAX));
    Ok(())
}

//Pushes maximum value for i16 datatype to stack.
fn max_i16(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::Int16(i16::MAX));
    Ok(())
}

//Pushes maximum value for i32 datatype to stack.
fn max_i32(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::Int32(i32::MAX));
    Ok(())
}

//Pushes maximum value for i64 datatype to stack.
fn max_i64(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::Int64(i64::MAX));
    Ok(())
}

//Pushes maximum value for i128 datatype to stack.
fn max_i128(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::Int128(i128::MAX));
    Ok(())
}

//Pushes maximum value for u8 datatype to stack.
fn max_u8(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::UInt8(u8::MAX));
    Ok(())
}

//Pushes maximum value for u16 datatype to stack.
fn max_u16(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::UInt16(u16::MAX));
    Ok(())
}

//Pushes maximum value for u32 datatype to stack.
fn max_u32(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::UInt32(u32::MAX));
    Ok(())
}

//Pushes maximum value for u64 datatype to stack.
fn max_u64(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::UInt64(u64::MAX));
    Ok(())
}

//Pushes maximum value for u128 datatype to stack.
fn max_u128(s: &mut State) -> Result<(), String>{
    s.stack.push(Value::UInt128(u128::MAX));
    Ok(())
}

trait ToFloat32 {
    fn into_float32(self) -> f32;
}

trait ToFloat64 {
    fn into_float64(self) -> f64;
}

//This eldritch bit of code essentially makes implementation blocks 
// for every input type to implement ToFloat32 and ToFloat64 traits.
macro_rules! impl_to_float {
    //What this block basically says is to loop through zero 
    // or more type arguments given and generate the two impl's below.
    ($($t:ty), *) => {
        $(
            impl ToFloat32 for $t{
                fn into_float32(self) -> f32{
                    self as f32
                }
            }
            impl ToFloat64 for $t {
                fn into_float64(self) -> f64{
                    self as f64
                }
            }
        )*
    }
}

impl_to_float!(isize, usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

fn numeric_error_cast_string(v: Value, t: &str, r: &str) -> String{
    format!("Operator (cast) error! Failed to cast {} to type {} because: {}", v, t, r)
}

fn invalid_cast_error(t: &str) -> String{
    format!("{} is not a valid type to cast \
        this data type to or is an invalid type", t)
}

//Tries to cast a numeric type to all the other types it could be.
fn cast_num_to_others<T>(t: &str, v: T) -> Result<SuperValue, String>
where 
    T: 
        TryInto<isize> + 
        TryInto<usize> +
        TryInto<i8> +
        TryInto<i16> +
        TryInto<i32> +
        TryInto<i64> +
        TryInto<i128> +
        TryInto<u8> +
        TryInto<u16> +
        TryInto<u32> +
        TryInto<u64> +
        TryInto<u128> +
        ToFloat32 + 
        ToFloat64 +
        Display,
    <T as TryInto<isize>>::Error: std::fmt::Display,
    <T as TryInto<usize>>::Error: std::fmt::Display,
    <T as TryInto<i8>>::Error: std::fmt::Display,
    <T as TryInto<i16>>::Error: std::fmt::Display,
    <T as TryInto<i32>>::Error: std::fmt::Display,
    <T as TryInto<i64>>::Error: std::fmt::Display,
    <T as TryInto<i128>>::Error: std::fmt::Display,
    <T as TryInto<u8>>::Error: std::fmt::Display,
    <T as TryInto<u16>>::Error: std::fmt::Display,
    <T as TryInto<u32>>::Error: std::fmt::Display,
    <T as TryInto<u64>>::Error: std::fmt::Display,
    <T as TryInto<u128>>::Error: std::fmt::Display,
{
    match t{
        "isize" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::IntSize(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        }, 
        "usize" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::UIntSize(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        }, 
        "i8" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::Int8(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "i16" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::Int16(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "i32" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::Int32(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "i64" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::Int64(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "i128" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::Int128(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "u8" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::UInt8(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "u16" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::UInt16(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "u32" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::UInt32(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "u64" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::UInt64(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },
        "u128" => {
            match v.try_into(){
                Ok(casted) => Ok(SuperValue::Reg(Value::UInt128(casted))),
                Err(reason) => Err(reason.to_string()),
            }
        },

        "f32" => {
            Ok(SuperValue::Reg(Value::Float32(v.into_float32())))
        },

        "f64" => {
            Ok(SuperValue::Reg(Value::Float64(v.into_float64())))
        },

        "Char" => {
            match v.try_into(){
                Ok(u32_val) => {
                    match std::char::from_u32(u32_val){
                        Some(casted) => Ok(SuperValue::Reg(Value::Char(casted))),
                        None => Err("given value is outside of valid UTF-8 Char range!".to_string()),
                    }   
                },
                Err(u32_cast_fail) => Err(format!("{}, which means that the given value was unable \
                    to be converted to a u32 to then be converted to a Char!", u32_cast_fail)),
            }
        },

        "String" => {
            Ok(SuperValue::Heap(HeapValue::String(v.to_string())))
        },
        t => Err(invalid_cast_error(t)), 
    }

}

fn bad_stringbox_for_casting_error(box_num: usize) -> String{
    bad_box_error("cast", "StringBox", "NA", box_num, usize::MAX, false)
}

//Function with a generic that carries out the casting action 
// for all numeric data types to make the main cast function more compact.
fn integer_cast_action<T>(s: &mut State, v: Value, v_inside: T, c: &str) -> Result<Value, String>
where 
    T: 
        TryInto<isize> + 
        TryInto<usize> +
        TryInto<i8> +
        TryInto<i16> +
        TryInto<i32> +
        TryInto<i64> +
        TryInto<i128> +
        TryInto<u8> +
        TryInto<u16> +
        TryInto<u32> +
        TryInto<u64> +
        TryInto<u128> +
        ToFloat32 + 
        ToFloat64 +
        Display,
    <T as TryInto<isize>>::Error: std::fmt::Display,
    <T as TryInto<usize>>::Error: std::fmt::Display,
    <T as TryInto<i8>>::Error: std::fmt::Display,
    <T as TryInto<i16>>::Error: std::fmt::Display,
    <T as TryInto<i32>>::Error: std::fmt::Display,
    <T as TryInto<i64>>::Error: std::fmt::Display,
    <T as TryInto<i128>>::Error: std::fmt::Display,
    <T as TryInto<u8>>::Error: std::fmt::Display,
    <T as TryInto<u16>>::Error: std::fmt::Display,
    <T as TryInto<u32>>::Error: std::fmt::Display,
    <T as TryInto<u64>>::Error: std::fmt::Display,
    <T as TryInto<u128>>::Error: std::fmt::Display,
{
    match cast_num_to_others(c, v_inside){
        Ok(SuperValue::Heap(HeapValue::String(st))) => {
            let new_bn = s.insert_to_heap(HeapValue::String(st));
            Ok(Value::StringBox(new_bn))
        },
        Ok(SuperValue::Reg(v)) => Ok(v),
        Err(reason) => Err(numeric_error_cast_string(v, c, &reason)),
        _ => Err(should_never_get_here_for_func("integer_cast_action")),
    }
}

fn string_cast_error(bn: usize, str_contents: &str, t: &str, reason: &str) -> String{
    format!("Operator (cast) error! Failed to cast \
        StringBox {} (\"{}\") to type {} because: {}", bn, str_contents, t, reason)
}

fn general_cast_action(s: &mut State, v: Value, c: &str) -> Result<Value, String>{
    let res = match v{
        Value::IntSize(n) => {
            integer_cast_action(s, Value::IntSize(n), n, c)
        },
        Value::UIntSize(n) => {
            integer_cast_action(s, Value::UIntSize(n), n, c)
        },

        Value::Int8(n) => {
            integer_cast_action(s, Value::Int8(n), n, c)
        },
        Value::Int16(n) => {
            integer_cast_action(s, Value::Int16(n), n, c)
        },
        Value::Int32(n) => {
            integer_cast_action(s, Value::Int32(n), n, c)
        },
        Value::Int64(n) => {
            integer_cast_action(s, Value::Int64(n), n, c)
        },
        Value::Int128(n) => {
            integer_cast_action(s, Value::Int128(n), n, c)
        },

        Value::UInt8(n) => {
            integer_cast_action(s, Value::UInt8(n), n, c)
        },
        Value::UInt16(n) => {
            integer_cast_action(s, Value::UInt16(n), n, c)
        },
        Value::UInt32(n) => {
            integer_cast_action(s, Value::UInt32(n), n, c)
        },
        Value::UInt64(n) => {
            integer_cast_action(s, Value::UInt64(n), n, c)
        },
        Value::UInt128(n) => {
            integer_cast_action(s, Value::UInt128(n), n, c)
        },

        Value::Float32(n) => {
            match c{
                "isize" => Ok(Value::IntSize(n as isize)),
                "usize" => Ok(Value::UIntSize(n as usize)),

                "i8" => Ok(Value::Int8(n as i8)),
                "i16" => Ok(Value::Int16(n as i16)),
                "i32" => Ok(Value::Int32(n as i32)),
                "i64" => Ok(Value::Int64(n as i64)),
                "i128" => Ok(Value::Int128(n as i128)),

                "u8" => Ok(Value::UInt8(n as u8)),
                "u16" => Ok(Value::UInt16(n as u16)),
                "u32" => Ok(Value::UInt32(n as u32)),
                "u64" => Ok(Value::UInt64(n as u64)),
                "u128" => Ok(Value::UInt128(n as u128)),

                "f32" => Ok(Value::Float32(n as f32)),
                "f64" => Ok(Value::Float64(n as f64)),

                "String" => {
                    let f32_str = format!("{}", Value::Float32(n));
                    let new_bn = s.insert_to_heap(HeapValue::String(f32_str[4..].to_string()));
                    Ok(Value::StringBox(new_bn))
                },

                t => Err(numeric_error_cast_string(Value::Float32(n), c, &(invalid_cast_error(c)))),            
            }
        },

        Value::Float64(n) => {
            match c{
                "isize" => Ok(Value::IntSize(n as isize)),
                "usize" => Ok(Value::UIntSize(n as usize)),

                "i8" => Ok(Value::Int8(n as i8)),
                "i16" => Ok(Value::Int16(n as i16)),
                "i32" => Ok(Value::Int32(n as i32)),
                "i64" => Ok(Value::Int64(n as i64)),
                "i128" => Ok(Value::Int128(n as i128)),

                "u8" => Ok(Value::UInt8(n as u8)),
                "u16" => Ok(Value::UInt16(n as u16)),
                "u32" => Ok(Value::UInt32(n as u32)),
                "u64" => Ok(Value::UInt64(n as u64)),
                "u128" => Ok(Value::UInt128(n as u128)),

                "f32" => Ok(Value::Float32(n as f32)),
                "f64" => Ok(Value::Float64(n as f64)),

                "String" => {
                    let f64_str = format!("{}", Value::Float64(n));
                    let new_bn = s.insert_to_heap(HeapValue::String(f64_str[4..].to_string()));
                    Ok(Value::StringBox(new_bn))
                },

                t => Err(numeric_error_cast_string(Value::Float64(n), c, &(invalid_cast_error(c)))),            
            }
        },

        Value::Char(ch) => {
            match c{
                "isize" => Ok(Value::IntSize(ch as isize)),
                "usize" => Ok(Value::UIntSize(ch as usize)),

                "i8" => Ok(Value::Int8(ch as i8)),
                "i16" => Ok(Value::Int16(ch as i16)),
                "i32" => Ok(Value::Int32(ch as i32)),
                "i64" => Ok(Value::Int64(ch as i64)),
                "i128" => Ok(Value::Int128(ch as i128)),

                "u8" => Ok(Value::UInt8(ch as u8)),
                "u16" => Ok(Value::UInt16(ch as u16)),
                "u32" => Ok(Value::UInt32(ch as u32)),
                "u64" => Ok(Value::UInt64(ch as u64)),
                "u128" => Ok(Value::UInt128(ch as u128)),

                "String" => {
                    let new_bn = s.insert_to_heap(HeapValue::String(ch.to_string()));
                    Ok(Value::StringBox(new_bn))
                },

                t => Err(numeric_error_cast_string(Value::Char(ch), c, &(invalid_cast_error(c)))),
            }

        },

        Value::Boolean(b) => {
            match c{
                "isize" => Ok(Value::IntSize(b as isize)),
                "usize" => Ok(Value::UIntSize(b as usize)),

                "i8" => Ok(Value::Int8(b as i8)),
                "i16" => Ok(Value::Int16(b as i16)),
                "i32" => Ok(Value::Int32(b as i32)),
                "i64" => Ok(Value::Int64(b as i64)),
                "i128" => Ok(Value::Int128(b as i128)),

                "u8" => Ok(Value::UInt8(b as u8)),
                "u16" => Ok(Value::UInt16(b as u16)),
                "u32" => Ok(Value::UInt32(b as u32)),
                "u64" => Ok(Value::UInt64(b as u64)),
                "u128" => Ok(Value::UInt128(b as u128)),

                "String" => {
                    let new_bn = s.insert_to_heap(HeapValue::String(b.to_string()));
                    Ok(Value::StringBox(new_bn))
                },

                t => Err(numeric_error_cast_string(Value::Boolean(b), c, &(invalid_cast_error(c)))),
            }

        },

        Value::StringBox(string_num) => {
            if s.validate_box(string_num){
                if let HeapValue::String(ref st) = &s.heap[string_num].0{
                    match c {
                        "isize" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::IntSize(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "usize" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::UIntSize(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        
                        "i8" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Int8(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "i16" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Int16(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "i32" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Int32(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "i64" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Int64(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "i128" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Int128(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },

                        "u8" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::UInt8(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "u16" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::UInt16(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "u32" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::UInt32(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "u64" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::UInt64(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "u128" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::UInt128(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },

                        "f32" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Float32(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },
                        "f64" => {
                            match (*st).parse(){
                                Ok(casted) => Ok(Value::Float64(casted)),
                                Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string())),
                            }
                        },

                        "Boolean" => {
                            if st == "True" || st == "true"{
                                Ok(Value::Boolean(true))
                            }else if st == "False" || st == "false"{
                                Ok(Value::Boolean(false))
                            }else{
                                Err(string_cast_error(string_num, st, c, 
                                    &String::from("provided string is not a valid Boolean")))
                            }
                        },

                        //Casting a string to a string is basically a no-op. 
                        "String" => {
                            Ok(Value::StringBox(string_num))
                        },

                        "List" => {
                            let char_ls: Vec<Value> = st
                                .chars()
                                .map(|c| Value::Char(c))
                                .collect();
                            let ls_bn = s.insert_to_heap(HeapValue::List(char_ls));
                            Ok(Value::ListBox(ls_bn))
                        },

                        t => Err(string_cast_error(string_num, st, c, &invalid_cast_error(t))),
                    }
                }else{
                    Err(should_never_get_here_for_func("general_cast_action"))
                }
            }else{
                Err(bad_stringbox_for_casting_error(string_num))
            }
        },

        Value::ListBox(ls_num) => {
            if s.validate_box(ls_num){
                if let ref ls = &s.heap[ls_num].0{
                    match c{
                        "List" => Ok(Value::ListBox(ls_num)), //No-op
                        "String" => {
                            let ls_str = format!("{}", ls);
                            let new_bn = s.insert_to_heap(HeapValue::String(ls_str[5..].to_string()));
                            Ok(Value::StringBox(new_bn))
                        },
                        t => Err(format!("Operator (cast) error! Failed \
                            to cast ListBox {} to {} because: {}", ls_num, c, &invalid_cast_error(c))),
                    }
                }else{
                    Err(should_never_get_here_for_func("general_cast_action"))
                }
            }else{
                Err(bad_box_error("cast", "ListBox", "NA", ls_num, usize::MAX, false))
            }
        },

        Value::ObjectBox(obj_num) => {
            if s.validate_box(obj_num){
                if let ref obj = &s.heap[obj_num].0{
                    match c {
                        "Object" => Ok(Value::ObjectBox(obj_num)),
                        "String" => {
                            let obj_str = format!("{}", obj);
                            let new_bn = s.insert_to_heap(HeapValue::String(obj_str[7..].to_string()));
                            Ok(Value::StringBox(new_bn))
                        },
                        _ => Err(format!("Operator (cast) error! Failed \
                            to cast ObjectBox {} to {} because: {}", obj_num, c, &invalid_cast_error(c))),
                    }
                }else{
                    Err(should_never_get_here_for_func("general_cast_action"))
                }
            }else{
                Err(bad_box_error("cast", "ObjectBox", "NA", obj_num, usize::MAX, false))
            }
        },

        _ => Err(format!("Operator (cast) error! Value being casted must be a castable type! Attempted value: {}", v))

    };

    res
}

//Performs all valid casts in existence wherein the top 
// of the stack tries to be casted to another data type.
fn cast_stuff(s: &mut State) -> Result<(), String>{
    if s.buffer.len() == 0{
        let res = match s.pop2(){
            (Some(v), Some(Value::StringBox(bn))) => {
                if s.validate_box(bn){
                    if let HeapValue::String(ref st) = &s.heap[bn].0{
                        //This is gross but it works.
                        // It's fine because st never changes so there's 
                        // no risk of clobbering or other memory horrors.
                        unsafe {
                            general_cast_action(&mut *(s as *const State as *mut State), v, st)
                        }
                    }else{
                        Err(should_never_get_here_for_func("cast_stuff"))
                    }
                }else{
                    Err(bad_box_error("cast", "StringBox", "NA", bn, usize::MAX, false))
                }
            },

            (Some(a), Some(b)) => {
                Err(format!("Operator (cast) error! Second to top \
                    of stack must be of type Value and a castable type, \
                    and top of stack must be of type StringBox! \
                    Attempted values: {} and {}", &a, &b))
            },
            (None, Some(_)) => Err(needs_n_args_only_n_provided("cast", "Two", "only one")),
            (None, None) => Err(needs_n_args_only_n_provided("cast", "Two", "none")),

            _ => Err(should_never_get_here_for_func("cast_stuff")),
        };

        push_val_or_err(res, s)
    }else{
        let res = match s.pop(){
            Some(v) => {
                let buff_ref: &str = &s.buffer;
                //Gross and unsafe but the buffer isn't changed so no clobbering risks.
                unsafe {
                    general_cast_action(&mut *(s as *const State as *mut State), v, buff_ref)
                }
            },
            None => Err(needs_n_args_only_n_provided("cast", "One", "none"))
        };

        push_val_or_err(res, s)
    }

}

//Creates an error string that indicates a wrong 
// given type for the various print io functions.
fn io_needing_one_item_on_stack_error(op_type: &str, needed_type: &str, attempted_value: &Value) -> String{
    format!("Operator ({}) error! Top of stack must be type {}! \
        Attempted value: {}", op_type, needed_type, attempted_value)
}

//Prints contents of a string box and consumes it. 
// Like everything else, the stringbox is not free'd.
fn print_line(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref st) = &s.heap[bn].0{
                    println!("{}", st);
                    Ok(())
                }else{
                    Err(should_never_get_here_for_func("print_line"))
                }
            }else{
                Err(bad_box_error("printLine", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(io_needing_one_item_on_stack_error("printLine", "StringBox", &v)),
        None => Err(needs_n_args_only_n_provided("printLine", "One", "none")),
    }
}

//Reads a line from stdin and allocates it as 
// a string on the heap, pushing a stringbox to the stack.
fn read_line_from_in(s: &mut State) -> Result<(), String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("READ-LINE FAILED");
    if input.ends_with("\n"){
        let _ = input.pop();
    }

    let bn = s.insert_to_heap(HeapValue::String(replace_literals_with_escapes(&input)));
    s.stack.push(Value::StringBox(bn));

    Ok(())

}

//Prints out a single char to stdout. 
// Top of stack must be a char.
fn print_char(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::Char(c)) => {
            print!("{}", c);
            Ok(())
        },
        Some(v) => Err(io_needing_one_item_on_stack_error("printChar", "Char", &v)),
        None => Err(needs_n_args_only_n_provided("printChar", "One", "none")),
    }
}

fn unable_to_read_error(op_type: &str, reason: &str) -> String{
    format!("Operator ({}) error! Unable to read from stdin because: {}", op_type, reason)
}

//Reads in one Char from stdin and pushes it to the stack.
//SEEMS TO WORK WELL ENOUGH, ASSUMING THERE'S NO CRAZY EDGE CASE THAT WOULD DESTROY THIS
fn read_char(s: &mut State) -> Result<(), String>{
    let mut buff: [u8; 1] = [0];
    let mut buff_collection: [u8; 4] = [0; 4];
    let mut buff_collection_length: usize = 0;

    //Reads until it hits a valid character.
    loop{
        //Reads one byte into the buffer and adds to the byte vec for conversion, 
        // or throws an error if there were explosions.
        match io::stdin().read(&mut buff){
            Ok(_) => {
                buff_collection[buff_collection_length] = buff[0];
                buff_collection_length += 1;
            }, 
            Err(e) => return Err(unable_to_read_error("readChar", &e.to_string())),
        }

        //Tries to convert the read in bytes to a valid utf-8 string.
        // Once it succeeds it grabs the first char from it and pushes it to the stack.
        if let Ok(st) = std::str::from_utf8(&buff_collection[0..buff_collection_length]){
            if let Some(c) = st.chars().nth(0){
                s.stack.push(Value::Char(c));
                return Ok(());
            }
        }

    }

}

//Prints contents of a string box and consumes it. 
// Like everything else, the stringbox is not free'd.
//Unlike printline, this operator doesn't append a newline character.
fn print_string(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref st) = &s.heap[bn].0{
                    print!("{}", st);
                    match io::stdout().flush(){
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("Operator (print) error! Failed \
                            to flush buffer after printing because: {}", e.to_string())),
                    }
                }else{
                    Err(should_never_get_here_for_func("print_string"))
                }
            }else{
                Err(bad_box_error("print", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(io_needing_one_item_on_stack_error("print", "StringBox", &v)),
        None => Err(needs_n_args_only_n_provided("print", "One", "none")),
    }
}

//Reads the contents of stdin into a string. Basically like readline \
// but doesn't stop reading until stdin is manually closed.
fn read_from_in(s: &mut State) -> Result<(), String>{
    let mut buff: [u8; 8192] = [0; 8192];
    let mut bytes: Vec<u8> = Vec::new();

    loop{
        match io::stdin().read(&mut buff){
            Ok(bytes_read) => {
                if bytes_read > 0{
                    bytes.extend_from_slice(&buff[0..bytes_read]);
                }else{
                    break;
                }
            },
            Err(e) => return Err(unable_to_read_error("read", &e.to_string())),
        }
    }

    match std::str::from_utf8(&bytes){
        Ok(st) => {
            let new_string = replace_literals_with_escapes(&st);
            let bn = s.insert_to_heap(HeapValue::String(new_string));
            s.stack.push(Value::StringBox(bn));
            Ok(())
        },
        Err(e) => {
            Err(format!("Operator (read) error! Unable to \
                convert input to a proper string because: {}", e))
        },
    }

}

//Prints each item on the stack while 
// also indicating if box types are valid or not.
fn debug_stack_print(s: &mut State) -> Result<(), String>{
    let filler_str = "--------------------------------";

    println!("{}", filler_str);
    println!("BEGIN STACK PRINT\n{}", filler_str);
    for item in s.stack.iter(){
        match item {
            Value::StringBox(bn) | Value::ListBox(bn) | Value::ObjectBox(bn) | Value::MiscBox(bn) => {
                //Determines if given box is valid or not and saves the value 
                // in the string which will be added to the print later on.
                let invalid_str: &str = if s.validate_box(*bn){
                    match (item, &s.heap[*bn].0){
                        (Value::StringBox(_), HeapValue::String(_)) => "",
                        (Value::ListBox(_), HeapValue::List(_)) => "",
                        (Value::ObjectBox(_), HeapValue::Object(_)) => "",
                        (Value::MiscBox(_), HeapValue::Primitive(_)) => "",
                        _ => "[INVALID]",
                    }
                }else{
                    "[INVALID]"
                };
                println!("{} {}", item, invalid_str);
            },
            i => println!("{}", i),
        }
    }

    println!("{}", filler_str);
    println!("STACK LENGTH: {}", s.stack.len());
    println!("{}\nEND STACK PRINT", filler_str); 
    println!("{}", filler_str);

    Ok(())
}

//Prints the whole heap to stdout for debugging purposes.
//This is something like O(n^2) at least so definitely only use it for debugging!
fn debug_heap_print(s: &mut State) -> Result<(), String>{
    let filler_str = "////////////////////////////////";
    
    println!("{}", filler_str);
    println!("BEGIN HEAP PRINT\n{}", filler_str);
    
    //Iterates through and prints each item on heap for debugging.
    for i in 0..(s.heap.len()){
        let box_type_str: &str = match &s.heap[i].0{
            HeapValue::String(_) => "StringBox",
            HeapValue::List(_) => "ListBox",
            HeapValue::Object(_) => "ObjectBox",
            HeapValue::Primitive(_) => "MiscBox",
        };

        //Determines if given memory cell is valid or not and prints accordingly.
        let invalid_str: &str = if s.heap[i].1{
            ""
        }else{
            " [FREE]"
        };
        println!("{} {}{}:\n\t{}", box_type_str, i, invalid_str, s.heap[i].0);
    }

    println!("{}", filler_str);
    print!("FREE'D BOX NUMBERS: [");
    for i in 0..(s.free_list.len()){
        if i < (s.free_list.len() - 1){
            print!("{}, ", s.free_list[i]);
        }else{
            print!("{}", s.free_list[i]);
        }
    }
    println!("]\n{}", filler_str);
    println!("FREE'D BOX COUNT: {}\n{}", s.free_list.len(), filler_str);
    println!("TOTAL HEAP ITEM COUNT: {}\n{}", s.heap.len(), filler_str);
    println!("PERCENT OF HEAP FREE'D: {:.2}\n{}", 
        (s.free_list.len() as f32) / (s.heap.len() as f32) * 100f32, filler_str);
    println!("END HEAP PRINT\n{}", filler_str);

    Ok(())
}

//Writes the data of one stringbox to a file with the name held in the other string box. 
// Creates a file if one doesn't exist. 
fn write_data_to_file(s: &mut State) -> Result<(), String>{
    match s.pop2(){
        (Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
            match (s.validate_box(a), s.validate_box(b)){
                (true, true) => {
                    if let (HeapValue::String(ref file_name), HeapValue::String(ref string_to_write)) = (&s.heap[a].0, &s.heap[b].0){
                        let file_path = Path::new(file_name);
                        let mut file = match OpenOptions::new().write(true).truncate(true).open(file_path){
                            Ok(f) => f,
                            Err(reason) => {
                                return Err(format!("Operator (fileWrite) error! \
                                    Unable to open file name {} \
                                    because: {}", file_name, reason.to_string()));
                            },
                        };

                        match file.write_all(string_to_write.as_bytes()){
                            Ok(_) => Ok(()),
                            Err(reason) => {
                                Err(format!("Operator (fileWrite) error! \
                                    Unable to write to {} because: {}", file_name, reason))
                            },
                        }

                    }else{
                        Err(should_never_get_here_for_func("write_data_to_file"))
                    }
                },
                (true, false) => Err(bad_box_error("fileWrite", "StringBox", "NA", a, usize::MAX, false)),
                (false, true) => Err(bad_box_error("fileWrite", "StringBox", "NA", b, usize::MAX, false)),
                (false, false) => Err(bad_box_error("fileWrite", "StringBox", "StringBox", a, b, true)),
            }
        },
        (Some(a), Some(b)) => {
            Err(format!("Operator (fileWrite) error! Second to top \
                and top of stack must both be of type StringBox! \
                Attempted values: {} and {}", &a, &b))
        },
        (None, Some(_)) => Err(needs_n_args_only_n_provided("fileWrite", "Two", "only one")),
        (None, None) => Err(needs_n_args_only_n_provided("fileWrite", "Two", "none")),
        _ => Err(should_never_get_here_for_func("write_data_to_file")),
    }
}

fn single_arg_file_io_type_error(op_type: &str, v: &Value) -> String{
    format!("Operator ({}) error! Top of stack must \
     be type StringBox! Attempted value: {}", op_type, v)
}

//Reads the contents of a file into a string and allocates it on the heap.
fn read_data_from_file(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref file_name) = &s.heap[bn].0{
                    let file_path = Path::new(file_name);
                    let mut file = match OpenOptions::new().read(true).open(file_path){
                        Ok(f) => f,
                        Err(reason) => {
                            return Err(format!("Operator (fileRead) error! Unable \
                                to open file {} because: {}", file_name, reason.to_string()));
                        },
                    };

                    let mut literal_file_string = String::new();
                    match file.read_to_string(&mut literal_file_string){
                        Ok(_) => {},
                        Err(reason) => {
                            return Err(format!("Operator (fileRead) error! Failed \
                                to read from file {} because: {}", file_name, reason));
                        },
                    }
                    
                    let file_string = replace_literals_with_escapes(&literal_file_string);
                    let new_bn = s.insert_to_heap(HeapValue::String(file_string));

                    s.stack.push(Value::StringBox(new_bn));
                    Ok(())

                }else{
                    Err(should_never_get_here_for_func("read_data_from_file"))
                }
            }else{
                Err(bad_box_error("fileRead", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(single_arg_file_io_type_error("fileRead", &v)),
        None => Err(needs_n_args_only_n_provided("fileRead", "One", "none")),
    }
}

//Creates a file with the desired name. Throws error if the file already exists.
fn create_file_based_on_string(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref file_name) = &s.heap[bn].0{
                    let file_path = Path::new(file_name);

                    //Throws error if file with given name already exists.
                    match File::open(file_path){
                        Ok(_) => {
                            return Err(format!("Operator (fileCreate) error! Unable \
                                to create file {} because it already exists!", file_name));
                        },
                        Err(_) => {},
                    }

                    match File::create(file_path){
                        Ok(_) => {},
                        Err(reason) => {
                            return Err(format!("Operator (fileCreate) error! Unable \
                                to create file {} because: {}", file_name, reason.to_string()));
                        },
                    }

                    Ok(())

                }else{
                    Err(should_never_get_here_for_func("create_file_based_on_string"))
                }
            }else{
                Err(bad_box_error("fileCreate", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(single_arg_file_io_type_error("fileCreate", &v)),
        None => Err(needs_n_args_only_n_provided("fileCreate", "One", "none")),
    }
}

//Deletes a file with the input name.
fn delete_file_based_on_string(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref file_name) = &s.heap[bn].0{
                    let file_path = Path::new(file_name);

                    match remove_file(file_path){
                        Ok(_) => {},
                        Err(reason) => {
                            return Err(format!("Operator (fileRemove) error! Unable \
                                to remove file {} because: {}", file_name, reason.to_string()));
                        },
                    }

                    Ok(())

                }else{
                    Err(should_never_get_here_for_func("delete_file_based_on_string"))
                }
            }else{
                Err(bad_box_error("fileRemove", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(single_arg_file_io_type_error("fileRemove", &v)),
        None => Err(needs_n_args_only_n_provided("fileRemove", "One", "none")),
    }
}

//Pushes a boolean based on whether or not the file exists.
fn file_exists(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref file_name) = &s.heap[bn].0{
                    let file_path = Path::new(file_name);

                    //THIS MIGHT BE TOO BROAD, WITH PERMISSIONS EDGE CASES GETTING IN THE WAY
                    let exists = match File::open(file_path){
                        Ok(_) => true,
                        Err(_) => false,
                    };

                    s.stack.push(Value::Boolean(exists));

                    Ok(())

                }else{
                    Err(should_never_get_here_for_func("file_exists"))
                }
            }else{
                Err(bad_box_error("fileExists", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(single_arg_file_io_type_error("fileExists", &v)),
        None => Err(needs_n_args_only_n_provided("fileExists", "One", "none")),
    }
}

//Consumes a value and pushes a stringbox whose contents 
// is a string that represents the type of the consumed value.
fn query_type(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(v) => {
            let type_str = type_to_string(&v);
            let bn = s.insert_to_heap(HeapValue::String(type_str));
            s.stack.push(Value::StringBox(bn));
            Ok(())
        },
        None => Err(needs_n_args_only_n_provided("queryType", "One", "none")),
    }
}

//If the top of the stack is a true boolean, the program leaves the current scope.
// This is useful for early function returns and breaking out of loops. 
fn leave_scope_if_true(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::Boolean(b)) => {
            s.leaving_scope = b;
            Ok(())
        },
        Some(v) => {
            Err(format!("Operator (leaveScopeIfTrue) error! Top of stack \
                must be of type Boolean! Attempted value: {}", &v))
        },
        None => Err(needs_n_args_only_n_provided("leaveScopeIfTrue", "One", "none")),
    }
}

//Throws an error containing a string held by a stringbox at the top of the stack.
fn throw_custom_error(s: &mut State) -> Result<(), String>{
    match s.pop(){
        Some(Value::StringBox(bn)) => {
            if s.validate_box(bn){
                if let HeapValue::String(ref err_str) = &s.heap[bn].0{
                    Err(err_str.clone())
                }else{
                    Err(should_never_get_here_for_func("throw_custom_error"))
                }
            }else{
                Err(bad_box_error("throwCustomError", "StringBox", "NA", bn, usize::MAX, false))
            }
        },
        Some(v) => Err(format!("Operator (throwCustomError) error! Top of stack \
                must be of type StringBox! Attempted value: {}", &v)),
        None => Err(needs_n_args_only_n_provided("throwCustomError", "One", "none")),
    }
}

//Fetches arguments passed to program and converts them into a list 
// of stringboxes where each stringbox contains an argument string.
//This basically is like argv in C.
fn get_args(s: &mut State) -> Result<(), String>{
    //Fetches initial args from environment.
    let all_args: Vec<String> = env::args().collect();
    
    //Creates a list which holds string boxes 
    // pointing to all the given arguments.
    let args: Vec<Value> = all_args[1..]
        .iter()
        .map(|st| 
            Value::StringBox(s.insert_to_heap(HeapValue::String(st.clone())))
        )
        .collect();

    //Inserts list of arg stringboxes into heap and pushes listbox.
    let bn = s.insert_to_heap(HeapValue::List(args));
    s.stack.push(Value::ListBox(bn));

    Ok(())
}

//Consumes top of stack and checks if it's a valid box.
fn is_valid_box(s: &mut State) -> Result<(), String>{
    let res = match s.pop(){
        Some(Value::NULLBox) => Ok(Value::Boolean(false)),
        Some(v) => {
            match v{
                Value::StringBox(bn) | Value::ListBox(bn) |
                Value::ObjectBox(bn) | Value::MiscBox(bn) => {
                    let is_valid = if s.validate_box(bn){
                        match (v, &s.heap[bn].0){
                            (Value::StringBox(_), HeapValue::String(_)) => true,
                            (Value::ListBox(_), HeapValue::List(_)) => true,
                            (Value::ObjectBox(_), HeapValue::Object(_)) => true,
                            (Value::MiscBox(_), HeapValue::Primitive(_)) => true,
                            _ => false,
                        }
                    }else{
                        false
                    };
                    Ok(Value::Boolean(is_valid))            
                },
                _ => Err(format!("Operator (isValidBox) error! \
                    Top of stack must be of type StringBox, ListBox, \
                    ObjectBox, MiscBox, or NULLBox! Attempted value: {}", &v)),         
            }
        },
        None => Err(needs_n_args_only_n_provided("isValidBox", "One", "none")),
    };

    push_val_or_err(res, s)
}

//Gets the current unix time as a 64 bit bload 
// and pushes it to the stack as such.
fn time_unix_now(s: &mut State) -> Result<(), String>{
    match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(time) => {
            let secs = time.as_secs();
            let nanos = time.subsec_nanos();

            let time_float: f64 = (secs as f64) + ((nanos as f64) / 1e9f64);
            s.stack.push(Value::Float64(time_float));
            Ok(())
        },
        Err(e) => Err(format!("Operator (timeUnixNow) error! Unable to \
            fetch the current Unix time because {}", e)),
    }
}

//Creates a frame for local variables to use.
fn create_frame(size: usize) -> Vec<(Value, bool)>{
    let mut frame: Vec<(Value, bool)> = Vec::with_capacity(size);
    for _ in 0..size{
        frame.push((Value::NULLBox, false));
    }
    frame
}

//Sets all validity booleans to false, allowing frame 
// to be reused with reletively low cost.
fn recycle_frame(frame: &mut Vec<(Value, bool)>){
    for i in 0..frame.len(){
        frame[i].1 = false;
    }
}

impl State{
    //Creates a new state.
    fn new(num_unique_var_names: usize) -> Self{
        //Fills out vec of function pointers for rapid indexing in operator function calling.
        let ops_vec: Vec<OpFunc> = vec![
            //Basic math operators.
            add, sub, mult, div, modulo, power,
            //Maximum values for each integer data type operators.
            max_isize, max_usize, 
            max_i8, max_i16, max_i32, max_i64, max_i128,
            max_u8, max_u16, max_u32, max_u64, max_u128,             
            //Stack operators.
            swap, drop, drop_stack, rot, dup, deep_dup,
            //Comparison operators.
            is_equal, is_not_equal, is_greater_than, is_less_than, is_greater_than_equal_to, 
            is_less_than_equal_to, string_compare,
            //String concatenation operator.
            concat,
            //Basic logical operators.
            and, or, xor, not,
            //List/String operations.
            list_push, list_pop, list_front_push, list_front_pop, index, length,
            is_empty, list_clear, list_contains, change_item_at,
            //Character operators
            whitespace_detect, alpha_char_detect, num_char_detect,
            //Object operators
            add_field, get_field, mut_field, remove_field,
            //Bitwise operators
            bit_or, bit_and, bit_xor, bit_not, bit_shift,
            //Casting operator
            cast_stuff,
            //IO operators
            print_line, read_line_from_in, print_char, read_char, print_string, 
            read_from_in, debug_stack_print, debug_heap_print,
            //File IO operators
            write_data_to_file, read_data_from_file, 
            create_file_based_on_string, delete_file_based_on_string, file_exists,
            query_type, leave_scope_if_true, throw_custom_error, 
            get_args, is_valid_box, time_unix_now
        ];
        
        State {
            stack: Vec::new(),
            fns: HashMap::new(),
            vars: create_frame(num_unique_var_names),
            heap: Vec::new(),
            free_list: Vec::new(),
            ops: ops_vec, 
            frames: vec![(0, create_frame(num_unique_var_names))],
            curr_frame: 0,
            frame_pool: Vec::new(),
            unique_var_name_count: num_unique_var_names,
            leaving_scope: false,
            buffer: String::with_capacity(256),
        }
    }

    //Inserts an item into the heap, 
    // returning an index to where it was inserted in the heap.
    fn insert_to_heap(&mut self, ins_val: HeapValue) -> usize{
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
        box_num < self.heap.len() && self.heap[box_num].1
    }

    //Frees a cell in a heap or does nothing if it's invalid already.
    fn free_heap_cell(&mut self, box_num: usize){
        if self.validate_box(box_num){
            self.heap[box_num].1 = false;
            self.free_list.push(box_num);
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
                if ((i + 3) < chars.len()) && (chars[i + 1] == '\\') && (chars[i + 3] == '\''){
                    tokens.push(String::from(format!("\'\\{}\'", chars[i + 2])));
                    i += 4;
                }else if ((i + 2) < chars.len()) && (chars[i + 2] == '\''){
                    tokens.push(String::from(format!("\'{}\'", chars[i + 1])));
                    i += 3;
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

    //If there was a valid token at the exact end of a file, it's picked up here.
    if curr_token.len() > 0{
        tokens.push(curr_token.iter().collect());
    }

    tokens

}

fn throw_parse_error(t: &str, attempted_token: &String){
    panic!("Parse error! Incorrectly constructed {}! Tried: {}", t, attempted_token);
}

fn replace_literals_with_escapes(s: &str) -> String{
    s
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\\"", "\"")
        .replace("\\\'", "\'")
        .replace("\\0", "\0")
}

//Given reference to list of seperated tokens, 
// differentiates each one as either a value or word.
//WARNING! OWNERSHIP TRANSFERS SO, YOU BETTER WATCH OUT!
fn lex_tokens(tokens: Vec<String>) -> Vec<Token>{
    let mut lexed: Vec<Token> = Vec::new();

    //Creates and fills out the ops map with the operators, 
    // ignoring the existing aliases for some of the operators.
    let mut ops_map: HashMap<String, usize> = HashMap::new();
    let mut i: usize = 1;
    let unique_strs = [
        "+", "-", "*", "/", "mod", "pow",
        "isizeMax", "usizeMax", 
        "i8Max", "i16Max", "i32Max", "i64Max", "i128Max",
        "u8Max", "u16Max", "u32Max", "u64Max", "u128Max", 
        "swap", "drop", "dropStack", "rot", "dup", "deepDup",
        "==", "!=", ">", "<", ">=", "<=", "stringCompare", "++",
        "and", "or", "xor", "not",
        "push", "pop", "fpush", "fpop", "index", "length", 
        "isEmpty", "clear", "contains", "changeItemAt",
        "isWhitespaceChar", "isAlphaChar", "isNumChar",
        "objAddField", "objGetField", "objMutField", "objRemField",
        "bitOr", "bitAnd", "bitXor", "bitNot", "bitShift", "cast",
        "printLine", "readLine", "printChar", "readChar", "print", 
        "read", "debugPrintStack", "debugPrintHeap",
        "fileWrite", "fileRead", "fileCreate", "fileRemove", "fileExists",
        "queryType", "leaveScopeIfTrue", "throwCustomError",
        "getArgs", "isValidBox", "timeUnixNow"
    ];
    for s in unique_strs.iter(){
        ops_map.insert(s.to_string(), i);
        i += 1;
    }

    //The following inserts add all the aliases that exist for some of the operators. 
    // The numbers given match the operation number 
    // of the appropriate previously inserted operation.

    //Alias for mod
    ops_map.insert("%".to_string(), *(ops_map.get("mod").unwrap()));

    //Alises for logical AND, OR, and NOT
    ops_map.insert("&&".to_string(), *(ops_map.get("and").unwrap()));
    ops_map.insert("||".to_string(), *(ops_map.get("or").unwrap()));
    ops_map.insert("!".to_string(), *(ops_map.get("not").unwrap()));

    //Aliases for push, pop, fpush, fpop, and length
    ops_map.insert("p".to_string(), *(ops_map.get("push").unwrap()));
    ops_map.insert("po".to_string(), *(ops_map.get("pop").unwrap()));
    ops_map.insert("fp".to_string(), *(ops_map.get("fpush").unwrap()));
    ops_map.insert("fpo".to_string(), *(ops_map.get("fpop").unwrap()));
    ops_map.insert("len".to_string(), *(ops_map.get("length").unwrap()));

    //Aliases for bitOr, bitAnd, and bitXor
    ops_map.insert("|".to_string(), *(ops_map.get("bitOr").unwrap()));
    ops_map.insert("&".to_string(), *(ops_map.get("bitAnd").unwrap()));
    ops_map.insert("^".to_string(), *(ops_map.get("bitXor").unwrap()));

    for tok in tokens.into_iter(){
        match tok{
            //Boolean lexing cases.
            ref t if t == "True" || t == "true" => {
                lexed.push(Token::V(
                    SuperValue::Reg(Value::Boolean(true)))
                );
            },
            ref t if t == "False" || t == "false" => {
                lexed.push(Token::V(
                    SuperValue::Reg(Value::Boolean(false)))
                );
            },
            //String case.
            ref t if t.starts_with("\"") && t.ends_with("\"") => {
                lexed.push(Token::V(
                    SuperValue::Heap(
                        HeapValue::String(
                            replace_literals_with_escapes(&tok[1..(tok.len() - 1)]))
                    ))
                );
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
                        '\'' => '\'',
                        '\"' => '\"',
                        _ => captured,
                    };
                }
                lexed.push(Token::V(SuperValue::Reg(Value::Char(captured))));
            },
            //List case.
            ref t if t == "[]" => lexed.push(Token::V(SuperValue::Heap(HeapValue::List(Vec::new())))),
            //Object case.
            ref t if t == "{}" => lexed.push(Token::V(SuperValue::Heap(HeapValue::Object(HashMap::new())))),
            //Float cases.
            ref t if t.ends_with("f32") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<f32>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Float32(parsed)))),
                    Err(_) => throw_parse_error("f32", &tok),
                }
            },
            ref t if t.ends_with("f64") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<f64>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Float64(parsed)))),
                    Err(_) => throw_parse_error("f64", &tok), 
                }
            },
            //Explicit integer cases for both signed and unsigned.
            ref t if t.ends_with("u8") && t.len() > 2 => {
                match tok[0..(tok.len() - 2)].parse::<u8>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt8(parsed)))),
                    Err(_) => throw_parse_error("u8", &tok), 
                }
            },
            ref t if t.ends_with("i8") && t.len() > 2 => {
                match tok[0..(tok.len() - 2)].parse::<i8>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int8(parsed)))),
                    Err(_) => throw_parse_error("i8", &tok), 
                }
            },
            ref t if t.ends_with("u16") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<u16>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt16(parsed)))),
                    Err(_) => throw_parse_error("u16", &tok),
                }
            },
            ref t if t.ends_with("i16") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<i16>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int16(parsed)))),
                    Err(_) => throw_parse_error("i16", &tok), 
                }
            },
            ref t if t.ends_with("u32") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<u32>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt32(parsed)))),
                    Err(_) => throw_parse_error("u32", &tok), 
                }
            },
            ref t if t.ends_with("i32") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<i32>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int32(parsed)))),
                    Err(_) => throw_parse_error("i32", &tok), 
                }
            },
            ref t if t.ends_with("u64") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<u64>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt64(parsed)))),
                    Err(_) => throw_parse_error("u64", &tok), 
                }
            },
            ref t if t.ends_with("i64") && t.len() > 3 => {
                match tok[0..(tok.len() - 3)].parse::<i64>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int64(parsed)))),
                    Err(_) => throw_parse_error("i64", &tok), 
                }
            },
            ref t if t.ends_with("u128") && t.len() > 4 => {
                match tok[0..(tok.len() - 4)].parse::<u128>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt128(parsed)))),
                    Err(_) => throw_parse_error("u128", &tok), 
                }
            },
            ref t if t.ends_with("i128") && t.len() > 4 => {
                match tok[0..(tok.len() - 4)].parse::<i128>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int128(parsed)))),
                    Err(_) => throw_parse_error("i128", &tok), 
                }
            },
            ref t if t.ends_with("usize") && t.len() > 5 => {
                match tok[0..(tok.len() - 5)].parse::<usize>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UIntSize(parsed)))),
                    Err(_) => throw_parse_error("usize", &tok), 
                }
            },
            ref t if t.ends_with("isize") && t.len() > 5 => {
                match tok[0..(tok.len() - 5)].parse::<isize>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::IntSize(parsed)))),
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
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Float32(parsed)))),
                    Err(_) => throw_parse_error("f32", &tok),
                }
            },
            //Type inference for integer.
            ref t if (t.chars().next().unwrap() == '-' && t.len() > 1) 
                    || (t.chars().next().unwrap() >= '0' 
                        && t.chars().next().unwrap() <= '9') 
                    => {
                match tok.parse::<isize>(){
                    Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::IntSize(parsed)))),
                    Err(_) => throw_parse_error("isize", &tok),
                }
            },

            //General catch-all case mostly meant for operators.
            _ => {
                let n: usize = *ops_map.get(&tok).unwrap_or(&0);
                lexed.push(Token::Word((tok, n)));
            }, 
        }
    }

    lexed
}

//This function does the heavy-lifting of recursively building the AST.
fn make_ast_prime(
    mut already_parsed: Vec<ASTNode>, 
    tokens: Vec<Token>, 
    token_index: usize,
    loc_nums: &mut HashMap<String, usize>,
    curr_loc_num: &mut usize, 
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
            //Stop on terminator case. (THIS MIGHT EXPLODE DUE TO THE RECONFIGURED WORD TOKENS)
            ref tok if terminators.contains(tok) => (already_parsed, tokens, token_index + 1, Some(token_index)),
            //Parse if statement case.
            Token::Word(ref cmd) if cmd.0 == "if" => {
                let (true_branch, false_branch, tokens_prime, token_index_prime) = parse_if(tokens, token_index + 1, loc_nums, curr_loc_num);
                already_parsed.push(ASTNode::If{if_true : Box::new(true_branch), if_false : Box::new(false_branch)});
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators) 
            },
            //While loop parsing case.
            Token::Word(ref cmd) if cmd.0 == "while" => {
                let (loop_body, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]);
                already_parsed.push(ASTNode::While(Box::new(ASTNode::Expression(loop_body))));
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
            },
            //Function case.
            Token::Word(ref cmd) if cmd.0 == "func" => {
                //Makes sure there's enough stuff to look to parse the function.
                if token_index + 2 > tokens.len(){
                    panic!("Insufficient tokens left for function to be parsed!");
                }

                let mut toks = tokens;
                let command = std::mem::take(&mut toks[token_index + 1]);
                let name = std::mem::take(&mut toks[token_index + 2]);
                let (command_str, name_str) = match (command, name){
                    (Token::Word(c), Token::Word(n)) => (c.0, n.0),
                    (_, _) => panic!("SHOULD NEVER GET HERE!!!"),
                };

                let (fbod, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), toks, token_index + 3, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]);

                let fbod_ast = Rc::new(ASTNode::Expression(fbod));

                already_parsed.push(
                    ASTNode::Function{func_cmd: command_str, func_name: name_str, func_bod: fbod_ast});
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)

            },
            //Var command parsing case.
            Token::Word(ref cmd) if cmd.0 == "var" => {
                let (mut var_data, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]);
                if var_data.len() >= 2{
                    let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
                        (ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c.0, n.0),
                        (_, _) => {panic!("Malformed variable command Error! \
                            Insufficient parameters given for variable command!")},
                    };
                    let vn: usize = match loc_nums.get(&name){
                        Some(n) => *n,
                        None => {
                            loc_nums.insert(name.clone(), *curr_loc_num);
                            let ret = *curr_loc_num;
                            *curr_loc_num += 1;
                            ret
                        },
                    };
                    already_parsed.push(ASTNode::Variable{var_name: name, var_cmd: cmd, var_num: vn});
                    make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)

                }else{
                    panic!("Malformed variable command Error! \
                        Insufficient parameters given for variable command!");
                }
            },
            //Loc command parsing case.
            Token::Word(ref cmd) if cmd.0 == "loc" => {
                let (mut var_data, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]);
                if var_data.len() >= 2{
                    let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
                        (ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c.0, n.0),
                        (_, _) => {panic!("Malformed local variable command Error! \
                            Insufficient parameters given for local variable command!")},
                    };
                    let var_num: usize = match loc_nums.get(&name){
                        Some(n) => *n,
                        None => {
                            loc_nums.insert(name.clone(), *curr_loc_num);
                            let ret = *curr_loc_num;
                            *curr_loc_num += 1;
                            ret
                        },
                    };
                    already_parsed.push(ASTNode::LocVar{name: name, cmd: cmd, num: var_num});
                    make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)

                }else{
                    panic!("Malformed local variable command Error! \
                        Insufficient parameters given for local variable command!");
                }
            },
            //Box command case.
            Token::Word(ref cmd) if cmd.0 == "box" => {
                let (mut box_data, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]);
                if box_data.len() >= 1{
                    let box_cmd = match std::mem::take(&mut box_data[0]){
                        ASTNode::Terminal(Token::Word(c)) => c.0,
                        _ => panic!("Malformed box command!"),
                    };

                    already_parsed.push(ASTNode::BoxOp(box_cmd));
                    make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
                }else{
                    panic!("Malformed box command! No box command token given!")
                }
            },
            //Attempt onError case.
            Token::Word(ref cmd) if cmd.0 == "attempt" => {
                let (att_branch, err_branch, tokens_prime, token_index_prime) = 
                    parse_att_err(tokens, token_index + 1, loc_nums, curr_loc_num);
                already_parsed.push(ASTNode::AttErr{attempt: Box::new(att_branch), err: Box::new(err_branch)});
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
            },
            //Defer case.
            Token::Word(ref cmd) if cmd.0 == "defer" => {
                let (defer_body, tokens_prime, token_index_prime, _) = 
                    make_ast_prime(
                        Vec::new(),
                        tokens, 
                        token_index + 1, 
                        loc_nums,
                        curr_loc_num,
                        vec![Token::Word((";".to_string(), 0))]
                    ); 
                already_parsed.push(ASTNode::Defer(Rc::new(ASTNode::Expression(defer_body))));
                make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
            },
            _ => {
                let mut toks = tokens;
                already_parsed.push(ASTNode::Terminal(std::mem::take(&mut toks[token_index])));
                make_ast_prime(already_parsed, toks, token_index + 1, loc_nums, curr_loc_num, terminators)
            },
        }
    }

}

//Used to recursively parse an attempt branch for AttErr
fn parse_att_err(
    tokens: Vec<Token>,
    token_index: usize, 
    loc_nums: &mut HashMap<String, usize>,
    curr_loc_num: &mut usize) -> (ASTNode, ASTNode, Vec<Token>, usize){
    let (att_branch, tokens_prime, token_index_prime, terminator_index) = 
        make_ast_prime(
            Vec::new(),
            tokens, 
            token_index, loc_nums, curr_loc_num,
            vec![Token::Word(("onError".to_string(), 0))]
        );
    match terminator_index{
        Some(i) => {
            match tokens_prime[i]{
                Token::Word(ref cmd) if cmd.0 == "onError" => {
                    let (error_branch, tokens_prime_prime, token_index_prime_prime, _) = 
                        make_ast_prime(
                            Vec::new(),
                            tokens_prime,
                            token_index_prime, 
                            loc_nums,
                            curr_loc_num,
                            vec![Token::Word((";".to_string(), 0))]
                        );
                    (ASTNode::Expression(att_branch), ASTNode::Expression(error_branch), 
                        tokens_prime_prime, token_index_prime_prime)
                },
                _ => panic!("Failed to correctly construct attempt onError block!"),
            }
        },
        None => panic!("REALLY SHOULD NEVER GET HERE!")
    }
}

fn parse_if(
    tokens: Vec<Token>, 
    token_index: usize, 
    loc_nums: &mut HashMap<String, usize>, 
    curr_loc_num: &mut usize) -> (ASTNode, ASTNode, Vec<Token>, usize){
    let (true_branch, tokens_prime, token_index_prime, terminator_index) = 
        make_ast_prime(
            Vec::new(), 
            tokens, 
            token_index, loc_nums, curr_loc_num, 
            vec![Token::Word(("else".to_string(), 0)), Token::Word((";".to_string(), 0))]
        );
    match terminator_index{
        Some(i) => {
            match tokens_prime[i]{
                Token::Word(ref cmd) if cmd.0 == "else" => {
                    let (false_branch, tokens_prime_prime, token_index_prime_prime) = 
                        parse_else(tokens_prime, token_index_prime, loc_nums, curr_loc_num);
                    (ASTNode::Expression(true_branch), false_branch, 
                        tokens_prime_prime, token_index_prime_prime)
                },
                _ => (ASTNode::Expression(true_branch), ASTNode::Expression(vec![]), tokens_prime, token_index_prime),  
            }
        },
        _ => panic!("SHOULD NEVER GET HERE!!!"),
    }
}

fn parse_else(
    tokens: Vec<Token>, 
    token_index: usize, 
    loc_nums: &mut HashMap<String, usize>,
    curr_loc_num: &mut usize) -> (ASTNode, Vec<Token>, usize){
    let (if_false, tokens_prime, token_index_prime, _) = 
        make_ast_prime(
            Vec::new(),
            tokens, 
            token_index, loc_nums, curr_loc_num,
            vec![Token::Word((";".to_string(), 0))]
        );
    (ASTNode::Expression(if_false), tokens_prime, token_index_prime)
}

//Consumes a vec of tokens and generates an Abstract Syntax Tree (AST) from it,
// returning it for the program to then run. 
// It also returns the number of unique local variable names for later use in running the program. 
fn make_ast(tokens: Vec<Token>) -> (ASTNode, usize){
    let mut loc_nums: HashMap<String, usize> = HashMap::new();
    let mut curr_loc_num: usize = 0;
    (ASTNode::Expression(make_ast_prime(Vec::new(), tokens, 0,
     &mut loc_nums, &mut curr_loc_num, Vec::new()).0), curr_loc_num)
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

//Error string for when var mak and var mut 
// don't have anything on the stack for them.
fn variable_lack_of_args_error(var_action: &str) -> String{
    format!("Variable (var) error! Variable {} needs one \
        item on the stack! None provided!", var_action)
}

//Error string for when loc mak and loc mut 
// don't have anything on the stack for them.
fn local_variable_lack_of_args_error(var_action: &str) -> String{
    format!("Local Variable (loc) error! Local variable {} needs one \
        item on the stack! None provided!", var_action)
}

//Frees a box on the heap or kicks back an error.
fn box_free_func(s: &mut State, v: Value, box_num: usize) -> Result<(), String>{
    if s.validate_box(box_num){
        s.free_heap_cell(box_num);
        Ok(())
    }else{
        Err(format!("Box free error! {} is invalid due \
            to having already been free'd!", &v))
    }
}

//Adds one to the current frame count. 
// This means that any local variables would be created a frame deeper than before.
fn add_frame(s: &mut State){
    s.curr_frame += 1
}

//Removes hashmap for local variables in current frame before leaving, 
// unless at global scope where nothing happens.
fn remove_frame(s: &mut State){
    if s.curr_frame > 0{
        if s.frames[s.frames.len() - 1].0 == s.curr_frame{
            let mut popped = s.frames.pop().unwrap();
            s.frame_pool.push(std::mem::take(&mut popped.1));
        }
        s.curr_frame -= 1;
    }
}

//Removes the stack frame before then creating the appropriate error string.
fn error_and_remove_frame(s: &mut State, err: String) -> Result<bool, String>{
    remove_frame(s);
    Err(err)
}

//Finds frame index of frame containing desired variable if found.
fn find_var(s: &mut State, num: usize) -> Option<usize>{
    for i in (0..(s.frames.len())).rev(){
        if s.frames[i].1[num].1{
            return Some(i);
        }
    }
    None
}

//Iterates over vec of deferred code backwards 
// to replicate stack behavior, 
fn run_deferred(s: &mut State, deferred: Vec<Rc<ASTNode>>) -> Result<(), String>{
    for code in deferred.iter().rev(){
        add_frame(s);
        match run_program(code, s){
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

//Iterates recursively through the AST and effectively runs the program doing so.
fn run_program(ast: &ASTNode, state: &mut State) -> Result<bool, String>{
    let mut deferred: Option<Vec<Rc<ASTNode>>> = None;

    match ast{
        ASTNode::Expression(nodes) => {
            for node in nodes.iter(){
                match node{
                    ASTNode::Terminal(Token::V(v)) => {
                        match v{
                            SuperValue::Heap(HeapValue::String(s)) => {
                                let new_bn = state.insert_to_heap(HeapValue::String(s.clone()));
                                state.stack.push(Value::StringBox(new_bn));
                            },
                            SuperValue::Heap(HeapValue::List(l)) => {
                                let new_bn = state.insert_to_heap(HeapValue::List(l.clone()));
                                state.stack.push(Value::ListBox(new_bn));
                            },
                            SuperValue::Heap(HeapValue::Object(o)) => {
                                let new_bn = state.insert_to_heap(HeapValue::Object(o.clone()));
                                state.stack.push(Value::ObjectBox(new_bn));
                            },
                            SuperValue::Reg(reg_val) => state.stack.push(reg_val.clone()),
                            _ => return error_and_remove_frame(state, 
                                should_never_get_here_for_func("pushing to stack in run program")),
                        }
                    },
                    ASTNode::Terminal(Token::Word((ref op, ref n))) => {
                        if *n > 0{
                            //Runs operator at index equivalent to a valid operator name.
                            match state.ops[n - 1](state){
                                Ok(_) => (),
                                Err(e) => return error_and_remove_frame(state, e),
                            }
                            //Leaves current scope if necessary, running 
                            // all deferred code that's been encountered and leaves the scope.
                            if state.leaving_scope{
                                state.leaving_scope = false;

                                //Runs deferred code if any has been deferred in scope.
                                if let Some(def) = deferred{
                                    match run_deferred(state, def){
                                        Ok(_) => (),
                                        Err(e) => return error_and_remove_frame(state, e),
                                    }
                                }

                                remove_frame(state);
                                return Ok(true);
                            }
                        }else{
                            return error_and_remove_frame(state, format!("Unrecognized Operator: {}", op));
                        } 
                    },
                    ASTNode::Variable{var_name: name, var_cmd: cmd, var_num: num} => {
                        match cmd as &str{
                            "mak" => {
                                if !state.vars[*num].1{
                                    match state.pop(){
                                        Some(v) => {
                                            state.vars[*num].0 = v;
                                            state.vars[*num].1 = true;
                                        },
                                        None => {
                                            return error_and_remove_frame(state, 
                                                variable_lack_of_args_error("creation (mak)"));
                                        },
                                    }
                                }else{
                                    return error_and_remove_frame(state, format!("Variable creation (var mak) \
                                        error! Variable {} already exists! \
                                        Try deleting it using del!", &name));
                                }
                            },
                            "get" => {
                                if state.vars[*num].1{
                                    state.stack.push(state.vars[*num].0.clone());
                                }else{
                                    return error_and_remove_frame(state, format!("Variable get (var get) error! \
                                        Variable {} doesn't exist. \
                                        Try making it first using var mak!", &name));
                                }
                            },
                            "mut" => {
                                if state.vars[*num].1{
                                    match state.pop(){
                                        Some(new_v) => {
                                            let old_v: &mut Value = &mut state.vars[*num].0;
                                            if is_valid_mutation(old_v, &new_v){
                                                *old_v = new_v;
                                            }else{
                                                let mut_err = invalid_mutation_error("var mut", 
                                                    "variable", name, &old_v, &new_v); 
                                                return error_and_remove_frame(state, mut_err);
                                            }
                                        },
                                        None => return error_and_remove_frame(state, 
                                            variable_lack_of_args_error("mutation (mut)")),
                                    }
                                }else{
                                    return error_and_remove_frame(state, 
                                        format!("Variable mutation (var mut) error! Variable {} doesn't exist. \
                                        Try making it first using var mak!", &name));
                                }
                            },
                            "del" => {
                                //Marks given variable as invalid, allowing slot 
                                // to be reused by var of the same number. 
                                //Otherwise, throws error.
                                if state.vars[*num].1{
                                    state.vars[*num].1 = false;
                                }else{ 
                                    return error_and_remove_frame(state, 
                                        format!("Variable deletion (var del) error! \
                                        Variable {} doesn't exist or was already deleted! \
                                        Try making it first using var mak!", &name));
                                }
                            },
                            c => {
                                return error_and_remove_frame(state, format!("Variable (var) error! \
                                    Unrecognized variable command! Valid: mak, get, mut, del . \
                                    Attempted: {}", c));
                            },
                        }
                    },
                    ASTNode::BoxOp(box_op) => {
                        match &box_op as &str{
                            "free" => {
                                match state.stack.pop(){
                                    Some(Value::StringBox(bn)) => {
                                        match box_free_func(state, Value::StringBox(bn), bn){
                                            Ok(_) => {},
                                            Err(e) => return error_and_remove_frame(state, e),
                                        }
                                    },
                                    Some(Value::ListBox(bn)) => {
                                        match box_free_func(state, Value::ListBox(bn), bn){
                                            Ok(_) => {},
                                            Err(e) => return error_and_remove_frame(state, e),
                                        }
                                    },
                                    Some(Value::ObjectBox(bn)) => {
                                        match box_free_func(state, Value::ObjectBox(bn), bn){
                                            Ok(_) => {},
                                            Err(e) => return error_and_remove_frame(state, e),
                                        }
                                    },
                                    Some(Value::MiscBox(bn)) => {
                                        match box_free_func(state, Value::MiscBox(bn), bn){
                                            Ok(_) => {},
                                            Err(e) => return error_and_remove_frame(state, e),
                                        }
                                    },
                                    Some(v) => {
                                        return error_and_remove_frame(state, 
                                            format!("Box free error! Top of stack must be of type StringBox, \
                                            ListBox, ObjectBox, or MiscBox! Attempted value: {}", &v));
                                    },

                                    None => {
                                        return error_and_remove_frame(state, 
                                            needs_n_args_only_n_provided("box free", "One", "none"));
                                    },

                                }
                            },
                            "null" => {
                                state.stack.push(Value::NULLBox);
                            },
                            "make" => {
                                match state.stack.pop(){
                                    Some(v) => {
                                        let new_bn = state.insert_to_heap(HeapValue::Primitive(v));
                                        state.stack.push(Value::MiscBox(new_bn));
                                    },
                                    None => return error_and_remove_frame(state, 
                                        needs_n_args_only_n_provided("box make", "One", "none")),
                                }
                            },
                            "open" => {
                                match state.stack.pop(){
                                    Some(Value::MiscBox(bn)) => {
                                        if state.validate_box(bn){
                                            if let HeapValue::Primitive(ref v) = &state.heap[bn].0{
                                                state.stack.push(v.clone());    
                                            }else{
                                                return error_and_remove_frame(state, 
                                                    should_never_get_here_for_func("box open (run program)"));
                                            }
                                        }else{
                                            return error_and_remove_frame(state, 
                                                bad_box_error("box open", "MiscBox", "NA", 
                                                    bn, usize::MAX, false));
                                        }
                                    },
                                    Some(v) => {
                                        return error_and_remove_frame(state, 
                                            format!("Box open error!\
                                             Top of stack must be type MiscBox! \
                                            Attempted value: {}", &v));
                                    },
                                    None => return error_and_remove_frame(state, 
                                        needs_n_args_only_n_provided("box open", "One", "none")),
                                }
                            },
                            "altr" => {
                                match state.pop2(){
                                    (Some(Value::MiscBox(bn)), Some(v)) => {
                                        if state.validate_box(bn){
                                            if let HeapValue::Primitive(ref mut old_v) = &mut state.heap[bn].0{
                                                if is_valid_mutation(old_v, &v){
                                                    *old_v = v;
                                                    state.stack.push(Value::MiscBox(bn));
                                                }else{
                                                    let old_v_cloned = old_v.clone();
                                                    return error_and_remove_frame(state, 
                                                        invalid_mutation_error("box altr", 
                                                        "MiscBox", &bn.to_string(), &old_v_cloned, &v));
                                                }
                                            }else{
                                                return error_and_remove_frame(state, 
                                                    should_never_get_here_for_func("box altr (run program)"));
                                            }
                                        }else{
                                            return error_and_remove_frame(state, 
                                                bad_box_error("box altr", "MiscBox", 
                                                    "NA", bn, usize::MAX, false));
                                        }
                                    },
                                    (Some(a), Some(b)) => {
                                        return error_and_remove_frame(state, 
                                            format!("Box altr error! Second to top of stack \
                                            must be type MiscBox and top of stack type Value! \
                                            Attempted values: {} and {}", &a, &b));
                                    },
                                    (None, Some(_)) => return error_and_remove_frame(state, 
                                        needs_n_args_only_n_provided("box altr", "Two", "only one")),
                                    (None, None) => return error_and_remove_frame(state, 
                                        needs_n_args_only_n_provided("box altr", "Two", "none")),
                                    _ => return error_and_remove_frame(state, 
                                        should_never_get_here_for_func("box altr")),
                                }
                            },
                            o => {
                                return error_and_remove_frame(state, 
                                    format!("Box error! Unrecognized box operation! \
                                    Valid: free, null, make, open, altr . Attempted: {}", o));
                            },
                        }
                    },
                    ASTNode::If{if_true: true_branch, if_false: false_branch} => {
                        match state.pop(){
                            Some(Value::Boolean(b)) => {
                                add_frame(state);
                                let res = match b{
                                    true => run_program(&true_branch, state),
                                    false => run_program(&false_branch, state),
                                };

                                match res{
                                    Ok(_) => {},
                                    Err(e) => return error_and_remove_frame(state, e),
                                }
                            },
                            Some(v) => {
                                return error_and_remove_frame(state, format!("If statement error! \
                                    Top of stack needs to be type Boolean \
                                    for effective branching to occur! \
                                    Attempted value: {}", &v));
                            },
                            None => return error_and_remove_frame(state, 
                                needs_n_args_only_n_provided("if", "One", "none")),
                        }
                    },
                    ASTNode::While(bod) => {
                        loop {
                            match state.pop(){
                                Some(Value::Boolean(b)) => {
                                    if b{
                                        add_frame(state);
                                        match run_program(&bod, state){
                                            Ok(left_early) => {
                                                if left_early{
                                                    break;
                                                }
                                            },
                                            Err(e) => return error_and_remove_frame(state, e),
                                        }
                                    }else{
                                        break;
                                    }
                                },
                                Some(v) => {
                                    return error_and_remove_frame(state, 
                                        format!("While loop error! Top of stack needs \
                                        to be of type Boolean to determine if loop needs \
                                        to run/run again! Attempted value: {}", &v));
                                },
                                None => return error_and_remove_frame(state, 
                                    needs_n_args_only_n_provided("while", "One", "none")),
                            }
                        }
                    },
                    ASTNode::Function{func_cmd: cmd, func_name: name, func_bod: bod} => {
                        match &cmd as &str{
                            "def" => {
                                match state.fns.get(name){
                                    Some(_) => {
                                        return error_and_remove_frame(state, 
                                            format!("Function definition (func def) error! \
                                            Function \"{}\" is already defined!", &name));
                                    },
                                    None => {
                                        state.fns.insert(name.clone(), Rc::clone(&bod));
                                    },
                                }
                            },
                            "call" => {
                                let func_body = match state.fns.get(name){
                                    Some(b) => b,
                                    None => {
                                        return error_and_remove_frame(state, 
                                            format!("Function call (func call) error! \
                                            Function \"{}\" is not defined! \
                                            Try defining it using func def !", name));
                                    }, 
                                };

                                //This gross blob of unsafe code makes function calls work.
                                //It's okay though because even though state changes, func_body never will, 
                                // so it's safe despite the borrow checker's complaints.
                                unsafe {
                                    add_frame(&mut *(state as *const State as *mut State));
                                    match run_program(func_body, &mut *(state as *const State as *mut State)){
                                        Ok(_) => {},
                                        Err(e) => return error_and_remove_frame(state, e),
                                    }
                                }

                            },
                            c => {
                                return error_and_remove_frame(state, 
                                    format!("Function error! Invalid function \
                                    command given! Valid: def, call . Attempted: {}", c));
                            },
                        }
                    },
                    ASTNode::LocVar{name: nam, cmd: c, num: n} => {
                        match c as &str{
                            "mak" => {
                                match state.pop(){
                                    Some(v) => {
                                        let last_index: usize = state.frames.len() - 1;
                                        if state.frames[last_index].0 == state.curr_frame{
                                            //If not valid Overwrites garbage value held at memory cell 
                                            // with value from stack and sets it to valid, 
                                            // making it an accessable variable.
                                            //Otherwise, throws error because cell is already taken, 
                                            // meaning variable exists.
                                            if !state.frames[last_index].1[*n].1{
                                                state.frames[last_index].1[*n].0 = v;
                                                state.frames[last_index].1[*n].1 = true;
                                            }else{
                                                return error_and_remove_frame(state, format!("Local Variable \
                                                    creation (loc mak) \
                                                    error! Local Variable {} already exists in given scope!", &nam));
                                            }
                                        }else{
                                            let mut new_frame: Vec<(Value, bool)> = if state.frame_pool.len() > 0{
                                                let mut new = state.frame_pool.pop().unwrap();
                                                recycle_frame(&mut new);
                                                new
                                            }else{
                                                create_frame(state.unique_var_name_count)    
                                            };
                                            new_frame[*n].0 = v;
                                            new_frame[*n].1 = true;
                                            state.frames.push((state.curr_frame, new_frame));
                                        }
                                    },
                                    None => {
                                        return error_and_remove_frame(state, 
                                            local_variable_lack_of_args_error("creation (mak)"));
                                    },
                                }
                            },
                            "get" => {
                                match find_var(state, *n){
                                    Some(frame_index) => {
                                        state.stack.push(state.frames[frame_index].1[*n].0.clone());
                                    },
                                    None => {
                                        return error_and_remove_frame(state, format!("\
                                            Local Variable get (loc get) error! \
                                            Local variable {} doesn't exist in any scope! \
                                            Try making it using loc mak!", nam));
                                    },
                                }
                            },
                            "mut" => {
                                match state.stack.pop(){
                                    Some(v) => {
                                        match find_var(state, *n){
                                            Some(frame_index) => {
                                                let old_v: &mut Value = &mut state.frames[frame_index].1[*n].0;
                                                if is_valid_mutation(old_v, &v){
                                                    *old_v = v;
                                                }else{
                                                    let mut_err = invalid_mutation_error("loc mut", 
                                                        "local variable", nam, &old_v, &v); 
                                                    return error_and_remove_frame(state, mut_err);
                                                }
                                            },
                                            None => {
                                                return error_and_remove_frame(state, format!("\
                                                    Local Variable mutation (loc mut) error! \
                                                    Local variable {} doesn't exist in any scope! \
                                                    Try making it using loc mak!", nam));
                                            },
                                        }
                                    },
                                    None => {
                                        return error_and_remove_frame(state, 
                                            local_variable_lack_of_args_error("mutation (mut)"))
                                    },
                                }
                            },
                            misc => {
                                return error_and_remove_frame(state, format!("Local Variable (loc) error! \
                                    Unrecognized local variable command! Valid: mak, get, mut . \
                                    Attempted: {}", misc));
                            },
                        }
                    },
                    ASTNode::AttErr{attempt: att, err: e} => {
                        add_frame(state);
                        match run_program(att, state){
                            Ok(_) => (),
                            Err(e1) => {
                                //Pushes error string to stack instead of returning it up stack. 
                                let err_bn = state.insert_to_heap(HeapValue::String(e1));
                                state.stack.push(Value::StringBox(err_bn));

                                //Recursively runs error code block and explodes if there's a problem.
                                add_frame(state);
                                match run_program(e, state){
                                    Ok(_) => (),
                                    Err(e2) => return error_and_remove_frame(state, e2),
                                }
                            },
                        }
                    },
                    ASTNode::Defer(body) => {
                        if let Some(ref mut def) = deferred{
                            def.push(Rc::clone(body));
                        }else{
                            deferred = Some(vec![Rc::clone(body)]);
                        }
                    },
                    _ => {},
                }
            }
        },
        _ => {return Err("Should never get to this point!".to_string());},
    }

    //Runs deferred code if any has been deferred.
    if let Some(def) = deferred{
        match run_deferred(state, def){
            Ok(_) => (),
            Err(e) => return error_and_remove_frame(state, e),
        }
    }

    remove_frame(state);
    Ok(false)
}

fn main(){
    //Creates argv and argc for finding file paths and stuff.
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    
    //Used to hold a string read in from an input file or stdin directly.
    let mut file_string = String::new();

    //Reads in data from file or from stdin, 
    // depending on inputs or lack thereof.
    if argc > 1{
        let file_path = Path::new(&argv[1]);
        let file_name = file_path.display();

        let mut code_file = match File::open(&file_path){
            Ok(f) => f,
            Err(reason) => panic!("Unable to open Lmao file {} for parsing because {}", file_name, reason),
        };

        match code_file.read_to_string(&mut file_string){
            Ok(_) => {},
            Err(reason) => panic!("Unable to read Lmao file {} because {}", file_name, reason),
        }
    }else{
        let sep_str = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
        println!("Enter Lmao code below:\n{}", sep_str);
        io::stdin().read_to_string(&mut file_string)
            .expect("Stdin read error! Failed to read from stdin!");
        println!("\n{}\nProgram result:\n", sep_str);
    }

    let file_chars: Vec<char> = file_string.chars().collect();
    let tokens = tokenize(&file_chars);

    let lexed = lex_tokens(tokens);

    let (ast, num_unique_loc_vars) = make_ast(lexed);

    let mut state = State::new(num_unique_loc_vars);

    let result = run_program(&ast, &mut state);

    match result{
        Ok(_) => {},
        Err(e) => println!("{}", e),
    }

}
