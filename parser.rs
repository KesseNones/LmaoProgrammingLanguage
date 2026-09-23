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
	IntSize(isize),

	//Unsigned integers.
	UInt8(u8),
	UInt16(u16),
	UInt32(u32),
	UInt64(u64),
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
			Value::IntSize(n) => write!(f, "isize {}", n),

			Value::UInt8(n) => write!(f, "u8 {}", n),
			Value::UInt16(n) => write!(f, "u16 {}", n),
			Value::UInt32(n) => write!(f, "u32 {}", n),
			Value::UInt64(n) => write!(f, "u64 {}", n),
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

#[derive(Clone, Copy)]
pub enum CastType{
	Usize, Uint8, Uint16, Uint32, Uint64,
	Size, Int8, Int16, Int32, Int64, 
	F32, F64, Char, Bool, StringBox, String, ListBox,
	List, ObjectBox, MiscBox
}
macro_rules! disp_match{
	($target:ident, $form:ident, $($var:ident, $res:literal),* $(,)?) => {
		match $target{
			$(CastType::$var => write!($form, $res),)*
		}	
	};
}

impl fmt::Display for CastType{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		disp_match!{self, f,
			Usize, "usize",
			Uint8, "u8",	
			Uint16, "u16",	
			Uint32, "u32",	
			Uint64, "u64",	

			Size, "isize",
			Int8, "i8",	
			Int16, "i16",	
			Int32, "i32",	
			Int64, "i64",	

			F32, "f32",
			F64, "f64",
	
			Char, "Char",
			Bool, "Boolean",
		
			StringBox, "StringBox",
			ListBox, "ListBox",
			ObjectBox, "ObjectBox",
			MiscBox, "MiscBox",

			List, "List",
			String, "String",
		}
	}
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

			"isize", Size,
			"i8", Int8,
			"i16", Int16,
			"i32", Int32,
			"i64", Int64,
			
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

						Size, IntSize, isize,
						Int8, Int8, i8,	
						Int16, Int16, i16,	
						Int32, Int32, i32,	
						Int64, Int64, i64,	
			
						F32, Float32, f32,
						F64, Float64, f64,
					}
				}	
			}
		)*
	}
}

impl_integer_casts!{usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64}

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

			Int8, Int8, i8,	
			Int16, Int16, i16,	
			Int32, Int32, i32,	
			Int64, Int64, i64,	

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

			Int8, Int8, i8,	
			Int16, Int16, i16,	
			Int32, Int32, i32,	
			Int64, Int64, i64,	

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

			Int8, Int8, i8,	
			Int16, Int16, i16,	
			Int32, Int32, i32,	
			Int64, Int64, i64,	

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

	UsizeMax, U8Max, U16Max, U32Max, U64Max,  
	IsizeMax, I8Max, I16Max, I32Max, I64Max, 

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
macro_rules! match_call{
	($mac:ident) => {
		$mac!{
			("+", Add), ("-", Sub), ("*", Mul), ("/", Div), 
			("mod", Mod), ("%", Mod), ("pow", Pow),

			("isizeMax", IsizeMax), ("usizeMax", UsizeMax),

			("i8Max", I8Max), ("i16Max", I16Max), ("i32Max", I32Max), 
			("i64Max", I64Max),

			("u8Max", U8Max), ("u16Max", U16Max), ("u32Max", U32Max),
			("u64Max", U64Max),

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
	};
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
		match_call!{op_match}	
	}
	pub fn stringify(&self) -> String{
		macro_rules! op_match{
			($(($name:literal, $var:ident)),* $(,)?) => {
				#[allow(unreachable_patterns)]
				match self{
					$(Operator::$var => $name.to_string(),)*			
					_ => "Unknown".to_string(),
				}	
			};
		}
		match_call!{op_match}
	}
}

//Can either be a value to push to the stack or 
// a command to run an operator or something like that.
#[derive(PartialEq, Eq, Clone)]
pub enum Token{
	Val(SuperValue),
	Op(Operator),
	If,
	Else,
	While,
	Var,
	Loc,
	Box,
	Func,
	Attempt,
	OnError,
	File{name: String, tokens: Vec<Token>},
	Terminator,
	Fragment(String),
	CastTo,
	Defer,
	NOTHING
}

