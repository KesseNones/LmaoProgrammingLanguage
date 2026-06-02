use crate::parser::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cmp::Ordering;
use std::fmt;
use fmt::Display;
use std::io;
use std::path::Path;
use std::fs::File;
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write; 
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

type OpFunc = fn(&mut State) -> Result<(), String>;

//Main mutable state
pub struct State{
	pub stack: Vec<Value>,
	pub fns: HashMap<String, Rc<ASTNode>>,
	pub vars: Vec<(Value, bool)>,
	pub heap: Vec<(HeapValue, bool)>,
	pub free_list: Vec<usize>,
	pub ops: Vec<OpFunc>,
	pub frames: Vec<(usize, Vec<(Value, bool)>)>,
	pub curr_frame: usize,
	pub frame_pool: Vec<Vec<(Value, bool)>>,
	pub unique_var_name_count: usize,
	pub leaving_scope: bool,
	pub buffer: String,
}


//Used for numerical operators like +, -, *, etc.
pub fn numerical_type_error_string(op_type: &str, v1: &Value, v2: &Value) -> String{
	format!("Operator ({}) error! Operand types must match and be numeric types! Attempted values: {} and {}", op_type, v1, v2)
}

pub fn needs_n_args_only_n_provided(op_type: &str, args_needed: &str, args_provided: &str) -> String{
	let plural_s: &str;
	if args_needed == "One"{
		plural_s = "";
	}else{
		plural_s = "s";
	}
	format!("Operator ({}) error! {} operand{} required on stack; {} provided!", op_type, args_needed, plural_s, args_provided)
}

pub fn should_never_get_here_for_func(func: &str) -> String{
	format!("Should never get here for {} function!", func)
}

pub fn push_val_or_err(r: Result<Value, String>, s: &mut State) -> Result<(), String>{
	match r{
		Ok(v) => {
			s.stack.push(v);
			Ok(())
		},
		Err(e) => Err(e),
	}
}

