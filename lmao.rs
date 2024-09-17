//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.1.1

use std::collections::HashMap;

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

enum Token{
    V(Value),
    Cmd(String)
}

enum ASTNode{
    Terminal(Token),
    If {ifTrue: Box<ASTNode>, ifFalse: Box<ASTNode>},
    While(Box<ASTNode>),
    Expression(Vec<ASTNode>),
    Function{funcCmd: String, funcName: String, funcBod: Box<ASTNode>},
    Variable{varName: String, varCmd: String},
    LocVar{name: String, cmd: String},
    BoxOp(String)
}

fn main(){
    
}