impl Default for Token{
	fn default() -> Self{
		Token::Val(SuperValue::default())
	}
}

impl fmt::Display for Token{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		match self {
			Token::Val(v) => write!(f, "Value: {}", v),
			Token::Op(o) => write!(f, "Op: {}", o.stringify()),
			Token::If => write!(f, "If"),
			Token::Else => write!(f, "Else"),
			Token::While => write!(f, "While"),
			Token::Var => write!(f, "Var"),
			Token::Loc => write!(f, "Loc"),
			Token::Box => write!(f, "Box"),
			Token::Func => write!(f, "Func"),
			Token::Defer => write!(f, "Defer"),
			Token::Attempt => write!(f, "Attempt"),
			Token::OnError => write!(f, "OnError"),
			Token::Terminator => write!(f, "Terminator: ;"),
			Token::Fragment(frag) => write!(f, "Fragment: {}", frag),
			Token::File{name: n, tokens: _} => write!(f, "File: {}", n),
			Token::CastTo => write!(f, "CastTo"),
			Token::NOTHING => write!(f, "NOTHING"),
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

#[derive(Clone)]
pub struct FuncData{
	pub cmd: FunCmd,
	pub name: String,
	pub bod: Rc<ASTNode>,
}

impl FuncData{
	pub fn new(c: FunCmd, n: &str, b: Rc<ASTNode>) -> Self{
		FuncData{
			cmd: c,
			name: n.to_string(),
			bod: Rc::clone(&b)	
		}
	}
}

#[derive(Clone)]
pub struct IfData{
	pub if_true: ASTNode,
	pub if_false: ASTNode
}

impl IfData{
	pub fn new(t: ASTNode, f: ASTNode) -> Self{
		IfData{if_true: t, if_false: f}
	}
}

#[derive(Clone)]
pub struct VarData{
	pub name: String,
	pub cmd: VarCmd,
}

impl VarData{
	fn new(n: &str, c: VarCmd) -> Self{
		VarData{
			name: n.to_string(),
			cmd: c
		}
	}
}

#[derive(Clone)]
pub struct AttErrData{
	pub att: ASTNode,
	pub err: ASTNode 
}
impl AttErrData{
	fn new(attempt: ASTNode, error: ASTNode) -> Self{
		AttErrData{att: attempt, err: error}
	}
}

#[derive(Clone)]
pub struct FileData{
	pub name: String,	
	pub program: ASTNode
}
impl FileData{
	fn new(name: &str, prog: ASTNode) -> Self{
		FileData{name: name.to_string(), program: prog}
	}
}

//The various types of nodes that are part of the Abstract Syntax Tree
#[derive(Clone)]
pub enum ASTNode{
	Op(Operator),
	Val(Value),	
	HeapVal(Box<HeapValue>),
	If(Box<IfData>),
	While(Box<ASTNode>),
	Expression(Box<Vec<ASTNode>>),
	Function(Box<FuncData>),
	Variable(Box<VarData>),
	LocVar(Box<VarData>),
	BoxOp(BoxCmd),
	AttErr(Box<AttErrData>),
	Defer(Rc<ASTNode>),
	CastTo(CastType),
	File(Box<FileData>)
}

//Useful for shorthand conversion of ASTNode vec to Expression.
impl From<Vec<ASTNode>> for ASTNode{
	fn from(v: Vec<ASTNode>) -> Self{
		ASTNode::Expression(Box::new(v))	
	}
}

impl Default for ASTNode{
	fn default()->Self{
		ASTNode::Val(Value::NULLBox)
	}
}

impl fmt::Display for ASTNode{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		match self{
			ASTNode::Op(id) => write!(f, "Operator {}", id.stringify()),
			ASTNode::Val(v) => write!(f, "{}", v),
			ASTNode::HeapVal(hv) => write!(f, "{}", hv),
			ASTNode::If(data) => write!(f, "If [true_branch: {}, false_branch: {}]", data.if_true, data.if_false),
			ASTNode::While(body) => write!(f, "While [{}]", body),
			ASTNode::Expression(vec) => {
				let strs: Vec<String> = vec.iter().map(|n| format!("{}", n)).collect();
				write!(f, "Expression [{}]", strs.join(", "))
			},
			ASTNode::Function(data) => {
				write!(f, "Function [cmd: {}, name: {}, body: {}]", data.cmd, data.name, data.bod)
			},
			ASTNode::Variable(data) => write!(f, "Variable [name: {}, cmd: {}]", data.name, data.cmd),
			ASTNode::LocVar(data) => write!(f, "Local Variable [name: {}, cmd: {}]", data.name, data.cmd),
			ASTNode::BoxOp(op) => write!(f, "BoxOp {}", op),
			ASTNode::AttErr(data) => write!(f, "AttErr [attempt: {}, err: {}]", data.att, data.err),
			ASTNode::Defer(bod) => write!(f, "Defer [{}]", bod),
			ASTNode::CastTo(data_type) => write!(f, "CastTo {}", data_type),
			ASTNode::File(data) => write!(f, "File {}", data.name)
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
pub fn parse_string_to_ast(argv: &Vec<String>, argc: usize, program_string: String) -> Result<ASTNode, String>{
	//Constructs means of checking for duplicate imports.
	let mut imported_files: HashMap<String, ()> = HashMap::new();

	match tokenize(&program_string, &mut imported_files, &argv[1]){
		Ok(tokens) => {
			match make_ast(tokens){
				Ok(res) => return Ok(res),
				Err(e) => return Err(e),
			}
		},
		Err(e) => return Err(e),	
	}
}

//Tokenizes program string into list of tokens.
pub fn tokenize(
program_string: &str, imported: &mut HashMap<String, ()>, program_name: &str) 
-> Result<Token, String>
{
	let chars: Vec<char> = program_string.chars().collect();
	let mut tokens: Vec<Token> = Vec::new();
	let mut curr_token: Vec<char> = Vec::new();

	let mut in_string = false;
	let mut in_comment = false;
	let mut in_char = false;

	let mut line = 1;

	let mut i: usize = 0;
	while i < chars.len(){
		match (chars[i], in_string, in_comment, in_char){
			//Start of string case.
			('\"', false, false, false) => {
				curr_token.push(chars[i]);
				in_string = true;
				i += 1;

			},
			//Makes it so strings can have double quotes inside them, as long as they are escaped.
			('\\', true, false, false) => {
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
			('\"', true, false, false) => {
				curr_token.push(chars[i]);
				in_string = false;
				i += 1;
			},
			//In string case.
			(_, true, false, false) => {
				curr_token.push(chars[i]);
				i += 1;
			},
			//Start of Char case.
			('\'', false, false, false) => {
				curr_token.push(chars[i]);
				in_char = true;
				i += 1;	
			},
			//End of Char case, or continuance with escape.
			('\'', false, false, true) => {
				if 
					curr_token.len() > 0 && 
					curr_token[curr_token.len() - 1] != '\\'{
					in_char = false	
				}
				curr_token.push(chars[i]);
				i += 1;
			},
			//In Char case.
			(c, false, false, true) => {
				curr_token.push(c);
				i += 1;	
			},
			//Comment entry case.
			('/', false, false, false) => {
				if ((i + 1) < chars.len()) && (chars[i + 1] == '/'){
					in_comment = true;
					i += 2;
				}else{
					curr_token.push(chars[i]);
					i += 1;
				}
			},
			//Exit comment case.
			('\n', false, true, false) => {
				in_comment = false;
				i += 1;
			},
			//In comment case.
			(_, false, true, false) => i += 1,
			//General parsing case.
			(c, false, false, false) => {
				if !c.is_whitespace(){
					curr_token.push(c);
				}else{
					if curr_token.len() > 0{
						let t = curr_token.iter().collect::<String>();
						curr_token.clear();

						if t.starts_with("import(\"") && t.ends_with("\")"){
							let import_str = "import(\"";
							let file_str = &t[import_str.len()..t.len()-2];

							let import_file_path = Path::new(file_str);

							if !imported.contains_key(file_str){
								imported.insert(file_str.to_string(), ());
			
								//Opens the input file to read from.
								let mut import_file = match File::open(&import_file_path){
									Ok(f) => f,
									Err(reason) => {
										let import_file_name = import_file_path.display();
										return Err(format!("Error at line {}. Unable to open import \
											file {} for parsing because {}", line, import_file_name, reason));
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

								//Pushes all tokens from recursive traversal into current lexed list.
								match tokenize(&import_code_str, imported, file_str){
									Ok(import_token) => {
										tokens.push(import_token)	
									},
									Err(e) => return Err(e),
								}
							}
						}else{
							match lex_token(&t){
								Ok(token) => tokens.push(token),
								Err(e) => return Err(e),
							}
						}
						if c == '\n' {line += 1;}
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

	if curr_token.len() > 0{
		match lex_token(&curr_token.iter().collect::<String>()){
			Ok(token) => tokens.push(token),
			Err(e) => return Err(e),
		}
	}

	Ok(Token::File{name: program_name.to_string(), tokens: tokens})
}

pub fn throw_parse_error(t: &str, attempted_token: &str) -> String{
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

//Takes in a string and returns a token or related error message.
fn lex_token(tok: &str) -> Result<Token, String>{
	macro_rules! tok_match{
		($el:ident, $(($tystr:literal, $ty:ty, $var:ident)),* $(,)?) => {
			match $el{
				t if t == "True" || t == "true" => {
					let bool_val = Value::Boolean(true);
					let sup = SuperValue::Reg(bool_val);
					Ok(Token::Val(sup))
				},
				t if t == "False" || t == "false" => {
					let bool_val = Value::Boolean(false);
					let sup = SuperValue::Reg(bool_val);
					Ok(Token::Val(sup))
				},
				//String case.
				t if t.starts_with("\"") && t.ends_with("\"") => {
					Ok(Token::Val(HeapValue::String(
					replace_literals_with_escapes(&t[1..(t.len() - 1)])).into()))
				}, 
				//Char case.
				t if t.starts_with("\'") && t.ends_with("\'") && t.len() < 13 => {
					let mut iter = t.chars();
					_ = iter.next();
					match (iter.next(), iter.next()){
						(Some('\\'), Some(c)) => {
							let res = match c{
								'n' => '\n',
								't' => '\t',
								'r' => '\r',
								'0' => '\0',
								'\'' => '\'',
								'\"' => '\"',
								'b' => '\x08',
								'f' => '\x0c',
								_ => '\\',
							};	
							Ok(Token::Val(Value::Char(res).into()))
						},
						(Some(c), Some(_)) => Ok(Token::Val(Value::Char(c).into())),
						_ => Err(format!("Parsing error! Token {} is not a valid Char!", t)),	
					}
				},
				//List case.
				t if t == "[]" => Ok(Token::Val(HeapValue::List(Vec::new()).into())),
				//Object case.
				t if t == "{}" => Ok(Token::Val(HeapValue::Object(HashMap::new()).into())),
				//Generalized macro case handling specified floats and integers.
				$(t if t.ends_with($tystr) && t.len() > $tystr.len() => {
					match $el[0..($el.len() - $tystr.len())].parse::<$ty>(){
						Ok(parsed) => Ok(Token::Val(Value::$var(parsed).into())),	
						Err(_) => Err(throw_parse_error($tystr, t)),
					}
				},)*
				//Type inference for float.
				t if t.contains(".") 
						&& (t.chars().next().unwrap() == '-' 
						|| (t.chars().next().unwrap() >= '0' 
							&& t.chars().next().unwrap() <= '9')) 
						=> {
					match $el.parse::<f32>(){
						Ok(parsed) => Ok(Token::Val(Value::Float32(parsed).into())),
						Err(_) => Err(throw_parse_error("f32", t)),
					}
				},
				//Type inference for integer.
				t if (t.chars().next().unwrap() == '-' && t.len() > 1) 
						|| (t.chars().next().unwrap() >= '0' 
							&& t.chars().next().unwrap() <= '9') 
						=> {
					match $el.parse::<isize>(){
						Ok(parsed) => Ok(Token::Val(Value::IntSize(parsed).into())),
						Err(_) => Err(throw_parse_error("isize", t)),
					}
				},
				//The keywords for starting fancy operators.
				"if" => Ok(Token::If),
				"while" => Ok(Token::While),
				"var" => Ok(Token::Var),
				"loc" => Ok(Token::Loc),
				"func" => Ok(Token::Func),
				"attempt" => Ok(Token::Attempt),
				"castTo" => Ok(Token::CastTo),
				"box" => Ok(Token::Box),
				"defer" => Ok(Token::Defer),
				//Terminators for some/all fancy operators.
				"onError" => Ok(Token::OnError),
				";" => Ok(Token::Terminator),
				"else" => Ok(Token::Else),
				//General catch-all case mostly meant for operators.
				t => {
					//If valid operator, kick that back.
					let op_val = Operator::new(&t);
					if op_val != Operator::Unknown{
						return Ok(Token::Op(op_val));
					}

					//If alphanumeric only, kick back as fragment.
					for c in t.chars(){
						if c.is_whitespace(){
							return Err(
								format!(
							"Parsing error! Token: \"{}\" is not a valid Token!", t)
							);
						}
					}	
					Ok(Token::Fragment(t.to_string()))
				}, 
			}	
		};
	}

	tok_match!{tok,
		("f32", f32, Float32),
		("f64", f64, Float64),
		
		("usize", usize, UIntSize),
		("u8", u8, UInt8),
		("u16", u16, UInt16),
		("u32", u32, UInt32),
		("u64", u64, UInt64),

		("isize", isize, IntSize),
		("i8", i8, Int8),
		("i16", i16, Int16),
		("i32", i32, Int32),
		("i64", i64, Int64),
	}
}

//This function does the heavy-lifting of recursively building the AST.
pub fn make_ast_prime(
	prog_name: &str,
	tokens: &Vec<Token>, 
	mut token_index: usize,
	fancy_op: Token
) -> Result<(Vec<ASTNode>, usize), String>{

	let mut already_parsed: Vec<ASTNode> = Vec::new();

	//Loops through all tokens in expression.
	loop{
		//If out of tokens to parse, end or throw error if there were terminators to look for.
		if token_index >= tokens.len(){
			if fancy_op != Token::NOTHING{
				return Err(format!("Ended Fancy Operator based on token: {} without finding a valid terminator!", fancy_op));
			}else{
				return Ok((already_parsed, token_index));
			}
		//If still tokens to parse, converts the tokens into an ASTNode.
		}else{
			match &tokens[token_index]{
				Token::Terminator => {
					match fancy_op{
						Token::Attempt => {
							return Err("Invalid terminator for attempt given! Expected \"onError\", found \";\"".to_string());
						},
						Token::NOTHING => {
							return Err("Invalid terminator given! Not inside fancy operator!".to_string());
						},
						_ => {
							return Ok((already_parsed, token_index));	
						},
					}
				},
				Token::Val(sup_val) => {
					match sup_val{
						SuperValue::Reg(v) => {
							already_parsed.push(ASTNode::Val(*v));
						},
						SuperValue::Heap(hv) => {
							already_parsed.push(ASTNode::HeapVal(Box::new(*hv.clone())));
						},
					}				
				},
				//Basically a direct translation.
				Token::Op(o) => already_parsed.push(ASTNode::Op(*o)),
				//If you see a rogue fragment like this, error out!
				Token::Fragment(f) => {
					return Err(format!("Parsing error! Fragment \"{}\" not contained by a fancy Operator!", f))
				},	
				Token::While =>{
					match make_ast_prime(
						prog_name, tokens, token_index + 1, Token::While
					)
					{
						Ok((loop_body, token_idx_prime)) => {
							let loop_expr = ASTNode::Expression(Box::new(loop_body));
							already_parsed.push(ASTNode::While(Box::new(loop_expr)));
							token_index = token_idx_prime;	
						},
						Err(e) => return Err(e),
					}
					
				},
				Token::Defer =>{
					match make_ast_prime(
						prog_name, tokens, token_index + 1, Token::Defer
					)
					{
						Ok((def_bod, token_idx_prime)) => {
							let def_exp = ASTNode::Expression(Box::new(def_bod));
							already_parsed.push(ASTNode::Defer(Rc::new(def_exp)));
							token_index = token_idx_prime;	
						},
						Err(e) => return Err(e),
					}
					
				},
				Token::If => {
					match make_ast_prime(
						prog_name, tokens, token_index + 1, Token::If
					)
					{
						Ok((if_body, token_idx_prime)) => {
							token_index = token_idx_prime;
							//If else statement detected, 
							// parses recursively the else body.
							// Otherwise just puts nothing in for else_body.
							let else_body: Vec<ASTNode>;
							match tokens[token_index]{
								Token::Else => {
									match make_ast_prime(
										prog_name, tokens, token_index + 1, Token::Else
									)
									{
										Ok((bod, tok_idx_prime_prime)) => {
											else_body = bod;	
											token_index = tok_idx_prime_prime;
										},
										Err(e) => return Err(e),
									}
								},
								Token::Terminator => else_body = Vec::new(),
								_ => return Err("SHOULD NEVER GET HERE!".to_string())
							}
	
							//Boxes up ASTNodes to then push as If node.
							let if_expr = ASTNode::Expression(Box::new(if_body));
							let else_expr = ASTNode::Expression(Box::new(else_body));
							let if_data = IfData::new(if_expr, else_expr);
							let if_node = ASTNode::If(Box::new(if_data));
							already_parsed.push(if_node);

						},
						Err(e) => return Err(e),
					}
				},
				//Acts like a terminator but only for If statements.
				Token::Else => {
					match fancy_op{
						Token::If => return Ok((already_parsed, token_index)),
						Token::NOTHING => {
							return Err("Parse error! Else token detected outside of fancy operator!".to_string());
						},
						t => {
							return Err(
								format!(
									"Parse error! Else token must follow \
									If token! Else following \"{}\" token here!", t
								)
							);
						}
					}
				},
				Token::Attempt => {
					match make_ast_prime(
						prog_name, tokens, token_index + 1, Token::Attempt
					)
					{
						Ok((att_body, token_idx_prime)) => {
							match make_ast_prime(
								prog_name, tokens, token_idx_prime + 1, Token::OnError
							)
							{
								Ok((err_bod, tok_idx_prime_prime)) => {
									let att_expr = ASTNode::Expression(Box::new(att_body));
									let err_expr = ASTNode::Expression(Box::new(err_bod));	
									let att_err_data = AttErrData::new(att_expr, err_expr);
									
									let att_err_node = ASTNode::AttErr(Box::new(att_err_data));

									already_parsed.push(att_err_node);
									token_index = tok_idx_prime_prime;
								},
								Err(e) => return Err(e),
							}
						},
						Err(e) => return Err(e),
					}
				},
				Token::OnError => {
					match fancy_op{
						Token::Attempt => return Ok((already_parsed, token_index)),
						Token::NOTHING => {
							return Err("Parse error! OnError token detected outside of fancy operator!".to_string());
						},
						t => {
							return Err(
								format!(
									"Parse error! OnError token must follow \
									Attempt token! OnError following \"{}\" token here!", t
								)
							);
						},
					}
				},
				Token::Var => {
					let ti = token_index;
					let ts = tokens;
					match (ts.get(ti + 1), ts.get(ti + 2), ts.get(ti + 3)) {
						(Some(Token::Fragment(a)), Some(Token::Fragment(name)), 
						Some(Token::Terminator)) => 
						{
							//Parses command and throws a fit if not known.
							let command = VarCmd::new(a);
							if command == VarCmd::Unknown{
								return Err(format!("Parsing error! Unknown Var command \"{}\" for variable named \"{}\"!", a, name));
							}

							let var_data = VarData::new(name, command);
							already_parsed.push(ASTNode::Variable(Box::new(var_data)));
							token_index += 3;
						},
						(Some(a), Some(b), Some(c)) => {
							return Err(
								format!("Parsing error! Var command needs 2 Fragment tokens and one Terminator token! Found: {} {} {}", a, b, c)
							);
						},
						(Some(a), Some(b), None) => {
							return Err(format!("Parsing error! Expected ending terminator token for total of 3 arguments! Only supplied with: {} and {}", a, b));
						},
						(Some(a), None, None) => {
							return Err(format!("Parsing error! Expected middle Fragment token and Terminator token at the end for total of 3 arguments! Only supplied with: {}", a));
						},
						(None, None, None) => {
							return Err(format!("Parsing error! No arguments supplied for Var token!"));
						},
						_ => return Err("SHOULD NEVER GET HERE!".to_string()),
					}
				},
				Token::Loc => {
					let ti = token_index;
					let ts = tokens;
					match (ts.get(ti + 1), ts.get(ti + 2), ts.get(ti + 3)) {
						(Some(Token::Fragment(a)), Some(Token::Fragment(name)), 
						Some(Token::Terminator)) => 
						{
							//Parses command and throws a fit if not known.
							let command = VarCmd::new(a);
							if command == VarCmd::Unknown || command == VarCmd::Delete{
								return Err(format!("Parsing error! Invalid Loc command \"{}\" for local variable named \"{}\"!", a, name));
							}

							let var_data = VarData::new(name, command);
							already_parsed.push(ASTNode::LocVar(Box::new(var_data)));
							token_index += 3;
						},
						(Some(a), Some(b), Some(c)) => {
							return Err(
								format!("Parsing error! Loc command needs 2 Fragment tokens and one Terminator token! Found tokens: {} {} {}", a, b, c)
							);
						},
						(Some(a), Some(b), None) => {
							return Err(format!("Parsing error! Expected ending terminator token for total of 3 arguments! Only supplied with first two: {} and {}", a, b));
						},
						(Some(a), None, None) => {
							return Err(format!("Parsing error! Expected middle Fragment token and Terminator token at the end for total of 3 arguments! Only supplied with: {}", a));
						},
						(None, None, None) => {
							return Err(format!("Parsing error! No arguments supplied for Loc token!"));
						},
						_ => return Err("SHOULD NEVER GET HERE!".to_string()),
					}
				},
				Token::Box => {
					match (tokens.get(token_index + 1), tokens.get(token_index + 2)){
						(Some(Token::Fragment(f)), Some(Token::Terminator)) => {
							let cmd = BoxCmd::new(f);
							if cmd == BoxCmd::Unknown{
								return Err(format!("Parsing error! Token \"{}\" is not a valid box command!", f));
							}
							already_parsed.push(ASTNode::BoxOp(cmd));	
							token_index += 2;
						},
						(Some(a), Some(b)) => {
							return Err(format!("Parsing error! Expected valid box command Fragment token and terminator token! Received tokens: \"{}\" and \"{}\"", a, b));
						},
						(Some(a), None) => {
							return Err(format!("Parsing error! Expected terminator after box command \"{}\"; found nothing!", a));
						},
						(None, None) => {
							return Err(format!("Parsing error! Expected box command fragment and terminator after box keyword. Found nothing!"));
						},
						_ => return Err("SHOULD NEVER GET HERE!".to_string()),
					}
				},
				Token::CastTo => {
					match (tokens.get(token_index + 1), tokens.get(token_index + 2)){
						(Some(Token::Fragment(ty)), Some(Token::Terminator)) => {
							match CastType::try_cast(ty, CastType::MiscBox){
								Ok(cast_type) => {
									already_parsed.push(ASTNode::CastTo(cast_type));
									token_index += 2;
								},
								Err(_) => return Err(format!("Parse error! Token \"{}\" is not a valid casting data type!", ty)),
							}
						},
						(Some(a), Some(b)) => {
							return Err(format!("Parsing error! Expected data type Fragment and terminator token! Received tokens: \"{}\" and \"{}\"", a, b));
						},
						(Some(a), None) => {
							return Err(format!("Parsing error! Expected terminator after data type \"{}\"; found nothing!", a));
						},
						(None, None) => {
							return Err(format!("Parsing error! Expected data type fragment and terminator tokens after CastTo token. Found nothing!"));
						},
						_ => return Err("SHOULD NEVER GET HERE!".to_string()),
					}
				},
				Token::Func => {
					match (tokens.get(token_index + 1), tokens.get(token_index + 2)) {
						(Some(Token::Fragment(c)), Some(Token::Fragment(name))) => {
							match FunCmd::new(c){
								FunCmd::Define => {
									match make_ast_prime(
										prog_name, tokens, token_index + 3, Token::Func
									)
									{
										Ok((f_bod, token_index_prime)) => {
											let bod_node = ASTNode::Expression(Box::new(f_bod));
											let f_data = FuncData::new(FunCmd::Define, name, Rc::new(bod_node));
											already_parsed.push(ASTNode::Function(Box::new(f_data)));
											token_index = token_index_prime;
										},
										Err(e) => return Err(e),
									}
								},
								FunCmd::Call => {
									//Since it's a function call, 
									// it checks for a terminator and that's it.
									match tokens.get(token_index + 3){
										Some(Token::Terminator) => {
											let empty_bod: Vec<ASTNode> = Vec::new();
											let bod_box = Box::new(empty_bod);
											let empty = ASTNode::Expression(bod_box);
											let f_data = FuncData::new(FunCmd::Call, name, Rc::new(empty));
											already_parsed.push(ASTNode::Function(Box::new(f_data)));

											token_index += 3;
										},
										Some(a) => {
											return Err(format!("Parsing error! Expected Terminator token at end of calling function \"{}\" found: \"{}\"", name, a));	
										},
										None => {
											return Err(format!("Parsing error! Expected Terminator token at end of calling function \"{}\" found nothing!", name));	
										},
									}
								},
								FunCmd::Unknown => {
									return Err(format!("Parsing error! Invalid command token given for function \"{}\"! Given: \"{}\"", name, c));
								}
							}
						},
						(Some(a), Some(b)) => {
							return Err(format!("Parsing error! Func token expected at least two fragment tokens after it! Found: \"{}\" and \"{}\"", a, b));	
						},
						(Some(a), None) => {
							return Err(format!("Parsing error! Func token expected at least two fragment tokens after it! Only found: \"{}\"", a));	
						},
						(None, None) => {
							return Err(format!("Parsing error! Func token expected at least two fragment tokens after it! Found nothing!"));	
						},
						_ => return Err("SHOULD NEVER GET HERE!".to_string()),
					}
				},
				Token::File{name: n, tokens: toks} => {
					match make_ast_prime(&n, &toks, 0, Token::NOTHING) {
						Ok((file_body, _)) => {
							let new_expr = ASTNode::Expression(Box::new(file_body));
							let f_data = FileData::new(&n, new_expr);
							already_parsed.push(ASTNode::File(Box::new(f_data)));
						},
						Err(e) => return Err(e)
					}	
				},
				Token::NOTHING => (),
			}
			token_index += 1;
		}
	}
}

// Consumes a file Token and creates an AST based on it.
pub fn make_ast(tokens: Token) -> Result<ASTNode, String>{
	if let Token::File{name: n, tokens: toks} = tokens{
		match make_ast_prime(&n, &toks, 0, Token::NOTHING){
			Ok((ast_vec, _)) => {
				let ast_expr = ASTNode::Expression(Box::new(ast_vec));
				let ast_data = FileData::new(&n, ast_expr);
				Ok(ASTNode::File(Box::new(ast_data)))
			},
			Err(e) => return Err(e),
		}
	}else{
		return Err(format!("Parsing error! AST creation must start at File level Token! \
Received token: {}", tokens));
	}
}
