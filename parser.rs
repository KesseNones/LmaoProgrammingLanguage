use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::mem::discriminant;

//This pub enum is used to contain all the possible data types of Lmao 
// that live everywhere but the Heap.
#[derive(PartialEq, Clone, Copy, PartialOrd)]
pub enum Value{
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

//Determines if a translation from one value type to another is valid. 
// Typically the types have to match unless it's nullbox to box stuff.
pub fn is_valid_mutation(a: Value, b: Value) -> bool{
	if discriminant(&a) == discriminant(&b){
		true
	}else{
		match (a, b){
			(Value::NULLBox, Value::StringBox(_)) => true,
			(Value::StringBox(_), Value::NULLBox) => true,
			(Value::NULLBox, Value::ListBox(_)) => true,
			(Value::ListBox(_), Value::NULLBox) => true,
			(Value::NULLBox, Value::ObjectBox(_)) => true,
			(Value::ObjectBox(_), Value::NULLBox) => true,
			(Value::NULLBox, Value::MiscBox(_)) => true,
			(Value::MiscBox(_), Value::NULLBox) => true,
			_ => false,
		}	
	}
}

impl Eq for Value {}

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
				if flt32.abs() <= 1000000.0 && flt32.abs() >= 0.000001{
					write!(f, "f32 {}", flt32)
				}else{
					write!(f, "f32 {:e}", flt32)
				}
			},
			Value::Float64(flt64) => {
				if flt64.abs() <= 1000000.0 && flt64.abs() >= 0.000001{
					write!(f, "f64 {}", flt64)
				}else{
					write!(f, "f64 {:e}", flt64)
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

//Used to contain values on the heap only.
#[derive(PartialEq, Eq, Clone)]
pub enum HeapValue{
	String(String),
	List(Vec<Value>),
	Object(HashMap<String, Value>),
	Primitive(Value),
}

//Function used in displaying Lists as well as casting them to strings.
fn stringify_val_vec(ls: &Vec<Value>) -> String{
	let stringified = ls	
		.iter()
		.map(|el| format!("{}", el))
		.collect::<Vec<String>>()
		.join(", ");
	format!("[{}]", stringified)
}

fn stringify_obj(obj: &HashMap<String, Value>) -> String{
	let stringified = obj
		.iter()
		.map(|(k, v)| format!("{}: {}", k, v))
		.collect::<Vec<String>>()
		.join(", ");
	format!("{}{}{}", "{", stringified, "}")
}

impl fmt::Display for HeapValue{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		match self {
			HeapValue::String(s) => write!(f, "String {:?}", s),
			HeapValue::List(ls) => {
				write!(f, "List {}", stringify_val_vec(ls))
			},
			HeapValue::Object(o) => {
				write!(f, "Object {}", stringify_obj(o))
			},
			HeapValue::Primitive(p) => write!(f, "{}", p),
		}
	}
}

impl Default for HeapValue{
	fn default() -> Self{
		HeapValue::Primitive(Value::default())
	}
}

//Exists in token lists and AST.
#[derive(PartialEq, Eq, Clone)]
pub enum SuperValue{
	Reg(Value), 
	Heap(Box<HeapValue>),
}

impl From<HeapValue> for SuperValue{
	fn from(v: HeapValue) -> Self{
		SuperValue::Heap(Box::new(v))
	}
}

impl From<Value> for SuperValue{
	fn from(v: Value) -> Self{
		SuperValue::Reg(v)
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

impl Default for SuperValue {
	fn default() -> Self{
		SuperValue::Reg(Value::default())
	}
}

pub enum CastType{
	Usize, Uint8, Uint16, Uint32, Uint64,
	Uint128, Size, Int8, Int16, Int32,
	Int64, Int128, F32, F64, Char,
	Bool, StringBox, String, ListBox,
	List, ObjectBox, MiscBox
}

pub enum CastError{
	InvalidCast
}

pub trait TryCast<T>: Sized{
	type CastError;
	
	fn try_cast(v: T, target: CastType) -> Result<Self,Self::CastError>;
}

//Used to make match statement less bloated for conversions.
macro_rules! target_match{
	($target:ident, $($tar:literal, $res:ident),* $(,)?) => {
		match $target{
			$($tar => Ok(CastType::$res),)*
			_ => Err(CastError::InvalidCast)
		}	
	};
}

impl TryCast<&str> for CastType{
	type CastError = CastError;
	fn try_cast(tar: &str, _: CastType) -> Result<Self, Self::CastError>{
		target_match!{tar,
			"usize", Usize,
			"u8", Uint8,
			"u16", Uint16,
			"u32", Uint32,
			"u64", Uint64,
			"u128", Uint128,

			"isize", Size,
			"i8", Int8,
			"i16", Int16,
			"i32", Int32,
			"i64", Int64,
			"i128", Int128,
			
			"f32", F32,
			"f64", F64,

			"Char", Char,
			"Boolean", Bool,

			"StringBox", StringBox,
			"String", String,
			"ListBox", ListBox,
			"ObjectBox", ObjectBox,
			"List", List,
			"MiscBox", MiscBox
		}
	}
}

//Macro that compacts the match statement for integer casting.
macro_rules! integer_match {
	($t:ty, $v:ident, $target:ident, $($var:ident, $cast_to:ident, $type:ty),* $(,)?) => {
		match $target {
			$(CastType::$var => Ok(SuperValue::Reg(Value::$cast_to($v as $type))),)*
			CastType::Bool => Ok(SuperValue::Reg(Value::Boolean($v != (0 as $t)))),
			
			CastType::Char => {
				let char_num = $v as u32;
				if let Some(chr) = std::char::from_u32(char_num){
					Ok(SuperValue::Reg(Value::Char(chr)))
				}else{Err(Self::CastError::InvalidCast)} 
			},

			CastType::StringBox => Ok(Value::StringBox($v as usize).into()),
			CastType::ListBox => Ok(Value::ListBox($v as usize).into()),
			CastType::ObjectBox => Ok(Value::ObjectBox($v as usize).into()),
			CastType::MiscBox => Ok(Value::MiscBox($v as usize).into()),

			CastType::String => Ok(HeapValue::String($v.to_string()).into()),
			
			_ => Err(Self::CastError::InvalidCast)
		}
	};
}

macro_rules! impl_integer_casts {
	//What this block basically says is to loop through zero 
	// or more type arguments given and generate the two impl's below.
	($($t:ty),*) => {
		$(
			impl TryCast<$t> for SuperValue{
				type CastError = CastError;
				fn try_cast(v: $t, target: CastType) -> Result<Self, Self::CastError>{
					integer_match!{$t, v, target, 
						Usize, UIntSize, usize,
						Uint8, UInt8, u8,	
						Uint16, UInt16, u16,	
						Uint32, UInt32, u32,	
						Uint64, UInt64, u64,	
						Uint128, UInt128, u128,	

						Size, IntSize, isize,
						Int8, Int8, i8,	
						Int16, Int16, i16,	
						Int32, Int32, i32,	
						Int64, Int64, i64,	
						Int128, Int128, i128,	
			
						F32, Float32, f32,
						F64, Float64, f64,
					}
				}	
			}
		)*
	}
}

impl_integer_casts!{usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64}

//Macro that compacts the match statement for bool casting.
macro_rules! bool_match{
	($v:ident , $target:ident, $($var:ident, $cast_to:ident, $type:ty),* $(,)?) => {
		match $target {
			$(CastType::$var => 
				Ok(SuperValue::Reg(Value::$cast_to((if $v {1} else {0}) as $type))),)*
			CastType::Bool => Ok(Value::Boolean($v).into()),
			CastType::String => Ok(HeapValue::String($v.to_string()).into()),
			_ => Err(Self::CastError::InvalidCast)
		}
	};
}
impl TryCast<bool> for SuperValue{
	type CastError = CastError;
	fn try_cast(v: bool, target: CastType) -> Result<Self, Self::CastError>{
		bool_match!{v, target, 
			Usize, UIntSize, usize,
			Uint8, UInt8, u8,	
			Uint16, UInt16, u16,	
			Uint32, UInt32, u32,	
			Uint64, UInt64, u64,	
			Uint128, UInt128, u128,	

			Int8, Int8, i8,	
			Int16, Int16, i16,	
			Int32, Int32, i32,	
			Int64, Int64, i64,	
			Int128, Int128, i128,	

			F32, Float32, f32,
			F64, Float64, f64
		}
	}	
}

//Macro that compacts the match statement for Char casting.
macro_rules! char_match{
	($v:ident , $target:ident, $($var:ident, $cast_to:ident, $type:ty),* $(,)?) => {
		match $target {
			$(CastType::$var => Ok(Value::$cast_to(($v as u32) as $type).into()),)*
			CastType::String => Ok(HeapValue::String($v.to_string()).into()),
			_ => Err(Self::CastError::InvalidCast)
		}
	};
}
impl TryCast<char> for SuperValue{
	type CastError = CastError;
	fn try_cast(v: char, target: CastType) -> Result<Self, Self::CastError>{
		char_match!{v, target, 
			Usize, UIntSize, usize,
			Uint8, UInt8, u8,	
			Uint16, UInt16, u16,	
			Uint32, UInt32, u32,	
			Uint64, UInt64, u64,	
			Uint128, UInt128, u128,	

			Int8, Int8, i8,	
			Int16, Int16, i16,	
			Int32, Int32, i32,	
			Int64, Int64, i64,	
			Int128, Int128, i128,	

			F32, Float32, f32,
			F64, Float64, f64
		}
	}	
}

//Macro that compacts the match statement for String casting.
macro_rules! string_match{
	(&$v:ident , $target:ident, $($var:ident, $cast_to:ident, $type:ty),* $(,)?) => {
		match $target {
			$(CastType::$var => {
				if let Ok(parsed) = $v.parse(){
					Ok(SuperValue::Reg(Value::$cast_to(parsed)))
				}else{
					Err(Self::CastError::InvalidCast)
				}
			},)*
			CastType::List => {
				let char_ls: Vec<Value> = $v	
					.chars()
					.map(|c| Value::Char(c))
					.collect();
				Ok(HeapValue::List(char_ls).into())
			},
			CastType::Bool => {
				match $v.as_str(){
					"True" | "true" => Ok(Value::Boolean(true).into()),
					"False" | "false" => Ok(Value::Boolean(false).into()),
					_ => Err(Self::CastError::InvalidCast)
				}
			}
			_ => Err(Self::CastError::InvalidCast)
		}
	};
}
impl TryCast<&String> for SuperValue{
	type CastError = CastError;
	fn try_cast(v: &String, target: CastType) -> Result<Self, Self::CastError>{
		string_match!{&v, target, 
			Usize, UIntSize, usize,
			Uint8, UInt8, u8,	
			Uint16, UInt16, u16,	
			Uint32, UInt32, u32,	
			Uint64, UInt64, u64,	
			Uint128, UInt128, u128,	

			Int8, Int8, i8,	
			Int16, Int16, i16,	
			Int32, Int32, i32,	
			Int64, Int64, i64,	
			Int128, Int128, i128,	

			F32, Float32, f32,
			F64, Float64, f64
		}
	}	
}

impl TryCast<&Vec<Value>> for SuperValue{
	type CastError = CastError;
	fn try_cast(v: &Vec<Value>, target: CastType) -> Result<Self, Self::CastError>{
		match target{
			CastType::String => Ok(HeapValue::String(stringify_val_vec(v)).into()),
			_ => Err(Self::CastError::InvalidCast)
		}
	}
}

impl TryCast<&HashMap<String, Value>> for SuperValue{
	type CastError = CastError;
	fn try_cast(v: &HashMap<String, Value>, target: CastType) -> Result<Self, Self::CastError>{
		match target{
			CastType::String => Ok(HeapValue::String(stringify_obj(v)).into()),
			_ => Err(Self::CastError::InvalidCast)
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Operator{
	Add, Sub, Mul, Div, 
	Mod, Pow, 

	UsizeMax, U8Max, U16Max, U32Max, U64Max, U128Max, 
	IsizeMax, I8Max, I16Max, I32Max, I64Max, I128Max, 

	Swap, Drop, DropStack, Rot, Dup, DeepDup, 

	Equal, NotEqual, GreaterThan, LessThan, GreaterThanEqualTo,
	LessThanEqualTo, StringCompare, Concat,

	And, Or, Xor, Not,

	Push, Pop, Fpush, Fpop, Index, Length,

	IsEmpty, Clear, Contains, ChangeItemAt,
	
	IsWhitespaceChar, IsAlphaChar, IsNumChar,

	ObjAddField, ObjGetField, ObjMutField, ObjRemField,

	BitOr, BitAnd, BitXor, BitNot, BitShift, Cast,

	PrintLine, ReadLine, PrintChar, ReadChar, Print, Read, 
	DebugPrintStack, DebugPrintHeap,

	FileWrite, FileRead, FileCreate, FileRemove, FileExists,

	QueryType, LeaveScopeIfTrue, ThrowCustomError, GetArgs, 
	IsValidBox, TimeUnixNow, TimeWait, 

	Unknown
}

impl Default for Operator{
	fn default() -> Self{Operator::Unknown}
}

impl Operator{
	fn new(op_name: &str) -> Self{
		macro_rules! op_match{
			($(($name:literal, $var:ident)),* $(,)?) => {
				match op_name{
					$($name => Operator::$var,)*			
					_ => Operator::Unknown,
				}	
			};
		}
		op_match!{
			("+", Add), ("-", Sub), ("*", Mul), ("/", Div), 
			("mod", Mod), ("%", Mod), ("pow", Pow),

			("isizeMax", IsizeMax), ("usizeMax", UsizeMax),

			("i8Max", I8Max), ("i16Max", I16Max), ("i32Max", I32Max), 
			("i64Max", I64Max), ("i128Max", I128Max),

			("u8Max", U8Max), ("u16Max", U16Max), ("u32Max", U32Max),
			("u64Max", U64Max), ("u128Max", U128Max),

			("swap", Swap),	("drop", Drop),	("dropStack", DropStack),	
			("rot", Rot), ("dup", Dup),	("deepDup", DeepDup),	

			("==", Equal), ("!=", NotEqual), (">", GreaterThan), ("<", LessThan), 
			(">=", GreaterThanEqualTo), ("<=", LessThanEqualTo), 
			("stringCompare", StringCompare), ("++", Concat),

			("and", And), ("&&", And), ("or", Or), ("||", Or), 
			("xor", Xor), ("not", Not), ("!", Not),

			("push", Push), ("p", Push), ("pop", Pop), ("po", Pop), 
			("fpush", Fpush), ("fp", Fpush), ("fpop", Fpop), ("fpo", Fpop), 
			("index", Index), ("length", Length), ("len", Length),

			("isEmpty", IsEmpty), ("clear", Clear), 
			("contains", Contains), ("changeItemAt", ChangeItemAt),

			("isWhitespaceChar", IsWhitespaceChar), ("isAlphaChar", IsAlphaChar), 
			("isNumChar", IsNumChar),

			("objAddField", ObjAddField), ("objGetField", ObjGetField), 
			("objMutField", ObjMutField), ("objRemField", ObjRemField),

			("bitOr", BitOr), ("|", BitOr), ("bitAnd", BitAnd), 
			("&", BitAnd), ("bitXor", BitXor), ("^", BitXor), ("bitNot", BitNot), 
			("bitShift", BitShift), ("cast", Cast),

			("printLine", PrintLine), ("readLine", ReadLine), 
			("printChar", PrintChar), ("readChar", ReadChar), 
			("print", Print), ("read", Read), ("debugPrintStack", DebugPrintStack), 
			("debugPrintHeap", DebugPrintHeap),

			("fileWrite", FileWrite), ("fileRead", FileRead), 
			("fileCreate", FileCreate), ("fileRemove", FileRemove), 
			("fileExists", FileExists),

			("queryType", QueryType), ("leaveScopeIfTrue", LeaveScopeIfTrue), 
			("throwCustomError", ThrowCustomError), ("getArgs", GetArgs), 
			("isValidBox", IsValidBox), ("timeUnixNow", TimeUnixNow), 
			("timeWait", TimeWait),
		}	
	}
}

//Can either be a value to push to the stack or 
// a command to run an operator or something like that.
#[derive(PartialEq, Eq, Clone)]
pub enum Token{
	V(SuperValue),
	Word((Box<String>, Operator))
}

impl From<(&str, Option<Operator>)> for Token{
	fn from(params: (&str, Option<Operator>)) -> Self{
		let string_box = Box::new(params.0.to_string());
		let op_val = if let Some(o) = params.1{o}else{Operator::Unknown};
		Token::Word((string_box, op_val))	
	}
}

impl Default for Token{
	fn default() -> Self{
		Token::V(SuperValue::default())
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

#[derive(Copy, Eq, PartialEq, Clone)]
pub enum VarCmd{
	Make,
	Get,
	Mutate,
	Delete,
	Unknown
}

impl VarCmd{
	pub fn new(val: &str) -> Self{
		match val{
			"mak" => VarCmd::Make,
			"get" => VarCmd::Get,
			"mut" => VarCmd::Mutate,
			"del" => VarCmd::Delete,
			_ => VarCmd::Unknown
		}
	}	
}

impl fmt::Display for VarCmd{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		let cmd_str = match self{
			VarCmd::Make => "mak",
			VarCmd::Get => "get",
			VarCmd::Mutate => "mut",
			VarCmd::Delete => "del",
			VarCmd::Unknown => "UNKNOWN"
		};		

		write!(f, "{}", cmd_str)
	}	
}

#[derive(Copy, Eq, PartialEq, Clone)]
pub enum BoxCmd{
	Free,
	Null,
	Make,
	Open,
	Alter,
	Unknown
}

impl BoxCmd{
	pub fn new(val: &str) -> Self{
		match val{
			"make" => BoxCmd::Make,
			"open" => BoxCmd::Open,
			"altr" => BoxCmd::Alter,
			"null" => BoxCmd::Null,
			"free" => BoxCmd::Free,
			_ => BoxCmd::Unknown
		}
	}	
}

impl fmt::Display for BoxCmd{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		let cmd_str = match self{
			BoxCmd::Make => "make",
			BoxCmd::Open => "open",
			BoxCmd::Alter => "altr",
			BoxCmd::Null => "null",
			BoxCmd::Free => "free",
			BoxCmd::Unknown => "UNKNOWN"
		};	

		write!(f, "{}", cmd_str)
	}	
}

#[derive(Copy, Eq, PartialEq, Clone)]
pub enum FunCmd{
	Define,
	Call,
	Unknown
}

impl FunCmd{
	pub fn new(val: &str) -> Self{
		match val{
			"def" => FunCmd::Define,
			"call" => FunCmd::Call,
			_ => FunCmd::Unknown,
		}
	}	
}

impl fmt::Display for FunCmd{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		let cmd_str = match self{
			FunCmd::Define => "def",
			FunCmd::Call => "call",
			FunCmd::Unknown => "UNKNOWN"
		};		

		write!(f, "{}", cmd_str)
	}	
}

//The various types of nodes that are part of the Abstract Syntax Tree
#[derive(Clone)]
pub enum ASTNode{
	Terminal(Token),
	If {if_true: Box<ASTNode>, if_false: Box<ASTNode>},
	While(Box<ASTNode>),
	Expression(Vec<ASTNode>),
	Function{cmd: FunCmd, func_name: Box<String>, func_bod: Rc<ASTNode>},
	Variable{var_name: Box<String>, cmd: VarCmd, var_num: usize},
	LocVar{name: Box<String>, cmd: VarCmd, num: usize},
	BoxOp(BoxCmd),
	AttErr{attempt: Box<ASTNode>, err: Box<ASTNode>},
	Defer(Rc<ASTNode>),
	CastTo(String),
}

impl Default for ASTNode{
	fn default() -> Self{
		ASTNode::Terminal(Token::default())
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
			ASTNode::Function{cmd: c, func_name: name, func_bod: body} => {
				write!(f, "Function [cmd: {}, name: {}, body: {}]", c, name, body)
			},
			ASTNode::Variable{var_name: name, cmd: c, var_num: n} => write!(f, "Variable [name: {}, cmd: {}, num: {}]", name, c, n),
			ASTNode::LocVar{name: nm, cmd: c, num: n} => write!(f, "Local Variable [name: {}, cmd: {}, num: {}]", nm, c, n),
			ASTNode::BoxOp(op) => write!(f, "BoxOp {}", op),
			ASTNode::AttErr{attempt: att, err: e} => write!(f, "AttErr [attempt: {}, err: {}]", att, e),
			ASTNode::Defer(bod) => write!(f, "Defer [{}]", bod),
			ASTNode::CastTo(data_type) => write!(f, "CastTo {}", data_type),
		}
	}
}

//Uses the implemented format traits to build a string for the given Value type. 
// From there, it only consumes the characters that name the actual type.
// This avoids needing a big match statement. 
pub fn type_to_string(v: Value) -> String{
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

//Takes in a file string and calls the necessary functions 
// to build an AST from it.
pub fn parse_string_to_ast(argv: &Vec<String>, argc: usize, program_string: String) -> Result<(ASTNode, usize), String>{
	match tokenize(program_string.chars().collect()){
		Ok(tokens) => {
			//Constructs means of checking for duplicate imports.
			let mut imported_files: HashMap<String, ()> = HashMap::new();
			if argc > 1{
				imported_files.insert(argv[1].clone(), ());
			}
	
			match lex_tokens(tokens, &mut imported_files){
				Ok(lexed) => {
					match make_ast(lexed){
						Ok(res) => return Ok(res),
						Err(e) => return Err(e),
					}
				},
				Err(e) => return Err(e),	
			}

		},
		Err(e) => return Err(e),
	}


}

//Tokenizes list of chars into list of strings.
pub fn tokenize(chars: Vec<char>) -> Result<Vec<String>, String>{
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
					return Err("Parse error! Char missing closing apostraphie!".to_string());
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
			_ => return Err("SHOULD NEVER GET HERE!!!!!!!".to_string()),
		}
	}

	if in_string{
		return Err("Parse error! String not ended with matching double quotation!".to_string());
	}

	//If there was a valid token at the exact end of a file, it's picked up here.
	if curr_token.len() > 0{
		tokens.push(curr_token.iter().collect());
	}

	Ok(tokens)

}

pub fn throw_parse_error(t: &str, attempted_token: &String) -> String{
	return format!("Parse error! Incorrectly constructed {}! Tried: {}", t, attempted_token);
}

pub fn replace_literals_with_escapes(s: &str) -> String{
	let chars: Vec<char> = s.chars().collect();	
	let max = chars.len();
	let mut new_str = String::new();

	let mut is_escaped = false;

	for i in 0..max{
		//Skips current char if it's already been escaped.
		if is_escaped{
			is_escaped = false;
			continue;
		}			

		if i + 1 < max && chars[i] == '\\'{
			match chars[i + 1]{
				'\\' => new_str.push('\\'),
				'n' => new_str.push('\n'),
				't' => new_str.push('\t'),
				'r' => new_str.push('\r'),
				'\"' => new_str.push('\"'),
				'\'' => new_str.push('\''),
				'0' => new_str.push('\0'),
				'b' => new_str.push('\x08'),
				'f' => new_str.push('\x0c'),
				other => new_str.push(other),
			}
			is_escaped = true;
		}else{
			new_str.push(chars[i]);
		}
	
	}
	new_str
}

//Given reference to list of seperated tokens, 
// differentiates each one as either a value or word.
//WARNING! OWNERSHIP TRANSFERS SO, YOU BETTER WATCH OUT!
pub fn lex_tokens(
	tokens: Vec<String>, 
	imported: &mut HashMap<String, ()>) -> Result<Vec<Token>, String>
{
	let mut lexed: Vec<Token> = Vec::new();

	macro_rules! tok_match{
		($el:ident, $(($tystr:literal, $ty:ty, $var:ident)),* $(,)?) => {
			match &$el{
				t if t == "True" || t == "true" => {
					lexed.push(Token::V(Value::Boolean(true).into()));
				},
				t if t == "False" || t == "false" => {
					lexed.push(Token::V(Value::Boolean(false).into()));
				},
				//String case.
				t if t.starts_with("\"") && t.ends_with("\"") => {
					lexed.push(Token::V(HeapValue::String(
					replace_literals_with_escapes(&t[1..(t.len() - 1)])).into()));
				}, 
				//Char case.
				t if t.starts_with("\'") && t.ends_with("\'") => {
					let mut iter = $el[1..].chars();
					let mut captured: char = iter.nth(0).unwrap();
					if captured == '\\'{
						captured = match iter.nth(0).unwrap(){
							'n' => '\n',
							't' => '\t',
							'r' => '\r',
							'0' => '\0',
							'\'' => '\'',
							'\"' => '\"',
							'b' => '\x08',
							'f' => '\x0c',
							_ => captured,
						};
					}
					lexed.push(Token::V(Value::Char(captured).into()));
				},
				//List case.
				t if t == "[]" => lexed.push(Token::V(HeapValue::List(Vec::new()).into())),
				//Object case.
				t if t == "{}" => lexed.push(
					Token::V(HeapValue::Object(HashMap::new()).into())),
				//Generalized macro case handling specified floats and integers.
				$(t if t.ends_with($tystr) && t.len() > $tystr.len() => {
					match $el[0..($el.len() - $tystr.len())].parse::<$ty>(){
						Ok(parsed) => lexed.push(Token::V(Value::$var(parsed).into())),	
						Err(_) => return Err(throw_parse_error($tystr, t)),
					}
				},)*
				//Type inference for float.
				t if t.contains(".") 
						&& (t.chars().next().unwrap() == '-' 
						|| (t.chars().next().unwrap() >= '0' 
							&& t.chars().next().unwrap() <= '9')) 
						=> {
					match $el.parse::<f32>(){
						Ok(parsed) => lexed.push(Token::V(Value::Float32(parsed).into())),
						Err(_) => return Err(throw_parse_error("f32", t)),
					}
				},
				//Type inference for integer.
				t if (t.chars().next().unwrap() == '-' && t.len() > 1) 
						|| (t.chars().next().unwrap() >= '0' 
							&& t.chars().next().unwrap() <= '9') 
						=> {
					match $el.parse::<isize>(){
						Ok(parsed) => lexed.push(Token::V(Value::IntSize(parsed).into())),
						Err(_) => return Err(throw_parse_error("isize", t)),
					}
				},

				//Recursive import() statement case.
				t if t.starts_with("import(") && t.ends_with(")") => {
					//Grabs file string out of import statement. 
					let import_str = "import("; 
					let file_str = &t[(import_str.len())..(t.len() - 1)];
					
					let import_file_path = Path::new(file_str);

					//If file not already imported, inserts into file hashmap.
					// If it is, then nothing happens.
					if !imported.contains_key(file_str){
						imported.insert(file_str.to_string(), ());

						//Opens the input file to read from.
						let mut import_file = match File::open(&import_file_path){
							Ok(f) => f,
							Err(reason) => {
								let import_file_name = import_file_path.display();
								return Err(format!("Unable to open import \
									file {} for parsing because {}", import_file_name, reason));
							}, 
						};

						//Reads in the code from the given file after opening it.
						let mut import_code_str = String::new();
						match import_file.read_to_string(&mut import_code_str){
							Ok(_) => {},
							Err(reason) => {
								let import_file_name = import_file_path.display();
								return Err(format!("Unable to read in\
									import file {} because {}", import_file_name, reason)); 
							}, 
						}

						//Pushes all $elens from recursive traversal into current lexed list.
						match tokenize(import_code_str.chars().collect()){
							Ok(import_tokens) => {
								match lex_tokens(import_tokens, imported){
									Ok(toks) => {
										for tok in toks.into_iter(){
											lexed.push(tok)
										}
									},
									Err(e) => return Err(e),
								}
									
							},
							Err(e) => return Err(e),
						}

					}

				}, 
				
				//General catch-all case mostly meant for operators.
				_ => {
					let op_val = Operator::new(&$el);
					lexed.push(Token::Word((Box::new($el), op_val)));
				}, 
			}	
		};
	}

	for tok in tokens.into_iter(){
		tok_match!{tok,
			("f32", f32, Float32),
			("f64", f64, Float64),
			
			("usize", usize, UIntSize),
			("u8", u8, UInt8),
			("u16", u16, UInt16),
			("u32", u32, UInt32),
			("u64", u64, UInt64),
			("u128", u128, UInt128),

			("isize", isize, IntSize),
			("i8", i8, Int8),
			("i16", i16, Int16),
			("i32", i32, Int32),
			("i64", i64, Int64),
			("i128", i128, Int128),
		}
	}

	Ok(lexed)
}

//This function does the heavy-lifting of recursively building the AST.
pub fn make_ast_prime(
	mut already_parsed: Vec<ASTNode>, 
	tokens: Vec<Token>, 
	token_index: usize,
	loc_nums: &mut HashMap<String, usize>,
	curr_loc_num: &mut usize, 
	terminators: Vec<Token>
) -> Result<(Vec<ASTNode>, Vec<Token>, usize, Option<usize>), String>{
	//If out of tokens to parse, end or throw error if there were terminators to look for.
	if token_index >= tokens.len(){
		if terminators.len() == 0{
			return Ok((already_parsed, tokens, token_index, None))
		}else{
			let mut terms = String::new();
			for t in terminators.iter(){
				terms.push_str(&format!("{}, ", t));
			}
			return Err(format!("Ended expression without finding one of: {}", terms));
		}
	//If still tokens to parse, converts the tokens into an ASTNode.
	}else{
		match tokens[token_index]{
			//Stop on terminator case. 
			ref tok if terminators.contains(tok) => Ok((already_parsed, tokens, token_index + 1, Some(token_index))),
			//Parse if statement case.
			Token::Word(ref cmd) if *cmd.0 == "if" => {
				match parse_if(tokens, token_index + 1, loc_nums, curr_loc_num){
					Ok((true_branch, false_branch, tokens_prime, token_index_prime)) => {
						already_parsed.push(ASTNode::If{if_true : Box::new(true_branch), if_false : Box::new(false_branch)});
						return make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators);
					},	
					Err(e) => return Err(e),
				}
			},
			//While loop parsing case.
			Token::Word(ref cmd) if *cmd.0 == "while" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![(";", None).into()]){
					Ok((loop_body, tokens_prime, token_index_prime, _)) => {
						already_parsed.push(ASTNode::While(Box::new(ASTNode::Expression(loop_body))));
						make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
					},
					Err(e) => return Err(e),
				}                 

			},
			//Function case.
			Token::Word(ref cmd) if *cmd.0 == "func" => {
				//Makes sure there's enough stuff to look to parse the function.
				if token_index + 2 > tokens.len(){
					return Err("Insufficient tokens left for function to be parsed!".to_string());
				}

				let mut toks = tokens;
				let command = std::mem::take(&mut toks[token_index + 1]);
				let name = std::mem::take(&mut toks[token_index + 2]);
				let (command_str, name_str) = match (command, name){
					(Token::Word(c), Token::Word(n)) => (c.0, n.0),
					(_, _) => return Err("SHOULD NEVER GET HERE!!!".to_string()),
				};

				match make_ast_prime(Vec::new(), toks, token_index + 3, loc_nums, curr_loc_num, vec![Token::Word((Box::new(";".to_string()), Operator::default()))]){
					Ok((fbod, tokens_prime, token_index_prime, _)) => {
						let fbod_ast = Rc::new(ASTNode::Expression(fbod));
						already_parsed.push(ASTNode::Function{cmd: FunCmd::new(&command_str), func_name: name_str, func_bod: fbod_ast});
						make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
					},
					Err(e) => return Err(e),
				}	

			},
			//Var command parsing case.
			Token::Word(ref cmd) if *cmd.0 == "var" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![(";", None).into()]){
					Ok((mut var_data, tokens_prime, token_index_prime, _)) => {
						if var_data.len() >= 2{
							let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
								(ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c.0, n.0),
								(_, _) => {return Err("Malformed variable command Error! \
									Insufficient parameters given for variable command!".to_string())},
							};
							let vn: usize = match loc_nums.get(&*name){
								Some(n) => *n,
								None => {
									loc_nums.insert(*name.clone(), *curr_loc_num);
									let ret = *curr_loc_num;
									*curr_loc_num += 1;
									ret
								},
							};
							already_parsed.push(ASTNode::Variable{var_name: name, cmd: VarCmd::new(&cmd), var_num: vn});
							make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)

						}else{
							return Err("Malformed variable command Error! \
								Insufficient parameters given for variable command!".to_string());
						}
					},
					Err(e) => return Err(e),
				}
			},
			//Loc command parsing case.
			Token::Word(ref cmd) if *cmd.0 == "loc" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![(";", None).into()]){
					Ok((mut var_data, tokens_prime, token_index_prime, _)) => {
						if var_data.len() >= 2{
							let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
								(ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c.0, n.0),
								(_, _) => return Err("Malformed local variable command Error! \
									Insufficient parameters given for local variable command!".to_string()),
							};
							let var_num: usize = match loc_nums.get(&*name){
								Some(n) => *n,
								None => {
									loc_nums.insert(*name.clone(), *curr_loc_num);
									let ret = *curr_loc_num;
									*curr_loc_num += 1;
									ret
								},
							};
							already_parsed.push(ASTNode::LocVar{name: name, cmd: VarCmd::new(&cmd), num: var_num});
							make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)

						}else{
							Err("Malformed local variable command Error! \
								Insufficient parameters given for local variable command!".to_string())
						}
					},
					Err(e) => return Err(e),
				}	
	
			},
			//Box command case.
			Token::Word(ref cmd) if *cmd.0 == "box" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![(";", None).into()]) {
					Ok((mut box_data, tokens_prime, token_index_prime, _)) => {
						if box_data.len() >= 1{
							let box_cmd = match std::mem::take(&mut box_data[0]){
								ASTNode::Terminal(Token::Word(c)) => c.0,
								_ => return Err("Malformed box command!".to_string()),
							};

							already_parsed.push(ASTNode::BoxOp(BoxCmd::new(&box_cmd)));
							make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
						}else{
							return Err("Malformed box command! No box command token given!".to_string());
						}
					},
					Err(e) => return Err(e),
				}
			},
			//Attempt onError case.
			Token::Word(ref cmd) if *cmd.0 == "attempt" => {
				match parse_att_err(tokens, token_index + 1, loc_nums, curr_loc_num){
					Ok((att_branch, err_branch, tokens_prime, token_index_prime)) => {
						already_parsed.push(ASTNode::AttErr{attempt: Box::new(att_branch), err: Box::new(err_branch)});
						return make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators);
					},
					Err(e) => return Err(e),
				} 
			},
			//Defer case.
			Token::Word(ref cmd) if *cmd.0 == "defer" => {
				match make_ast_prime(
						Vec::new(),
						tokens, 
						token_index + 1, 
						loc_nums,
						curr_loc_num,
						vec![(";", None).into()]
					) {
					Ok((defer_body, tokens_prime, token_index_prime, _)) => {
						already_parsed.push(ASTNode::Defer(Rc::new(ASTNode::Expression(defer_body))));
						make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
					},
					Err(e) => return Err(e),
				} 
			},
			//castTo case
			Token::Word(ref cmd) if *cmd.0 == "castTo" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, 
						curr_loc_num, vec![(";", None).into()])  {
					Ok((mut cast_data, tokens_prime, token_index_prime, _)) => {
						if cast_data.len() >= 1{
							let data_type = match std::mem::take(&mut cast_data[0]){
								ASTNode::Terminal(Token::Word(d)) => d.0,
								_ => return Err("Malformed castTo!".to_string())
							};

							already_parsed.push(ASTNode::CastTo(*data_type));
							make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
						}else{
							return Err("Malformed castTo command! No data type given!".to_string())
						}
					},	
					Err(e) => return Err(e),
				}
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
pub fn parse_att_err(
	tokens: Vec<Token>,
	token_index: usize, 
	loc_nums: &mut HashMap<String, usize>,
	curr_loc_num: &mut usize) -> Result<(ASTNode, ASTNode, Vec<Token>, usize), String>{
	match make_ast_prime(
			Vec::new(),
			tokens, 
			token_index, loc_nums, curr_loc_num,
			vec![("onError", None).into()]
		) {
		Ok((att_branch, tokens_prime, token_index_prime, terminator_index)) => {
			match terminator_index{
				Some(i) => {
					match tokens_prime[i]{
						Token::Word(ref cmd) if *cmd.0 == "onError" => {
							match make_ast_prime(
									Vec::new(),
									tokens_prime,
									token_index_prime, 
									loc_nums,
									curr_loc_num,
									vec![(";", None).into()]
								) {
								Ok((error_branch, tokens_prime_prime, token_index_prime_prime, _)) => {
									
									Ok((ASTNode::Expression(att_branch), ASTNode::Expression(error_branch), 
										tokens_prime_prime, token_index_prime_prime))
								},
								Err(e) => return Err(e),
							}
						},
						_ => Err("Failed to correctly construct attempt onError block!".to_string()),
					}
				},
				None => return Err("REALLY SHOULD NEVER GET HERE!".to_string())
			}
		},
		Err(e) => return Err(e),	
	}
}

