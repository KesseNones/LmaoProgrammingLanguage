use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::Read;

//This pub enum is used to contain all the possible data types of Lmao 
// that live everywhere but the Heap.
#[derive(PartialEq, Clone)]
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
		HeapValue::Primitive(Value::default())
	}
}

//Exists in token lists and AST.
#[derive(PartialEq, Eq, Clone)]
pub enum SuperValue{
	Reg(Value), 
	Heap(HeapValue),
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

//Can either be a value to push to the stack or 
// a command to run an operator or something like that.
#[derive(PartialEq, Eq, Clone)]
pub enum Token{
	V(SuperValue),
	Word((String, usize))
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

//The various types of nodes that are part of the Abstract Syntax Tree
#[derive(Clone)]
pub enum ASTNode{
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
			ASTNode::Function{func_cmd: cmd, func_name: name, func_bod: body} => {
				write!(f, "Function [cmd: {}, name: {}, body: {}]", cmd, name, body)
			},
			ASTNode::Variable{var_name: name, var_cmd: cmd, var_num: n} => write!(f, "Variable [name: {}, cmd: {}, num: {}]", name, cmd, n),
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
pub fn type_to_string(v: &Value) -> String{
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

//Builds hashmap to translate operator strings to their respective indices.
pub fn make_ops_hashmap() -> HashMap<String, usize>{
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
		"getArgs", "isValidBox", "timeUnixNow", "timeWait"
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
	
	ops_map
}

//Takes in a file string and calls the necessary functions 
// to build an AST from it.
pub fn parse_string_to_ast(argv: &Vec<String>, argc: usize, program_string: String) -> Result<(ASTNode, usize), String>{
	match tokenize(program_string.chars().collect()){
		Ok(tokens) => {
			let ops_map = make_ops_hashmap();
		  
			//Constructs means of checking for duplicate imports.
			let mut imported_files: HashMap<String, ()> = HashMap::new();
			if argc > 1{
				imported_files.insert(argv[1].clone(), ());
			}
	
			match lex_tokens(tokens, &ops_map, &mut imported_files){
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
	ops_map: &HashMap<String, usize>, 
	imported: &mut HashMap<String, ()>) -> Result<Vec<Token>, String>
{
	let mut lexed: Vec<Token> = Vec::new();

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
						'b' => '\x08',
						'f' => '\x0c',
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
					Err(_) => return Err(throw_parse_error("f32", &tok)),
				}
			},
			ref t if t.ends_with("f64") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<f64>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Float64(parsed)))),
					Err(_) => return Err(throw_parse_error("f64", &tok)),
				}
			},
			//Explicit integer cases for both signed and unsigned.
			ref t if t.ends_with("u8") && t.len() > 2 => {
				match tok[0..(tok.len() - 2)].parse::<u8>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt8(parsed)))),
					Err(_) => return Err(throw_parse_error("u8", &tok)),
				}
			},
			ref t if t.ends_with("i8") && t.len() > 2 => {
				match tok[0..(tok.len() - 2)].parse::<i8>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int8(parsed)))),
					Err(_) => return Err(throw_parse_error("i8", &tok)),
				}
			},
			ref t if t.ends_with("u16") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<u16>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt16(parsed)))),
					Err(_) => return Err(throw_parse_error("u16", &tok)),
				}
			},
			ref t if t.ends_with("i16") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<i16>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int16(parsed)))),
					Err(_) => return Err(throw_parse_error("i16", &tok)),
				}
			},
			ref t if t.ends_with("u32") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<u32>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt32(parsed)))),
					Err(_) => return Err(throw_parse_error("u32", &tok)),
				}
			},
			ref t if t.ends_with("i32") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<i32>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int32(parsed)))),
					Err(_) => return Err(throw_parse_error("i32", &tok)),
				}
			},
			ref t if t.ends_with("u64") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<u64>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt64(parsed)))),
					Err(_) => return Err(throw_parse_error("u64", &tok)),
				}
			},
			ref t if t.ends_with("i64") && t.len() > 3 => {
				match tok[0..(tok.len() - 3)].parse::<i64>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int64(parsed)))),
					Err(_) => return Err(throw_parse_error("u64", &tok)),
				}
			},
			ref t if t.ends_with("u128") && t.len() > 4 => {
				match tok[0..(tok.len() - 4)].parse::<u128>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UInt128(parsed)))),
					Err(_) => return Err(throw_parse_error("u128", &tok)),
				}
			},
			ref t if t.ends_with("i128") && t.len() > 4 => {
				match tok[0..(tok.len() - 4)].parse::<i128>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::Int128(parsed)))),
					Err(_) => return Err(throw_parse_error("i128", &tok)),
				}
			},
			ref t if t.ends_with("usize") && t.len() > 5 => {
				match tok[0..(tok.len() - 5)].parse::<usize>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::UIntSize(parsed)))),
					Err(_) => return Err(throw_parse_error("usize", &tok)),
				}
			},
			ref t if t.ends_with("isize") && t.len() > 5 => {
				match tok[0..(tok.len() - 5)].parse::<isize>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::IntSize(parsed)))),
					Err(_) => return Err(throw_parse_error("isize", &tok)),
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
					Err(_) => return Err(throw_parse_error("f32", &tok)),
				}
			},
			//Type inference for integer.
			ref t if (t.chars().next().unwrap() == '-' && t.len() > 1) 
					|| (t.chars().next().unwrap() >= '0' 
						&& t.chars().next().unwrap() <= '9') 
					=> {
				match tok.parse::<isize>(){
					Ok(parsed) => lexed.push(Token::V(SuperValue::Reg(Value::IntSize(parsed)))),
					Err(_) => return Err(throw_parse_error("isize", &tok)),
				}
			},

			//Recursive import() statement case.
			ref t if t.starts_with("import(") && t.ends_with(")") => {
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

					//Pushes all tokens from recursive traversal into current lexed list.
					match tokenize(import_code_str.chars().collect()){
						Ok(import_tokens) => {
							match lex_tokens(import_tokens, ops_map, imported){
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
				let n: usize = *ops_map.get(&tok).unwrap_or(&0);
				lexed.push(Token::Word((tok, n)));
			}, 
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
			Token::Word(ref cmd) if cmd.0 == "if" => {
				match parse_if(tokens, token_index + 1, loc_nums, curr_loc_num){
					Ok((true_branch, false_branch, tokens_prime, token_index_prime)) => {
						already_parsed.push(ASTNode::If{if_true : Box::new(true_branch), if_false : Box::new(false_branch)});
						return make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators);
					},	
					Err(e) => return Err(e),
				}
			},
			//While loop parsing case.
			Token::Word(ref cmd) if cmd.0 == "while" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]){
					Ok((loop_body, tokens_prime, token_index_prime, _)) => {
						already_parsed.push(ASTNode::While(Box::new(ASTNode::Expression(loop_body))));
						make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
					},
					Err(e) => return Err(e),
				}                 

			},
			//Function case.
			Token::Word(ref cmd) if cmd.0 == "func" => {
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

				match make_ast_prime(Vec::new(), toks, token_index + 3, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]){
					Ok((fbod, tokens_prime, token_index_prime, _)) => {
						let fbod_ast = Rc::new(ASTNode::Expression(fbod));
						already_parsed.push(ASTNode::Function{func_cmd: command_str, func_name: name_str, func_bod: fbod_ast});
						make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
					},
					Err(e) => return Err(e),
				}	

			},
			//Var command parsing case.
			Token::Word(ref cmd) if cmd.0 == "var" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]){
					Ok((mut var_data, tokens_prime, token_index_prime, _)) => {
						if var_data.len() >= 2{
							let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
								(ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c.0, n.0),
								(_, _) => {return Err("Malformed variable command Error! \
									Insufficient parameters given for variable command!".to_string())},
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
							return Err("Malformed variable command Error! \
								Insufficient parameters given for variable command!".to_string());
						}
					},
					Err(e) => return Err(e),
				}
			},
			//Loc command parsing case.
			Token::Word(ref cmd) if cmd.0 == "loc" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]){
					Ok((mut var_data, tokens_prime, token_index_prime, _)) => {
						if var_data.len() >= 2{
							let (cmd, name) = match (std::mem::take(&mut var_data[0]), std::mem::take(&mut var_data[1])){
								(ASTNode::Terminal(Token::Word(c)), ASTNode::Terminal(Token::Word(n))) => (c.0, n.0),
								(_, _) => return Err("Malformed local variable command Error! \
									Insufficient parameters given for local variable command!".to_string()),
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
							Err("Malformed local variable command Error! \
								Insufficient parameters given for local variable command!".to_string())
						}
					},
					Err(e) => return Err(e),
				}	
	
			},
			//Box command case.
			Token::Word(ref cmd) if cmd.0 == "box" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, curr_loc_num, vec![Token::Word((";".to_string(), 0))]) {
					Ok((mut box_data, tokens_prime, token_index_prime, _)) => {
						if box_data.len() >= 1{
							let box_cmd = match std::mem::take(&mut box_data[0]){
								ASTNode::Terminal(Token::Word(c)) => c.0,
								_ => return Err("Malformed box command!".to_string()),
							};

							already_parsed.push(ASTNode::BoxOp(box_cmd));
							make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
						}else{
							return Err("Malformed box command! No box command token given!".to_string());
						}
					},
					Err(e) => return Err(e),
				}
			},
			//Attempt onError case.
			Token::Word(ref cmd) if cmd.0 == "attempt" => {
				match parse_att_err(tokens, token_index + 1, loc_nums, curr_loc_num){
					Ok((att_branch, err_branch, tokens_prime, token_index_prime)) => {
						already_parsed.push(ASTNode::AttErr{attempt: Box::new(att_branch), err: Box::new(err_branch)});
						return make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators);
					},
					Err(e) => return Err(e),
				} 
			},
			//Defer case.
			Token::Word(ref cmd) if cmd.0 == "defer" => {
				match make_ast_prime(
						Vec::new(),
						tokens, 
						token_index + 1, 
						loc_nums,
						curr_loc_num,
						vec![Token::Word((";".to_string(), 0))]
					) {
					Ok((defer_body, tokens_prime, token_index_prime, _)) => {
						already_parsed.push(ASTNode::Defer(Rc::new(ASTNode::Expression(defer_body))));
						make_ast_prime(already_parsed, tokens_prime, token_index_prime, loc_nums, curr_loc_num, terminators)
					},
					Err(e) => return Err(e),
				} 
			},
			//castTo case
			Token::Word(ref cmd) if cmd.0 == "castTo" => {
				match make_ast_prime(Vec::new(), tokens, token_index + 1, loc_nums, 
						curr_loc_num, vec![Token::Word((";".to_string(), 0))])  {
					Ok((mut cast_data, tokens_prime, token_index_prime, _)) => {
						if cast_data.len() >= 1{
							let data_type = match std::mem::take(&mut cast_data[0]){
								ASTNode::Terminal(Token::Word(d)) => d.0,
								_ => return Err("Malformed castTo!".to_string())
							};

							already_parsed.push(ASTNode::CastTo(data_type));
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
			vec![Token::Word(("onError".to_string(), 0))]
		) {
		Ok((att_branch, tokens_prime, token_index_prime, terminator_index)) => {
			match terminator_index{
				Some(i) => {
					match tokens_prime[i]{
						Token::Word(ref cmd) if cmd.0 == "onError" => {
							match make_ast_prime(
									Vec::new(),
									tokens_prime,
									token_index_prime, 
									loc_nums,
									curr_loc_num,
									vec![Token::Word((";".to_string(), 0))]
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
			vec![Token::Word(("else".to_string(), 0)), Token::Word((";".to_string(), 0))]
		){
		Ok((true_branch, tokens_prime, token_index_prime, terminator_index)) => {
			match terminator_index{
				Some(i) => {
					match tokens_prime[i]{
						Token::Word(ref cmd) if cmd.0 == "else" => {
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
			vec![Token::Word((";".to_string(), 0))]
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