//Adds two values of matching numerical types together, pusing the result to the stack.
pub fn add(s: &mut State) -> Result<(), String>{
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
pub fn sub(s: &mut State) -> Result<(), String>{
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
pub fn mult(s: &mut State) -> Result<(), String>{
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

pub fn division_by_zero_error(t: &str) -> String{
	format!("Operator (/) error! Division by zero occuring between two operands of type {}!", t)
}

//Pops two items from top of stack and divides them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as division by zero.
pub fn div(s: &mut State) -> Result<(), String>{
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
pub fn modulo_by_zero_error(t: &str) -> String{
	format!("Operator (%) error! Modulo by zero occuring between two operands of type {}!", t)
}

//Pops two items from top of stack and modulos them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as modulo by zero.
pub fn modulo(s: &mut State) -> Result<(), String>{
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
pub fn power(s: &mut State) -> Result<(), String>{
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
pub fn swap(s: &mut State) -> Result<(), String>{
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
pub fn drop(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(_) => Ok(()),
		None => Err(needs_n_args_only_n_provided("drop", "One", "none")),
	}
}

//Clears existing stack to be empty. 
// This can be useful if you want a clean stack without doing a ton of drops.
pub fn drop_stack(s: &mut State) -> Result<(), String>{
	s.stack.clear();
	Ok(())
}

//Rotates top three items on stack, 
// putting the top item below the previous two.
pub fn rot(s: &mut State) -> Result<(), String>{
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
pub fn dup(s: &mut State) -> Result<(), String>{
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
pub fn error_for_deep_dup_due_to_bad_box(box_type: &str, disp_value: Value) -> String{
	format!("Operator (deepDup) error. Deep duplication of {} failed \
		because it's an invalid {} number due to being been free'd!", disp_value, box_type)
}

//Works like dup but duplicates the data held by box types 
// and creates a new box to hold the duplicated data.
pub fn deep_dup(s: &mut State) -> Result<(), String>{
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
pub fn equality_error(op_type: &str, v1: &Value, v2: &Value) -> String{
	format!("Operator ({}) error! Comparisons of equality and inequality \
		must have matching types! Attempted values: {} and {}", op_type, v1, v2)
}

//Checks for equality between two data types. For boxes it checks to see 
// if the box numbers are equal and for NULL box it checks for self-equality.
//Consumes both items from stack and pushes resulting boolean based on their comparison.
pub fn is_equal(s: &mut State) -> Result<(), String>{
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
pub fn is_not_equal(s: &mut State) -> Result<(), String>{
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

pub fn comparison_error(op_type: &str, v1: &Value, v2: &Value) -> String{
	format!("Operator ({}) error! Non-equality comparison operators need matching \
		non-null types to function! Attempted values: {} and {}", op_type, v1, v2)
}

//Compares two values on stack to see if the second to top is greater than the top.
pub fn is_greater_than(s: &mut State) -> Result<(), String>{
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
pub fn is_less_than(s: &mut State) -> Result<(), String>{
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
pub fn is_greater_than_equal_to(s: &mut State) -> Result<(), String>{
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
pub fn is_less_than_equal_to(s: &mut State) -> Result<(), String>{
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
pub fn bad_box_error(op_type: &str, box_type1: &str, box_type2: &str, bn1: usize, bn2: usize, is_two_boxes: bool) -> String{
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
pub fn concat(s: &mut State) -> Result<(), String>{
	let res: Result<Value, String> = match s.pop2(){
		(Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
			//Only concatenates the strings if both boxes are valid and different!
			if a != b{
				match (s.validate_box(a), s.validate_box(b)){
					(true, true) => {
						//Concatenates string from Box B to string in Box A.
						let mut a_str: HeapValue = std::mem::take(&mut s.heap[a].0);
						if let (HeapValue::String(s1), HeapValue::String(s2)) = (&mut a_str, &s.heap[b].0){
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
						if let (HeapValue::List(ls1), HeapValue::List(ls2)) = (&mut list_a, &s.heap[b].0){
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

pub fn logical_operator_type_error(op_type: &str, v1: &Value, v2: &Value) -> String{
	format!("Operator ({}) error! Logical operation requires two operands \
		of matching type Boolean! Attempted values: {} and {}", op_type, v1, v2)
}

//Performs logical AND on two operands.
pub fn and(s: &mut State) -> Result<(), String>{
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
pub fn or(s: &mut State) -> Result<(), String>{
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
pub fn xor(s: &mut State) -> Result<(), String>{
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
pub fn not(s: &mut State) -> Result<(), String>{
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
pub fn list_push(s: &mut State) -> Result<(), String>{
	let res: Result<Value, String> = match s.pop2(){
		(Some(Value::ListBox(bn)), Some(v)) => {
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &mut s.heap[bn].0{
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
				if let HeapValue::String(st) = &mut s.heap[bn].0{
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
pub fn pop_error(op_type: &str, collection_type: &str, op_detail: &str) -> String{
	format!("Operator ({}) error! {} needs to be greater \
		than length 0 for {} operation to actually pop something!", op_type, collection_type, op_detail)
}

//Pops from the end of a list/string and pushes the popped thing to the stack.
pub fn list_pop(s: &mut State) -> Result<(), String>{
	let res: Result<(Value, Value), String> = match s.pop(){
		Some(Value::ListBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &mut s.heap[bn].0{
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
				if let HeapValue::String(st) = &mut s.heap[bn].0{
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
pub fn list_front_push(s: &mut State) -> Result<(), String>{
	let res: Result<Value, String> = match s.pop2(){
		(Some(Value::ListBox(bn)), Some(v)) => {
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &mut s.heap[bn].0{
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
				if let HeapValue::String(st) = &mut s.heap[bn].0{
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
pub fn list_front_pop(s: &mut State) -> Result<(), String>{
	let res: Result<(Value, Value), String> = match s.pop(){
		Some(Value::ListBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &mut s.heap[bn].0{
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
				if let HeapValue::String(st) = &mut s.heap[bn].0{
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
pub fn index(s: &mut State) -> Result<(), String>{
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
pub fn length(s: &mut State) -> Result<(), String>{
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
pub fn is_empty(s: &mut State) -> Result<(), String>{
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
pub fn list_clear(s: &mut State) -> Result<(), String>{
	let res: Result<Value, String> = match s.pop(){
		Some(Value::ListBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &mut s.heap[bn].0{
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
				if let HeapValue::String(st) = &mut s.heap[bn].0{
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
pub fn list_contains(s: &mut State) -> Result<(), String>{
	let res = match s.pop2(){
		(Some(Value::ListBox(bn)), Some(v)) => {
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &s.heap[bn].0{
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
					if let (HeapValue::Object(o), HeapValue::String(s)) = (&s.heap[a].0, &s.heap[b].0){
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
				if let HeapValue::String(st) = &s.heap[bn].0{
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
pub fn change_item_at(s: &mut State) -> Result<(), String>{
	let res = match s.pop3(){
		(Some(Value::ListBox(bn)), Some(Value::UIntSize(i)), Some(v)) => {
			//Changes item in list to new value at 
			// index i assuming list is valid and index is in range.
			if s.validate_box(bn){
				if let HeapValue::List(ls) = &mut s.heap[bn].0{
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
pub fn non_char_error(op_type: &str, v: &Value) -> String{
	format!("Operator ({}) error! Top of stack must \
		be of type Char! Attempted value: {}", op_type, v)
}

//Conumes a character and pushes a boolean saying whether or not it's whitespace.
pub fn whitespace_detect(s: &mut State) -> Result<(), String>{
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
pub fn alpha_char_detect(s: &mut State) -> Result<(), String>{
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
pub fn num_char_detect(s: &mut State) -> Result<(), String>{
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

pub fn invalid_types_for_obj_add_or_mut(op_type: &str, v1: &Value, v2: &Value, v3: &Value) -> String{
	format!("Operator ({}) error! Third to top of stack must be of type ObjectBox, \
		second to top must be type StringBox, and top must be type Value! \
		Attempted values: {}, {}, and {}", op_type, v1, v2, v3)
}

//Adds a field to the given object and pushes the mutated object back.
pub fn add_field(s: &mut State) -> Result<(), String>{
	let res = match s.pop3(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b)), Some(v)) => {
			match (s.validate_box(a), s.validate_box(b)){
				(true, true) => {
					let mut obj_to_mut = std::mem::take(&mut s.heap[a].0);
					if let (HeapValue::Object(o), HeapValue::String(st)) = (&mut obj_to_mut, &s.heap[b].0){
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

pub fn invalid_types_for_get_or_rem(op_type: &str, v1: &Value, v2: &Value) -> String{
	format!("Operator ({}) error! Second to top of stack must \
		be type ObjectBox and top must be type StringBox! \
		Attempted values: {} and {}", op_type, v1, v2)
}

pub fn field_not_in_obj_err(op_type: &str, box_num: usize, field_name: &str) -> String{
	format!("Operator ({}) error! Field \"{}\" doesn't exist \
		in ObjectBox {} ! Try adding it!", op_type, field_name, box_num) 
}

//Given an object and string box, conumes the boxes 
// and pushes the value at that key if it exists.
pub fn get_field(s: &mut State) -> Result<(), String>{
	let res = match s.pop2(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
			match (s.validate_box(a), s.validate_box(b)){
				(true, true) => {
					if let (HeapValue::Object(o), HeapValue::String(st)) = (&s.heap[a].0, &s.heap[b].0){
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
pub fn is_valid_mutation(a: &Value, b: &Value) -> bool{
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

pub fn invalid_mutation_error(op_type: &str, thing_being_mutated: &str, thing_name: &str, v1: &Value, v2: &Value) -> String{
	format!("Operator ({}) error! Invalid mutation of {} {} ! \
		Unable to mutate {} to {}", op_type, thing_being_mutated, thing_name, v1, v2)
}

//Mutates the field to a new value in an object if it exists and it's a valid mutation.
pub fn mut_field(s: &mut State) -> Result<(), String>{
	let res = match s.pop3(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b)), Some(v)) => {
			match (s.validate_box(a), s.validate_box(b)){
				(true, true) => {
					let mut obj_to_mut = std::mem::take(&mut s.heap[a].0);
					if let (HeapValue::Object(o), HeapValue::String(st)) = (&mut obj_to_mut, &s.heap[b].0){
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
pub fn remove_field(s: &mut State) -> Result<(), String>{
	let res = match s.pop2(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
			match (s.validate_box(a), s.validate_box(b)){
				(true, true) => {
					let mut obj_to_mut = std::mem::take(&mut s.heap[a].0);
					if let (HeapValue::Object(o), HeapValue::String(st)) = (&mut obj_to_mut, &s.heap[b].0){
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
pub fn string_compare(s: &mut State) -> Result<(), String>{
	let res = match s.pop2(){
		(Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
			match (s.validate_box(a), s.validate_box(b)){
				(true, true) => {
					if let (HeapValue::String(str_a), HeapValue::String(str_b)) = (&s.heap[a].0, &s.heap[b].0){
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
pub fn bit_or(s: &mut State) -> Result<(), String>{
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
pub fn bit_and(s: &mut State) -> Result<(), String>{
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
pub fn bit_xor(s: &mut State) -> Result<(), String>{
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
pub fn bit_not(s: &mut State) -> Result<(), String>{
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
pub fn shift<T: std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> + Default>(n: T, shift_n: isize) -> T{
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
pub fn bit_shift(s: &mut State) -> Result<(), String>{
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
pub fn max_isize(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::IntSize(isize::MAX));
	Ok(())
}

//Pushes maximum value for usize datatype to stack.
pub fn max_usize(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::UIntSize(usize::MAX));
	Ok(())
}

//Pushes maximum value for i8 datatype to stack.
pub fn max_i8(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::Int8(i8::MAX));
	Ok(())
}

//Pushes maximum value for i16 datatype to stack.
pub fn max_i16(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::Int16(i16::MAX));
	Ok(())
}

//Pushes maximum value for i32 datatype to stack.
pub fn max_i32(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::Int32(i32::MAX));
	Ok(())
}

//Pushes maximum value for i64 datatype to stack.
pub fn max_i64(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::Int64(i64::MAX));
	Ok(())
}

//Pushes maximum value for i128 datatype to stack.
pub fn max_i128(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::Int128(i128::MAX));
	Ok(())
}

//Pushes maximum value for u8 datatype to stack.
pub fn max_u8(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::UInt8(u8::MAX));
	Ok(())
}

//Pushes maximum value for u16 datatype to stack.
pub fn max_u16(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::UInt16(u16::MAX));
	Ok(())
}

//Pushes maximum value for u32 datatype to stack.
pub fn max_u32(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::UInt32(u32::MAX));
	Ok(())
}

//Pushes maximum value for u64 datatype to stack.
pub fn max_u64(s: &mut State) -> Result<(), String>{
	s.stack.push(Value::UInt64(u64::MAX));
	Ok(())
}

//Pushes maximum value for u128 datatype to stack.
pub fn max_u128(s: &mut State) -> Result<(), String>{
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

pub fn numeric_error_cast_string(v: Value, t: &str, r: &str, buff_len: usize) -> String{
	let ops = ["cast" , "castTo"];
	let op_type = ops[(buff_len > 0) as usize];
	format!("Operator ({}) error! Failed to cast {} to type {} because: {}", op_type, v, t, r)
}

pub fn invalid_cast_error(t: &str) -> String{
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
		Err(reason) => Err(numeric_error_cast_string(v, c, &reason, s.buffer.len())), 
		_ => Err(should_never_get_here_for_func("integer_cast_action")),
	}
}

//Generates String for error involving casting Strings to stuff.
pub fn string_cast_error(bn: usize, str_contents: &str, t: &str, reason: &str, buff_len: usize) -> String{
	let ops = ["cast", "castTo"];
	let op_type = ops[(buff_len > 0) as usize];
	format!("Operator ({}) error! Failed to cast \
		StringBox {} (\"{}\") to type {} because: {}", op_type, bn, str_contents, t, reason)
}

//Performs the actual casting for both cast and castTo
pub fn general_cast_action(s: &mut State, v: Value, c: &str) -> Result<Value, String>{
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

				_ => Err(numeric_error_cast_string(Value::Float32(n), c, &(invalid_cast_error(c)), s.buffer.len())),
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

				_ => Err(numeric_error_cast_string(Value::Float64(n), c, &(invalid_cast_error(c)), s.buffer.len())),
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

				_ => Err(numeric_error_cast_string(Value::Char(ch), c, &(invalid_cast_error(c)), s.buffer.len())),
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

				_ => Err(numeric_error_cast_string(Value::Boolean(b), c, &(invalid_cast_error(c)), s.buffer.len())),
			}

		},

		Value::StringBox(string_num) => {
			if s.validate_box(string_num){
				if let HeapValue::String(st) = &s.heap[string_num].0{
					match c {
						"isize" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::IntSize(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"usize" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::UIntSize(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						
						"i8" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Int8(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"i16" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Int16(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"i32" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Int32(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"i64" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Int64(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"i128" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Int128(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},

						"u8" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::UInt8(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"u16" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::UInt16(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"u32" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::UInt32(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"u64" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::UInt64(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"u128" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::UInt128(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},

						"f32" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Float32(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},
						"f64" => {
							match (*st).parse(){
								Ok(casted) => Ok(Value::Float64(casted)),
								Err(e) => Err(string_cast_error(string_num, st, c, &e.to_string(), s.buffer.len())),
							}
						},

						"Boolean" => {
							if st == "True" || st == "true"{
								Ok(Value::Boolean(true))
							}else if st == "False" || st == "false"{
								Ok(Value::Boolean(false))
							}else{
								Err(string_cast_error(string_num, st, c, 
									&String::from("provided string is not a valid Boolean"), s.buffer.len()))
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

						_ => Err(string_cast_error(string_num, st, c, &invalid_cast_error(c), s.buffer.len())),
					}
				}else{
					Err(should_never_get_here_for_func("general_cast_action"))
				}
			}else{
				let ops = ["cast", "castTo"];
				let op_type = ops[(s.buffer.len() > 0) as usize];
				Err(bad_box_error(op_type, "StringBox", "NA", string_num, usize::MAX, false))
			}
		},

		Value::ListBox(ls_num) => {
			if s.validate_box(ls_num){
				let ref ls = &s.heap[ls_num].0;
				match c{
					"List" => Ok(Value::ListBox(ls_num)), //No-op
					"String" => {
						let ls_str = format!("{}", ls);
						let new_bn = s.insert_to_heap(HeapValue::String(ls_str[5..].to_string()));
						Ok(Value::StringBox(new_bn))
					},
					_ => {
						let ops = ["cast", "castTo"];
						let op_type = ops[(s.buffer.len() > 0) as usize];
						Err(format!("Operator ({}) error! Failed \
						to cast ListBox {} to {} because: {}", op_type, ls_num, c, &invalid_cast_error(c)))
					},                 
				}
			}else{
				let ops = ["cast", "castTo"];
				let op_type = ops[(s.buffer.len() > 0) as usize];
				Err(bad_box_error(op_type, "ListBox", "NA", ls_num, usize::MAX, false))
			}
		},

		Value::ObjectBox(obj_num) => {
			if s.validate_box(obj_num){
				let ref obj = &s.heap[obj_num].0;
				match c {
					"Object" => Ok(Value::ObjectBox(obj_num)),
					"String" => {
						let obj_str = format!("{}", obj);
						let new_bn = s.insert_to_heap(HeapValue::String(obj_str[7..].to_string()));
						Ok(Value::StringBox(new_bn))
					},
					_ => {
						let ops = ["cast", "castTo"];
						let op_type = ops[(s.buffer.len() > 0) as usize];
						Err(format!("Operator ({}) error! Failed \
						to cast ObjectBox {} to {} because: {}", op_type, obj_num, c, &invalid_cast_error(c)))
					},  
				}
			}else{
				let ops = ["cast", "castTo"];
				let op_type = ops[(s.buffer.len() > 0) as usize];
				Err(bad_box_error(op_type, "ObjectBox", "NA", obj_num, usize::MAX, false))
			}
		},

		Value::NULLBox => {
		   match c {
				"String" => {
					let new_bn = s.insert_to_heap(HeapValue::String("NULLBox".to_string()));
					Ok(Value::StringBox(new_bn))
				},
				_ => {
					let ops = ["cast", "castTo"];
					let op_type = ops[(s.buffer.len() > 0) as usize];
					Err(format!("Operator ({}) error! Failed \
					to cast NULLBox to {} because: {}", op_type, c, &invalid_cast_error(c)))
				},
		   }     
		},

		_ => {
			let ops = ["cast", "castTo"];
			let op_type = ops[(s.buffer.len() > 0) as usize];
			Err(format!("Operator ({}) error! Value being casted must be a castable type! Attempted value: {}", op_type, v))
		}, 
	};

	res
}

//Performs all valid casts in existence wherein the top 
// of the stack tries to be casted to another data type.
pub fn cast_stuff(s: &mut State) -> Result<(), String>{
	if s.buffer.len() == 0{
		let res = match s.pop2(){
			(Some(v), Some(Value::StringBox(bn))) => {
				if s.validate_box(bn){
					if let HeapValue::String(st) = &s.heap[bn].0{
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
pub fn io_needing_one_item_on_stack_error(op_type: &str, needed_type: &str, attempted_value: &Value) -> String{
	format!("Operator ({}) error! Top of stack must be type {}! \
		Attempted value: {}", op_type, needed_type, attempted_value)
}

//Prints contents of a string box and consumes it. 
// Like everything else, the stringbox is not free'd.
pub fn print_line(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(st) = &s.heap[bn].0{
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
pub fn read_line_from_in(s: &mut State) -> Result<(), String>{
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
pub fn print_char(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::Char(c)) => {
			print!("{}", c);
			Ok(())
		},
		Some(v) => Err(io_needing_one_item_on_stack_error("printChar", "Char", &v)),
		None => Err(needs_n_args_only_n_provided("printChar", "One", "none")),
	}
}

pub fn unable_to_read_error(op_type: &str, reason: &str) -> String{
	format!("Operator ({}) error! Unable to read from stdin because: {}", op_type, reason)
}

//Reads in one Char from stdin and pushes it to the stack.
//SEEMS TO WORK WELL ENOUGH, ASSUMING THERE'S NO CRAZY EDGE CASE THAT WOULD DESTROY THIS
pub fn read_char(s: &mut State) -> Result<(), String>{
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
pub fn print_string(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(st) = &s.heap[bn].0{
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
pub fn read_from_in(s: &mut State) -> Result<(), String>{
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
pub fn debug_stack_print(s: &mut State) -> Result<(), String>{
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
pub fn debug_heap_print(s: &mut State) -> Result<(), String>{
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
pub fn write_data_to_file(s: &mut State) -> Result<(), String>{
	match s.pop2(){
		(Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
			match (s.validate_box(a), s.validate_box(b)){
				(true, true) => {
					if let (HeapValue::String(file_name), HeapValue::String(string_to_write)) = (&s.heap[a].0, &s.heap[b].0){
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

pub fn single_arg_file_io_type_error(op_type: &str, v: &Value) -> String{
	format!("Operator ({}) error! Top of stack must \
	 be type StringBox! Attempted value: {}", op_type, v)
}

//Reads the contents of a file into a string and allocates it on the heap.
pub fn read_data_from_file(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(file_name) = &s.heap[bn].0{
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
pub fn create_file_based_on_string(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(file_name) = &s.heap[bn].0{
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
pub fn delete_file_based_on_string(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(file_name) = &s.heap[bn].0{
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
pub fn file_exists(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(file_name) = &s.heap[bn].0{
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
pub fn query_type(s: &mut State) -> Result<(), String>{
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
pub fn leave_scope_if_true(s: &mut State) -> Result<(), String>{
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
pub fn throw_custom_error(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			if s.validate_box(bn){
				if let HeapValue::String(err_str) = &s.heap[bn].0{
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
pub fn get_args(s: &mut State) -> Result<(), String>{
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
pub fn is_valid_box(s: &mut State) -> Result<(), String>{
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
pub fn time_unix_now(s: &mut State) -> Result<(), String>{
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

//Causes the program to pause for a specified number of seconds.
//Accepts either f64 or f32.
pub fn time_wait(s: &mut State) -> Result<(), String>{
	match s.pop(){
		Some(Value::Float64(t)) => {
			let seconds = t as u64;
			let nanos: u32 = ((t - (seconds as f64)) * 1000000000f64) as u32;
			let sleep_durr = time::Duration::new(seconds, nanos);
			thread::sleep(sleep_durr);
			Ok(())
		},
		Some(Value::Float32(t)) => {
			let seconds = t as u64;
			let nanos: u32 = ((t - (seconds as f32)) * 1000000000f32) as u32;
			let sleep_durr = time::Duration::new(seconds, nanos);
			thread::sleep(sleep_durr);
			Ok(())
		},
		Some(v) => Err(format!("Operator (timeWait) error! Top of stack must be type f32 or f64! Attempted value: {}", v)),
		None => Err(needs_n_args_only_n_provided("timeWait", "One", "none")),
	}
}

//Creates a frame for local variables to use.
pub fn create_frame(size: usize) -> Vec<(Value, bool)>{
	let mut frame: Vec<(Value, bool)> = Vec::with_capacity(size);
	for _ in 0..size{
		frame.push((Value::NULLBox, false));
	}
	frame
}

//Sets all validity booleans to false, allowing frame 
// to be reused with reletively low cost.
pub fn recycle_frame(frame: &mut Vec<(Value, bool)>){
	for i in 0..frame.len(){
		frame[i].1 = false;
	}
}

impl State{
	//Creates a new state.
	pub fn new(num_unique_var_names: usize) -> Self{
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
			get_args, is_valid_box, time_unix_now, time_wait
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
	pub fn insert_to_heap(&mut self, ins_val: HeapValue) -> usize{
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
	pub fn validate_box(&self, box_num: usize) -> bool{
		box_num < self.heap.len() && self.heap[box_num].1
	}

	//Frees a cell in a heap or does nothing if it's invalid already.
	pub fn free_heap_cell(&mut self, box_num: usize){
		if self.validate_box(box_num){
			self.heap[box_num].1 = false;
			self.free_list.push(box_num);
		}
	}

	pub fn pop(&mut self) -> Option<Value>{
		self.stack.pop()
	}

	pub fn pop2(&mut self) -> (Option<Value>, Option<Value>){
		let top = self.pop();
		let second_to_top = self.pop();
		(second_to_top, top)
	}

	pub fn pop3(&mut self) -> (Option<Value>, Option<Value>, Option<Value>){
		let top = self.pop();
		let second_to_top = self.pop();
		let third_to_top = self.pop();
		(third_to_top, second_to_top, top)
	}

}

//Adds one to the current frame count. 
// This means that any local variables would be created a frame deeper than before.
pub fn add_frame(s: &mut State){
    s.curr_frame += 1
}

//Removes hashmap for local variables in current frame before leaving, 
// unless at global scope where nothing happens.
pub fn remove_frame(s: &mut State){
    if s.curr_frame > 0{
        if s.frames[s.frames.len() - 1].0 == s.curr_frame{
            let mut popped = s.frames.pop().unwrap();
            s.frame_pool.push(std::mem::take(&mut popped.1));
        }
        s.curr_frame -= 1;
    }
}

//Removes the stack frame before then creating the appropriate error string.
pub fn error_and_remove_frame(s: &mut State, err: String) -> Result<bool, String>{
    remove_frame(s);
    Err(err)
}