pub fn parse_if(
	tokens: Vec<Token>, 
	token_index: usize, 
	loc_nums: &mut HashMap<String, usize>, 
	curr_loc_num: &mut usize) -> Result<(ASTNode, ASTNode, Vec<Token>, usize), String>{
	match make_ast_prime(
			Vec::new(), 
			tokens, 
			token_index, loc_nums, curr_loc_num, 
			vec![("else", None).into(), (";", None).into()]
		){
		Ok((true_branch, tokens_prime, token_index_prime, terminator_index)) => {
			match terminator_index{
				Some(i) => {
					match tokens_prime[i]{
						Token::Word(ref cmd) if *cmd.0 == "else" => {
							match parse_else(tokens_prime, token_index_prime, loc_nums, curr_loc_num) {
								Ok((false_branch, tokens_prime_prime, token_index_prime_prime)) => {
				
								Ok((ASTNode::Expression(true_branch), false_branch, 
									tokens_prime_prime, token_index_prime_prime))
								},
								Err(e) => return Err(e),
							}
						},
						_ => Ok((ASTNode::Expression(true_branch), ASTNode::Expression(vec![]), tokens_prime, token_index_prime)),  
					}
				},
				_ => return Err("SHOULD NEVER GET HERE!!!".to_string()),
			}
		},
		Err(e) => return Err(e),	
	}
}

pub fn parse_else(
	tokens: Vec<Token>, 
	token_index: usize, 
	loc_nums: &mut HashMap<String, usize>,
	curr_loc_num: &mut usize) -> Result<(ASTNode, Vec<Token>, usize), String>{
	match  
		make_ast_prime(
			Vec::new(),
			tokens, 
			token_index, loc_nums, curr_loc_num,
			vec![(";", None).into()]
		){
		Ok((if_false, tokens_prime, token_index_prime, _)) => {
			Ok((ASTNode::Expression(if_false), tokens_prime, token_index_prime))
		},
		Err(e) => return Err(e),
	}
}

//Consumes a vec of tokens and generates an Abstract Syntax Tree (AST) from it,
// returning it for the program to then run. 
// It also returns the number of unique local variable names for later use in running the program. 
pub fn make_ast(tokens: Vec<Token>) -> Result<(ASTNode, usize), String>{
	let mut loc_nums: HashMap<String, usize> = HashMap::new();
	let mut curr_loc_num: usize = 0;
	match make_ast_prime(Vec::new(), tokens, 0, &mut loc_nums, &mut curr_loc_num, Vec::new()){
		Ok(res) => return Ok((ASTNode::Expression(res.0), curr_loc_num)),	
		Err(e) => return Err(e),
	}
}
