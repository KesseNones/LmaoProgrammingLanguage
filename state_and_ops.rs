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
use std::mem::discriminant;  
use std::ops::{Shl, Shr};

//Useful for setting stuff to null.
const NULL: Value = Value::NULLBox;

#[derive(Clone)]
pub struct Stack{
	data: Vec<Value>,
}
impl Stack{
	pub fn new() -> Self{
		Stack{data: Vec::new()}
	}

	pub fn pop(&mut self) -> Option<Value>{
		self.data.pop()
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
	
	pub fn push(&mut self, v: Value){
		self.data.push(v)	
	}

	pub fn clear_stack(&mut self){
		self.data.clear()
	}
	pub fn read_stack(&self) -> &Vec<Value>{
		&self.data
	}
	pub fn size(&self) -> usize{
		self.data.len()
	}
}

#[derive(Clone)]
pub struct Heap{
	heap: Vec<(HeapValue, bool)>,
	free_list: Vec<usize>
}
impl Heap{
	pub fn new() -> Self{
		Heap{
			heap: Vec::new(),
			free_list: Vec::new()
		}
	}		

	//Inserts item into Heap and returns a Box pointing to it.
	pub fn insert_to_heap(&mut self, ins_val: HeapValue) -> Value{
		//Inserts to heap or reuses cell if one exists.
		let mut box_num: usize = 0;
		match self.free_list.pop(){
			Some(index) => {
				self.heap[index] = (ins_val, true);
				box_num = index;	
			},
			None => {
				box_num = self.heap.len();
				self.heap.push((ins_val, true));
			},
		}
		
		//Wraps box number into proper box value.
		match &self.heap[box_num].0{
			HeapValue::String(_) => Value::StringBox(box_num),
			HeapValue::List(_) => Value::ListBox(box_num),
			HeapValue::Object(_) => Value::ObjectBox(box_num),
			HeapValue::Primitive(_) => Value::MiscBox(box_num),
		}

	}
		
	//If it's an actual Box type with a number, 
	// it kicks it back, otherwise None.
	fn box_to_int(&self, v: Value) -> Option<usize>{
		match v {
			Value::StringBox(n) => Some(n),
			Value::ListBox(n) => Some(n),
			Value::ObjectBox(n) => Some(n),
			Value::MiscBox(n) => Some(n),
			_ => None,
		}	
	}

	pub fn box_type_to_string(&self, v: Value) -> Option<String>{
		match v {
			Value::StringBox(n) => Some("StringBox".to_string()),
			Value::ListBox(n) => Some("ListBox".to_string()),
			Value::ObjectBox(n) => Some("ObjectBox".to_string()),
			Value::MiscBox(n) => Some("MiscBox".to_string()),
			_ => None,
		}	
	}

	pub fn is_box(&self, v: Value) -> bool{
		match v {
			Value::StringBox(n) => true,
			Value::ListBox(n) => true,
			Value::ObjectBox(n) => true,
			Value::MiscBox(n) => true,
			_ => false,
		}	
	}

	pub fn is_string_box(&self, v: Value) -> bool{
		if let Value::StringBox(_) = v {
			true
		}else{
			false
		}
	}

	pub fn is_list_box(&self, v: Value) -> bool{
		if let Value::ListBox(_) = v {
			true
		}else{
			false
		}
	}

	pub fn is_object_box(&self, v: Value) -> bool{
		if let Value::ObjectBox(_) = v {
			true
		}else{
			false
		}
	}

	pub fn is_misc_box(&self, v: Value) -> bool{
		if let Value::MiscBox(_) = v {
			true
		}else{
			false
		}
	}
	
		
	fn is_valid_index(&self, idx: usize) -> bool{
		idx < self.heap.len() && self.heap[idx].1
	}

	//Returns a boolean based on whether or not the desired box number is valid.
	pub fn validate_box(&self, bx: Value) -> bool{
		if let Some(num) = self.box_to_int(bx) && self.is_valid_index(num){
			match (bx, &self.heap[num].0){
				(Value::StringBox(_), HeapValue::String(_)) => true,
				(Value::ListBox(_), HeapValue::List(_)) => true,
				(Value::ObjectBox(_), HeapValue::Object(_)) => true,
				(Value::MiscBox(_), HeapValue::Primitive(_)) => true,
				_ => false
			}
		}else{
			false
		}
	}

	//Frees heap cell the given box is pointing to.
	//Returns boolean based on if it succeeded or not.
	pub fn free_heap_cell(&mut self, bx: Value) -> bool{
		if self.validate_box(bx){
			let box_num = self.box_to_int(bx).unwrap();
			self.heap[box_num].1 = false;
			self.free_list.push(box_num);
			true
		}else{
			false
		}
	}

	//Gets a reference to an item on the heap, or nothing 
	// if the box is invalid or isn't a box.
	pub fn get_heap_ref(&self, bx: Value) -> Option<&HeapValue>{
		if self.validate_box(bx){
			let num = self.box_to_int(bx).unwrap();	
			Some(&self.heap[num].0)
		}else{
			None
		}
	}

	//Same as ref but it returns a mutable reference.
	pub fn get_heap_ref_mut(&mut self, bx: Value) -> Option<&mut HeapValue>{
		if self.validate_box(bx){
			let num = self.box_to_int(bx).unwrap();	
			Some(&mut self.heap[num].0)
		}else{
			None
		}
	}

	//Works like ref_mut but it gets up to two at once.
	//Makes the borrow checker happy.
	pub fn get_heap_ref_mut_and_ref(&mut self, bx1: Value, bx2: Value) -> 
	(Option<&mut HeapValue>, Option<&HeapValue>)
	{
		match (self.validate_box(bx1), self.validate_box(bx2)){
			(true, true) => {
				let num1 = self.box_to_int(bx1).unwrap();
				let num2 = self.box_to_int(bx2).unwrap();

				if num1 == num2 {(None, None)}
				else{
					if num1 < num2{
						let (left, right) = self.heap.split_at_mut(num2);
						(Some(&mut left[num1].0), Some(&right[0].0))
					}else{
						let (left, right) = self.heap.split_at_mut(num1);
						(Some(&mut left[0].0), Some(&right[num2].0))
					}
				}
			},
			(true, false) => (self.get_heap_ref_mut(bx1), None),
			(false, true) => (self.get_heap_ref_mut(bx2), None),
			(false, false) => (None, None),
		}
	}

	pub fn read_heap(&self) -> &Vec<(HeapValue, bool)>{
		&self.heap
	}

	pub fn read_free_list(&self) -> &Vec<usize>{
		&self.free_list
	}

	//Returns true if within heap bounds and cell is valid.
	pub fn cell_number_is_valid(&self, box_num: usize) -> bool {
		box_num < self.heap.len() && self.heap[box_num].1	
	}

}

#[derive(Clone)]
pub struct Variables{
	vars: HashMap<String, Value>,
	curr_scope: usize,
	loc_frames: Vec<(usize, HashMap<String, Value>)>
}
impl Variables{
	pub fn new() -> Self{
		Variables{
			vars: HashMap::new(),
			curr_scope: 0,
			loc_frames: Vec::new()
		}
	}

	//Attempts to create a variable of name name with value val. 
	// Returns boolean of if it succeeded.
	pub fn mak_var(&mut self, name: &str, val: Value) -> bool{
		if !self.vars.contains_key(name){
			self.vars.insert(name.to_string(), val);
			true	
		}else{
			false
		}
	}

	//Attempts to get a variable by a name.
	pub fn get_var(&self, name: &str) -> Option<Value>{
		self.vars.get(name).copied()
	}


	//Mutates a variable to the new value.
	//Returns an integer indicating results.
	// 0 -> it worked
	// 1 -> the variable doesn't exist
	// 2 -> the mutation is invalid
	pub fn mut_var(&mut self, name: &str, new_val: Value) -> usize{
		match self.vars.get(name).copied(){
			Some(old_val) => {
				if is_valid_mutation(old_val, new_val){
					self.vars.insert(name.to_string(), new_val);
					0
				}
				else{2}
			},
			None => 1,
		}
	}

	//Tries to delete a variable.
	// If it exists, it's gone.
	// If not, nothing happens.
	// Boolean indicates success.
	pub fn del_var(&mut self, name: &str) -> bool{
		match self.vars.get(name){
			Some(_) => {self.vars.remove(name); true},
			None => false
		}
	}
	pub fn add_frame(&mut self){
		self.curr_scope += 1;		
	}
	pub fn remove_frame(&mut self){
		//Pops frame from stack if current scope had a frame.
		let len = self.loc_frames.len();
		if len > 0 && self.loc_frames[len - 1].0 == self.curr_scope{
			self.loc_frames.pop();
		}
		self.curr_scope -= 1;		
	}

	//Creates a local variable, potentially a whole frame if needed.
	// Returns a boolean of success.
	// Failure is if the variable is already there.
	pub fn mak_loc(&mut self, name: &str, val: Value) -> bool{
		let mut len = self.loc_frames.len();

		//Creates local var frame if it doesn't exist for current scope.
		if len == 0 || self.loc_frames[len - 1].0 != self.curr_scope{
			self.loc_frames.push((self.curr_scope, HashMap::new()));
			len += 1;
		}

		if !self.loc_frames[len - 1].1.contains_key(name){
			self.loc_frames[len - 1].1.insert(name.to_string(), val);
			true	
		}else{false}
	}

	//Traverses back up the stack, searching each frame for the given variable.
	// If found, it's returned, if not, it's none.
	pub fn get_loc(&self, name: &str) -> Option<Value>{
		for frame in self.loc_frames.iter().rev(){
			if let Some(v) = frame.1.get(name).copied(){
				return Some(v);
			}
		}
		None
	}

	//Like mut_var but handles the multi-scoping logic.
	pub fn mut_loc(&mut self, name: &str, new_val: Value) -> usize{
		for frame in self.loc_frames.iter_mut().rev(){
			if let Some(v) = frame.1.get(name).copied(){
				if is_valid_mutation(v, new_val){
					frame.1.insert(name.to_string(), new_val);
					return 0;
				}else{return 2;}
			}
		}
		1
	}

}

#[derive(Clone)]
pub struct Functions{
	fns: HashMap<String, Rc<ASTNode>>
}
impl Functions{
	pub fn new() -> Self{
		Functions{fns: HashMap::new()}	
	}
	
	//Attempts to define a function. 
	// true -> it worked.
	// false -> it's already defined.
	pub fn func_def(&mut self, name: &str, body: Rc<ASTNode>) -> bool{
		if !self.fns.contains_key(name)	{
			self.fns.insert(name.to_string(), Rc::clone(&body));
			true
		}else{
			false
		}
	}

	pub fn get_body(&self, name: &str) -> Option<Rc<ASTNode>> {
		self.fns.get(name).cloned()
	}

	pub fn function_exists(&self, name:&str) -> bool{
		self.fns.contains_key(name)
	}

}

pub enum RetCode{
	Normal,
	LeavingScopeEarly,
}
type OpFunction = 
fn(&mut Stack, &mut Heap, Option<&str>) -> Result<RetCode, String>;

//Used for numerical operators like +, -, *, etc.
pub fn numerical_type_error_string(op_name: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Operand types must match and be numeric types! Attempted values: {} and {}", op_name, v1, v2)
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

pub fn push_val_or_err(r: Result<Value, String>, s: &mut Stack) -> Result<RetCode, String>{
	match r{
		Ok(v) => {
			s.push(v);
			Ok(RetCode::Normal)
		},
		Err(e) => Err(e),
	}
}


//Adds two values of matching numerical types together, pusing the result to the stack.
pub fn add(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "+";
	macro_rules! add_match{
		($($var:ident),* $(,)?) => {
			let res = match s.pop2(){
				$((Some(Value::$var(a)), Some(Value::$var(b))) => {
					Ok(Value::$var(a.wrapping_add(b)))
				}, )*
				(Some(Value::Float32(a)), Some(Value::Float32(b))) => {
					Ok(Value::Float32(a + b))
				},
				(Some(Value::Float64(a)), Some(Value::Float64(b))) => {
					Ok(Value::Float64(a + b))
				},

				(Some(a), Some(b)) => {
					Err(numerical_type_error_string(op_name, a, b))
				},

				(None, Some(_)) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
				},

				(None, None) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
				},

				_ => Err(should_never_get_here_for_func("add")),
			};	
			push_val_or_err(res, s)
		};
	}
	add_match!{IntSize, UIntSize, 
		Int8, Int16, Int32, Int64, Int128,
		UInt8, UInt16, UInt32, UInt64, UInt128,
	}
	
}

//Subtracts two values of matching numerical types, pusing the result to the stack.
pub fn sub(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "-";
	macro_rules! sub_match{
		($($var:ident),* $(,)?) => {
			let res = match s.pop2(){
				$((Some(Value::$var(a)), Some(Value::$var(b))) => {
					Ok(Value::$var(a.wrapping_sub(b)))
				}, )*
				(Some(Value::Float32(a)), Some(Value::Float32(b))) => {
					Ok(Value::Float32(a - b))
				},
				(Some(Value::Float64(a)), Some(Value::Float64(b))) => {
					Ok(Value::Float64(a - b))
				},

				(Some(a), Some(b)) => {
					Err(numerical_type_error_string(op_name, a, b))
				},

				(None, Some(_)) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
				},

				(None, None) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
				},

				_ => Err(should_never_get_here_for_func("sub")),
			};	
			push_val_or_err(res, s)
		};
	}
	sub_match!{IntSize, UIntSize, 
		Int8, Int16, Int32, Int64, Int128,
		UInt8, UInt16, UInt32, UInt64, UInt128,
	}
}

//Pops two items from top of stack and multiplies them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands.
pub fn mult(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "*";
	macro_rules! sub_match{
		($($var:ident),* $(,)?) => {
			let res = match s.pop2(){
				$((Some(Value::$var(a)), Some(Value::$var(b))) => {
					Ok(Value::$var(a.wrapping_mul(b)))
				}, )*
				(Some(Value::Float32(a)), Some(Value::Float32(b))) => {
					Ok(Value::Float32(a * b))
				},
				(Some(Value::Float64(a)), Some(Value::Float64(b))) => {
					Ok(Value::Float64(a * b))
				},

				(Some(a), Some(b)) => {
					Err(numerical_type_error_string(op_name, a, b))
				},

				(None, Some(_)) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
				},

				(None, None) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
				},

				_ => Err(should_never_get_here_for_func("mult")),
			};	
			push_val_or_err(res, s)
		};
	}
	sub_match!{IntSize, UIntSize, 
		Int8, Int16, Int32, Int64, Int128,
		UInt8, UInt16, UInt32, UInt64, UInt128,
	}
}

pub fn division_by_zero_error(v1: Value, v2: Value) -> String{
	format!("Operator (/) error! Division by zero error between {} and {}!", v1, v2)
}

//Pops two items from top of stack and divides them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as division by zero.
pub fn div(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "/";
	macro_rules! div_match{
		($($var:ident, $type:ty),* $(,)?) => {
			let res = match s.pop2(){
				$((Some(Value::$var(a)), Some(Value::$var(b))) => {
					let v1 = Value::$var(a);
					let v2 = Value::$var(b);
					if b != <$type>::default(){
						Ok(Value::$var(a / b))	
					}else{
						Err(division_by_zero_error(v1, v2))	
					}
				}, )*

				(Some(a), Some(b)) => {
					Err(numerical_type_error_string(op_name, a, b))
				},

				(None, Some(_)) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
				},

				(None, None) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
				},

				_ => Err(should_never_get_here_for_func("div")),
			};	
			push_val_or_err(res, s)
		};
	}
	div_match!{IntSize, isize, UIntSize, usize,
		Int8, i8, Int16, i16, Int32, i32, Int64, i64, Int128, i128,
		UInt8, u8, UInt16, u16, UInt32, u32, UInt64, u64,  UInt128, u128,
		Float32, f32, Float64, f64
	}
}

//Creates string for error return in modulo function.
pub fn modulo_by_zero_error(v1: Value, v2: Value) -> String{
	format!("Operator (%) error! Modulo by zero error between {} and {}!", v1, v2)
}

//Pops two items from top of stack and modulos them, pushing result to stack.
// Throws errors for non-matching types and insufficient operands, as well as modulo by zero.
pub fn modulo(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "% or mod";
	macro_rules! mod_match{
		($($var:ident),* $(,)?) => {
			let res = match s.pop2(){
				$((Some(Value::$var(a)), Some(Value::$var(b))) => {
					let v1 = Value::$var(a);
					let v2 = Value::$var(b);
					if b != 0{
						Ok(Value::$var(a / b))	
					}else{
						Err(modulo_by_zero_error(v1, v2))	
					}
				}, )*

				(Some(a), Some(b)) => {
					Err(numerical_type_error_string(op_name, a, b))
				},

				(None, Some(_)) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
				},

				(None, None) => {
					Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
				},

				_ => Err(should_never_get_here_for_func("modulo")),
			};	
			push_val_or_err(res, s)
		};
	}
	mod_match!{IntSize, UIntSize, 
		Int8, Int16, Int32, Int64, Int128,
		UInt8, UInt16, UInt32, UInt64, UInt128,
	}
}

//Adds two values of matching numerical types together, pusing the result to the stack.
pub fn power(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
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
pub fn swap(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(a), Some(b)) => {
			s.push(b);
			s.push(a);
			Ok(RetCode::Normal)
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided("swap", "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided("swap", "Two", "none")),
		_ => Err(should_never_get_here_for_func("swap")),
	}
}

//Removes the top item from the stack 
// or errors out if stack is empty.
pub fn drop(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop(){
		Some(_) => Ok(RetCode::Normal),
		None => Err(needs_n_args_only_n_provided("drop", "One", "none")),
	}
}

//Clears existing stack to be empty. 
// This can be useful if you want a clean stack without doing a ton of drops.
pub fn drop_stack(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.clear_stack();
	Ok(RetCode::Normal)
}

//Rotates top three items on stack, 
// putting the top item below the previous two.
pub fn rot(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop3(){
		(Some(a), Some(b), Some(c)) => {
			s.push(c);
			s.push(a);
			s.push(b);

			Ok(RetCode::Normal)
		},
		(None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided("rot", "Three", "only two")),
		(None, None, Some(_)) => Err(needs_n_args_only_n_provided("rot", "Three", "only one")),
		(None, None, None) => Err(needs_n_args_only_n_provided("rot", "Three", "none")),
		_ => Err(should_never_get_here_for_func("rot")),
	}
}

//Very literally just copies the top element of the stack and pushes it. 
// If it's a box, the box itself is copied, not the data it contains.
pub fn dup(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop(){
		Some(v) => {
			s.push(v);
			s.push(v);
			Ok(RetCode::Normal)
		},
		None => Err(needs_n_args_only_n_provided("dup", "One", "none")),
	}
}

//Works like dup but duplicates the data held by box types 
// and creates a new box to hold the duplicated data.
pub fn deep_dup(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "deepDup";
	if let Some(v) = s.pop(){
		if let Some(val) = h.get_heap_ref(v){
			let dupped = val.clone();
			let new_box = h.insert_to_heap(dupped);
			s.push(v);
			s.push(new_box);
			Ok(RetCode::Normal)
		}else{
			Err(bad_box_error(op_name, v, Value::NULLBox, false))
		}
	}else{
		Err(needs_n_args_only_n_provided(op_name, "One", "none"))
	}
}

//Used in == and != operators to generate an error.
pub fn equality_error(op_type: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Comparisons of equality and inequality \
		must have matching types! Attempted values: {} and {}", op_type, v1, v2)
}

//Checks for equality between two data types. For boxes it checks to see 
// if the box numbers are equal and for NULL box it checks for self-equality.
//Consumes both items from stack and pushes resulting boolean based on their comparison.
pub fn is_equal(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(Value::StringBox(_) | Value::ListBox(_) | Value::ObjectBox(_) | Value::MiscBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::StringBox(_) | Value::ListBox(_) | Value::ObjectBox(_) | Value::MiscBox(_)))   => {
			s.push(Value::Boolean(false));
			Ok(RetCode::Normal)
		},
		(Some(a), Some(b)) => {
			if discriminant(&a) == discriminant(&b) {
				s.push(Value::Boolean(a == b));
				Ok(RetCode::Normal)
			}else{
				Err(equality_error("==", a, b))
			}
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("==", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("==", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("is_equal")),
	}
	
}

//Checks for inequality between two data types. For boxes it checks to see 
// if the box numbers are equal and for NULL box it checks for self-inequality.
//Consumes both items from stack and pushes resulting boolean based on their comparison.
pub fn is_not_equal(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(Value::StringBox(_) | Value::ListBox(_) | Value::ObjectBox(_) | Value::MiscBox(_)), Some(Value::NULLBox)) | (Some(Value::NULLBox), Some(Value::StringBox(_) | Value::ListBox(_) | Value::ObjectBox(_) | Value::MiscBox(_)))   => {
			s.push(Value::Boolean(true));
			Ok(RetCode::Normal)
		},
		(Some(a), Some(b)) => {
			if discriminant(&a) == discriminant(&b) {
				s.push(Value::Boolean(a != b));
				Ok(RetCode::Normal)
			}else{
				Err(equality_error("!=", a, b))
			}
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("!=", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("!=", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("is_not_equal")),
	}
}

pub fn comparison_error(op_type: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Non-equality comparison operators need matching \
		non-null types to function! Attempted values: {} and {}", op_type, v1, v2)
}

//Compares two values on stack to see if the second to top is greater than the top.
pub fn is_greater_than(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(a), Some(b)) => {
			if discriminant(&a) == discriminant(&b) {
				s.push(Value::Boolean(a > b));
				Ok(RetCode::Normal)
			}else{
				Err(comparison_error(">", a, b))
			}
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(">", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided(">", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("is_greater_than")),
	}
}

//Compares two values on stack to see if the second to top is less than the top.
pub fn is_less_than(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(a), Some(b)) => {
			if discriminant(&a) == discriminant(&b) {
				s.push(Value::Boolean(a < b));
				Ok(RetCode::Normal)
			}else{
				Err(comparison_error("<", a, b))
			}
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("<", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("<", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("is_less_than")),
	}
}

//Compares two values on stack to see if the second to top is greater than or equal to the top.
pub fn is_greater_than_equal_to(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(a), Some(b)) => {
			if discriminant(&a) == discriminant(&b) {
				s.push(Value::Boolean(a >= b));
				Ok(RetCode::Normal)
			}else{
				Err(comparison_error(">=", a, b))
			}
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(">=", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided(">=", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("is_greater_than_equal_to")),
	}
}

//Compares two values on stack to see if the second to top is less than or equal to the top.
pub fn is_less_than_equal_to(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(a), Some(b)) => {
			if discriminant(&a) == discriminant(&b) {
				s.push(Value::Boolean(a <= b));
				Ok(RetCode::Normal)
			}else{
				Err(comparison_error("<=", a, b))
			}
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("<=", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("<=", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("is_less_than_equal_to")),
	}
}

//Creates error string for when a box involved is invalid.
pub fn bad_box_error(op_name: &str, b1: Value, b2: Value, is_two_boxes: bool) -> String{
	if !is_two_boxes{
		format!("Operator ({}) error! {} is not a valid Box/Box type!", op_name, b1)
	}else{
		format!("Operator ({}) error! {} and {} are not valid Boxes/Box types!", op_name, b1, b2)
	}
}

fn matching_box_error(v1: Value, v2: Value) -> String{
	format!("Operator (++) error! Concatentation requires the top two boxes to have different numbers! Attempted merging {} with {}", v1, v2)	
}

fn bad_concat_type_error(v1: Value, v2: Value) -> String{
	format!("Operator (++) error! Concatenation needs top two operands to be matching types of type StringBox or ListBox! Attempted values: {} and {}", v1, v2) 
}

//Concatenates two strings or two lists together.
// The second to top item gets a copy of the top's values appended. 
// The top is NOT free'd for simplicity.
pub fn concat(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "++";
	match s.pop2(){
		(Some(a), Some(b)) => {
			let are_both_string_boxes = h.is_string_box(a) && h.is_string_box(b);
			let are_both_list_boxes = h.is_list_box(a) && h.is_list_box(b);

			if (are_both_string_boxes || are_both_list_boxes) && (a == b){
				Err(matching_box_error(a, b))
			}else{
				match h.get_heap_ref_mut_and_ref(a, b){
					(Some(HeapValue::String(s1)), Some(HeapValue::String(s2))) => {
						s1.push_str(s2);
						s.push(a);
						Ok(RetCode::Normal)
					},
					(Some(HeapValue::List(ls1)), Some(HeapValue::List(ls2))) => {
						ls1.extend(ls2.iter().copied());
						s.push(a);
						Ok(RetCode::Normal)
					}
					(Some(_), Some(_)) => Err(bad_concat_type_error(a, b)),
					(Some(_), None) => Err(bad_box_error(op_name, b, NULL, false)),
					(None, Some(_)) => Err(bad_box_error(op_name, a, NULL, false)),
					(None, None) => Err(bad_box_error(op_name, a, b, false)),
				}
			}
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func("concat"))
	}
}

pub fn logical_operator_type_error(op_type: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Logical operation requires two operands \
		of type Boolean! Attempted values: {} and {}", op_type, v1, v2)
}

//Performs logical AND on two operands.
pub fn and(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
			s.push(Value::Boolean(a && b));
			Ok(RetCode::Normal)
		},
		(Some(a), Some(b)) => {
			Err(logical_operator_type_error("and/&&", a, b))
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("and/&&", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("and/&&", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("and")),
	}
}

//Performs logical OR on two operands.
pub fn or(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
			s.push(Value::Boolean(a || b));
			Ok(RetCode::Normal)
		},
		(Some(a), Some(b)) => {
			Err(logical_operator_type_error("or/||", a, b))
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("or/||", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("or/||", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("or")),
	}
}

//Performs logical XOR on two operands.
pub fn xor(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop2(){
		(Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
			s.push(Value::Boolean(a != b));
			Ok(RetCode::Normal)
		},
		(Some(a), Some(b)) => {
			Err(logical_operator_type_error("xor", a, b))
		},

		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided("xor", "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided("xor", "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("xor")),
	}
}

//Performs logical NOT on top of stack if boolean.
pub fn not(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop(){
		Some(Value::Boolean(x)) => {s.push(Value::Boolean(!x)); Ok(RetCode::Normal)},
		Some(x) => {
			Err(format!("Operator (not/!) error! Logical NOT requires \
				one Boolean type on the stack. Attempted value: {}", x))
		},
		None => Err(needs_n_args_only_n_provided("not/!", "One", "none")),
	}
}

fn list_push_type_error(v1: Value, v2: Value) -> String{
	format!("Operator (push/p) error! Push operator requires a ListBox/StringBox second to top on the stack and a Value/Char on top of the stack! Attempted values: {} and {}", v1, v2)
}

//Pushes a value to a list or a character to a string.
pub fn list_push(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "push/p";
	match s.pop2(){
		(Some(Value::ListBox(bn)), Some(v)) => {
			let lsbx = Value::ListBox(bn);
			if let Some(HeapValue::List(ls)) = h.get_heap_ref_mut(lsbx){
				ls.push(v);
				s.push(lsbx);
				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		(Some(Value::StringBox(bn)), Some(Value::Char(c))) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(st)) = h.get_heap_ref_mut(sbx){
				st.push(c);
				s.push(sbx);
				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		(Some(a), Some(b)) => {
			Err(list_push_type_error(a,b))
		},
		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
		},
		(None, None) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
		},
		_ => Err(should_never_get_here_for_func("list_push")),
	}
}

//Generates error string for 0 length errors for pop and fpop error.
pub fn pop_error(op_type: &str, collection_type: &str, op_detail: &str) -> String{
	format!("Operator ({}) error! {} needs to be greater \
		than length 0 for {} operation to actually pop something!", op_type, collection_type, op_detail)
}

//Pops from the end of a list/string and pushes the popped thing to the stack.
pub fn list_pop(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "pop/po";
	let res: Result<(Value, Value), String> = match s.pop(){
		Some(Value::ListBox(bn)) => {
			let lsbx = Value::ListBox(bn);
			if let Some(HeapValue::List(ls)) = h.get_heap_ref_mut(lsbx){
				match ls.pop(){
					Some(v) => Ok((lsbx, v)),
					None => Err(pop_error(op_name, "List", "pop")),
				}
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(st)) = h.get_heap_ref_mut(sbx){
				match st.pop(){
					Some(v) => Ok((sbx, Value::Char(v))),
					None => Err(pop_error(op_name, "String", "pop")),
				}
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => {
			Err(format!("Operator (pop/po) error! Top of stack needs \
				to be of type StringBox or ListBox! Attempted value: {}", v))
		},
		None => {
			Err(needs_n_args_only_n_provided(op_name, "One", "none"))
		},
	};  

	match res{
		Ok((v1, v2)) => {
			s.push(v1);
			s.push(v2);
			Ok(RetCode::Normal)
		},
		Err(e) => Err(e),
	}
}

//Pushes a value to the front of a list or a character to the front of a string.
pub fn list_front_push(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "fpush/fp";
	match s.pop2(){
		(Some(Value::ListBox(bn)), Some(v)) => {
			let lsbx = Value::ListBox(bn);
			if let Some(HeapValue::List(ls)) = h.get_heap_ref_mut(lsbx){
				ls.insert(0, v);
				s.push(lsbx);
				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		(Some(Value::StringBox(bn)), Some(Value::Char(c))) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(st)) = h.get_heap_ref_mut(sbx){
				st.insert(0, c);
				s.push(sbx);
				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		(Some(a), Some(b)) => {
			Err(format!("Operator (fpush/fp) error! Front push operator requires \
				a ListBox/StringBox second to top on the stack \
				and a Value/Char on top of the stack! Attempted values: {} and {}", a, b))
		},
		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
		},
		(None, None) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
		},
		_ => Err(should_never_get_here_for_func("list_front_push")),
	}
}

//Pops from the front of a list/string and pushes the popped thing to the stack.
pub fn list_front_pop(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "fpop/fpo";
	let res: Result<(Value, Value), String> = match s.pop(){
		Some(Value::ListBox(bn)) => {
			let lsbx = Value::ListBox(bn);
			if let Some(HeapValue::List(ls)) = h.get_heap_ref_mut(lsbx){
				if ls.len() > 0{
					Ok((Value::ListBox(bn), ls.remove(0)))
				}else{
					Err(pop_error(op_name, "List", "front pop"))
				}
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(st)) = h.get_heap_ref_mut(sbx){
				if st.len() > 0{
					Ok((Value::StringBox(bn), Value::Char(st.remove(0))))
				}else{
					Err(pop_error(op_name, "String", "front pop"))
				}
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => {
			Err(format!("Operator (fpop/fpo) error! Top of stack needs \
				to be of type StringBox or ListBox! Attempted value: {}", v))
		},
		None => {
			Err(needs_n_args_only_n_provided(op_name, "One", "none"))
		},
	};  

	match res{
		Ok((v1, v2)) => {
			s.push(v1);
			s.push(v2);
			Ok(RetCode::Normal)
		},
		Err(e) => Err(e),
	}
}

//Indexes into a list or string, 
// pushing the indexed item to the stack.
pub fn index(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "index";
	match s.pop2(){
		(Some(Value::ListBox(bn)), Some(Value::UIntSize(i))) => {
			let lsbx = Value::ListBox(bn);
			if let Some(HeapValue::List(ls)) = h.get_heap_ref(lsbx){
				if i < ls.len(){
					s.push(ls[i]);
					Ok(RetCode::Normal)
				}else{
					Err(format!("Operator (index) error! \
						Index {} is out of range of List of size {}", i, ls.len()))
				}
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		(Some(Value::StringBox(bn)), Some(Value::UIntSize(i))) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(st)) = h.get_heap_ref(sbx){
				if i < st.len(){
					s.push(Value::Char(st.chars().nth(i).unwrap()));
					Ok(RetCode::Normal)
				}else{
					Err(format!("Operator (index) error! \
						Index {} is out of range of String of size {}", i, st.len()))
				}
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		(Some(a), Some(b)) => {
			Err(format!("Operator (index) error! Index operator requires second \
				to top of stack to be either a ListBox or a StringBox, \
				and requires the top of the stack to be of type usize! \
				Attempted values: {} and {}", a, b))
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func(op_name)),
	}
}

//Determines length of string or list and pushes it to stack.
pub fn length(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "length/len";
	match s.pop(){
		Some(v) => {
			match h.get_heap_ref(v){
				Some(HeapValue::String(st)) => {
					s.push(Value::UIntSize(st.len())); 
					Ok(RetCode::Normal)
				},
				Some(HeapValue::List(l)) => {
					s.push(Value::UIntSize(l.len())); 
					Ok(RetCode::Normal)
				},
				Some(_) => Err(format!("Operator ({}) error! Top of stack must be type StringBox or ListBox! Given: {}", op_name, v)),
				None => Err(bad_box_error(op_name, v, NULL, false))
			}
		},
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Takes a string/list and pushes a boolean based on whether it's empty or not.
pub fn is_empty(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "isEmpty";
	match s.pop(){
		Some(v) => {
			match h.get_heap_ref(v){
				Some(HeapValue::String(st)) => {
					s.push(Value::Boolean(st.len() == 0)); 
					Ok(RetCode::Normal)
				},
				Some(HeapValue::List(l)) => {
					s.push(Value::Boolean(l.len() == 0)); 
					Ok(RetCode::Normal)
				},
				Some(_) => Err(format!("Operator ({}) error! Top of stack must be type StringBox or ListBox! Given: {}", op_name, v)),
				None => Err(bad_box_error(op_name, v, NULL, false))
			}
		},
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Clears a list/string to empty.
pub fn list_clear(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "clear";
	match s.pop(){
		Some(v) => {
			match h.get_heap_ref_mut(v){
				Some(HeapValue::String(st)) => {
					st.clear();
					Ok(RetCode::Normal)
				},
				Some(HeapValue::List(l)) => {
					l.clear();
					Ok(RetCode::Normal)
				},
				Some(_) => Err(format!("Operator ({}) error! Top of stack must be type StringBox or ListBox! Given: {}", op_name, v)),
				None => Err(bad_box_error(op_name, v, NULL, false))
			}
		},
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Consumes a list/object/string box and a value/char and 
// pushes a boolean based on whether or not that value/value/char
// is in that list/object/string box.
pub fn list_contains(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "contains";
	match s.pop2(){
		(Some(Value::ListBox(bn)), Some(v)) => {
			let lsbx = Value::ListBox(bn);
			if let Some(HeapValue::List(ls)) = h.get_heap_ref(lsbx){
				s.push(Value::Boolean(ls.contains(&v)));
				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
			let obx = Value::ObjectBox(a);
			let sbx = Value::StringBox(b);
			match (h.get_heap_ref(obx), h.get_heap_ref(sbx)){
				(Some(HeapValue::Object(obj)), Some(HeapValue::String(st))) => {
					s.push(Value::Boolean(obj.contains_key(st)));
					Ok(RetCode::Normal)	
				},
				(Some(_), None) => {
					Err(bad_box_error(op_name, sbx, NULL, false))
				},
				(None, Some(_)) => {
					Err(bad_box_error(op_name, obx, NULL, false))
				},
				(None, None) => {
					Err(bad_box_error(op_name, obx, sbx, true))
				},
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("list_contains"))
			}
		},
		(Some(Value::StringBox(bn)), Some(Value::Char(c))) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(st)) = h.get_heap_ref(sbx){
				s.push(Value::Boolean(st.contains(c)));
				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		(Some(a), Some(b)) => {
			Err(format!("Operator (contains) error! Second to top \
				of stack must be type ListBox/ObjectBox/StringBox and top \
				of stack must be Value/StringBox/Char respectably! \
				Attempted values: {} and {}", a, b))
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func("list_contains")),
	}
}

fn item_change_type_error(a: Value, b: Value, c: Value) -> String{
	format!("Operator (changeItemAt) error! \
		Third to top of stack must be type ListBox, \
		second to top of stack must be type usize, \
		and top of stack must by type Value! Attempted values: {}, {}, and {}", a, b, c)
}

//Alters an item in a list at a particular index to something else.
pub fn change_item_at(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "changeItemAt";
	match s.pop3(){
		(Some(Value::ListBox(bn)), Some(Value::UIntSize(i)), Some(v)) => {
			let lsbx = Value::ListBox(bn);
			//Changes item in list to new value at 
			// index i assuming list is valid and index is in range.
			if let Some(HeapValue::List(ls)) = h.get_heap_ref_mut(lsbx){
				if i < ls.len(){
					ls[i] = v;
					s.push(lsbx);
					Ok(RetCode::Normal)
				}else{
					Err(format!("Operator (changeItemAt) error! \
						Index {} is out of range of List of size {}", i, ls.len()))
				}
			}else{
				Err(bad_box_error(op_name, lsbx, NULL, false))
			}
		},
		(Some(a), Some(b), Some(c)) => {
			Err(item_change_type_error(a, b, c))
		},
		(None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Three", "only two")),
		(None, None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Three", "only one")),
		(None, None, None) => Err(needs_n_args_only_n_provided(op_name, "Three", "none")),
		_ => Err(should_never_get_here_for_func("change_item_at")),
	}
}

//Creates an error string for the three char operators below.
pub fn non_char_error(op_type: &str, v: Value) -> String{
	format!("Operator ({}) error! Top of stack must \
		be of type Char! Attempted value: {}", op_type, v)
}

//Conumes a character and pushes a boolean saying whether or not it's whitespace.
pub fn whitespace_detect(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop(){
		Some(Value::Char(c)) => {
			s.push(Value::Boolean(c.is_whitespace()));
			Ok(RetCode::Normal)
		},
		Some(v) => {
			Err(non_char_error("isWhitespaceChar", v))
		},
		None => Err(needs_n_args_only_n_provided("isWhitespaceChar", "One", "none")),
	}
}

//Determines if top of stack is an alphabetical char.
pub fn alpha_char_detect(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop(){
		Some(Value::Char(c)) => {
			s.push(Value::Boolean(c.is_alphabetic()));
			Ok(RetCode::Normal)
		},
		Some(v) => {
			Err(non_char_error("isAlphaChar", v))
		},
		None => Err(needs_n_args_only_n_provided("isAlphaChar", "One", "none")),
	}
}

//Determines if top of stack is a numeric char.
pub fn num_char_detect(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	match s.pop(){
		Some(Value::Char(c)) => {
			s.push(Value::Boolean(c.is_numeric()));
			Ok(RetCode::Normal)
		},
		Some(v) => {
			Err(non_char_error("isNumChar", v))
		},
		None => Err(needs_n_args_only_n_provided("isNumChar", "One", "none")),
	}
}

pub fn invalid_types_for_obj_add_or_mut(op_type: &str, v1: Value, v2: Value, v3: Value) -> String{
	format!("Operator ({}) error! Third to top of stack must be of type ObjectBox, \
		second to top must be type StringBox, and top must be type Value! \
		Attempted values: {}, {}, and {}", op_type, v1, v2, v3)
}

//Adds a field to the given object and pushes the mutated object back.
pub fn add_field(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "objAddField";
	match s.pop3(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b)), Some(v)) => {
			let objbx = Value::ObjectBox(a);
			let sbx = Value::StringBox(b);
			match (h.get_heap_ref_mut_and_ref(objbx, sbx)){
				(Some(HeapValue::Object(obj)), Some(HeapValue::String(st))) => {
					if !obj.contains_key(st){
						obj.insert(st.clone(), v);
						s.push(objbx);
						Ok(RetCode::Normal)
					}else{
						Err(format!("Operator (objAddField) error! \
							ObjectBox {} already contains field \"{}\"! \
							Try removing it first!", a, st))
					}
				},
				(Some(_), None) => Err(bad_box_error(op_name, sbx, NULL, false)),
				(None, Some(_)) => Err(bad_box_error(op_name, objbx, NULL, false)),
				(None, None) => Err(bad_box_error(op_name, objbx, sbx, true)),
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("add_field")),
			}
		},
		(Some(a), Some(b), Some(c)) => {
			Err(invalid_types_for_obj_add_or_mut(op_name, a, b, c))
		},
		(None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Three", "only two")),
		(None, None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Three", "only one")),
		(None, None, None) => Err(needs_n_args_only_n_provided(op_name, "Three", "none")),
		_ => Err(should_never_get_here_for_func("add_field")),
	}
}

pub fn invalid_types_for_get_or_rem(op_type: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Second to top of stack must \
		be type ObjectBox and top must be type StringBox! \
		Attempted values: {} and {}", op_type, v1, v2)
}

pub fn field_not_in_obj_err(op_type: &str, field_name: &str, err_box: Value) -> String{
	format!("Operator ({}) error! Field \"{}\" doesn't exist \
		in {} ! Try adding it!", op_type, field_name, err_box) 
}

//Given an object and string box, conumes the boxes 
// and pushes the value at that key if it exists.
pub fn get_field(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "objGetField";
	match s.pop2(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
			let bx1 = Value::ObjectBox(a);
			let bx2 = Value::StringBox(b);
			match (h.get_heap_ref(bx1), h.get_heap_ref(bx2)){
				(Some(HeapValue::Object(obj)), Some(HeapValue::String(st))) => {
					if let Some(v) = obj.get(st){
						s.push(*v);
						Ok(RetCode::Normal)
					}else{
						Err(field_not_in_obj_err(op_name, st, bx1))	
					}
				},
				(Some(_), None) => Err(bad_box_error(op_name, bx2, NULL, false)),
				(None, Some(_)) => Err(bad_box_error(op_name, bx1, NULL, false)),
				(None, None) => Err(bad_box_error(op_name, bx1, bx2, true)),
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("get_field"))
			}
		},
		(Some(a), Some(b)) => {
			Err(invalid_types_for_get_or_rem(op_name, a, b))
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func("get_field")),
	}
}

pub fn invalid_mutation_error(op_name: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Invalid mutation of {} to {} !", op_name, v1, v2)
}

//Mutates the field to a new value in an object if it exists and it's a valid mutation.
pub fn mut_field(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "objMutField";
	match s.pop3(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b)), Some(v)) => {
			let obx = Value::ObjectBox(a);
			let sbx = Value::StringBox(b);
			match (h.get_heap_ref_mut_and_ref(obx, sbx)){
				(Some(HeapValue::Object(obj)), Some(HeapValue::String(st))) => {
					if let Some(old_val) = obj.get_mut(st){
						if is_valid_mutation(*old_val, v){
							*old_val = v;
							s.push(obx);
							Ok(RetCode::Normal)
						}else{
							Err(invalid_mutation_error(op_name, *old_val, v))
						}
					}else{
						Err(field_not_in_obj_err(op_name, st, obx))
					}
				},
				(Some(_), None) => Err(bad_box_error(op_name, sbx, NULL, false)),
				(None, Some(_)) => Err(bad_box_error(op_name, obx, NULL, false)),
				(None, None) => Err(bad_box_error(op_name, obx, sbx, true)),
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("mutate_field"))
			}
		},
		(Some(a), Some(b), Some(c)) => {
			Err(invalid_types_for_obj_add_or_mut(op_name, a, b, c))
		},
		(None, Some(_), Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Three", "only two")),
		(None, None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Three", "only one")),
		(None, None, None) => Err(needs_n_args_only_n_provided(op_name, "Three", "none")),
		_ => Err(should_never_get_here_for_func("mut_field")),
	}
}

//Removes a field from an object at the desired key held in the string box.
pub fn remove_field(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "objRemField";
	match s.pop2(){
		(Some(Value::ObjectBox(a)), Some(Value::StringBox(b))) => {
			let obx = Value::ObjectBox(a); 
			let sbx = Value::StringBox(b);
			match (h.get_heap_ref_mut_and_ref(obx, sbx)){
				(Some(HeapValue::Object(obj)), Some(HeapValue::String(st))) => {
					match obj.remove(st){
						Some(_) => {s.push(obx); Ok(RetCode::Normal)},
						None => {Err(field_not_in_obj_err(op_name, st, obx))}
					}
				},
				(Some(_), None) => Err(bad_box_error(op_name, sbx, NULL, false)),
				(None, Some(_)) => Err(bad_box_error(op_name, obx, NULL, false)),
				(None, None) => Err(bad_box_error(op_name, obx, sbx, true)),
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("remove_field"))
			}
		},
		(Some(a), Some(b)) => {
			Err(invalid_types_for_get_or_rem(op_name, a, b))
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func("remove_field")),
	}
}

//Acts like C's strcmp, eating two string boxes and pushing 
// an integer to indicate the result of the comparison between their contents.
// If the second to top is less than the top, a negative one is pushed
// If the second to top is equal to the top, a zero is pushed
// If the second to top is greater than the top, a one is pushed
pub fn string_compare(s: &mut Stack, h: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "stringCompare";
	match s.pop2(){
		(Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
			let sbx1 = Value::StringBox(a);
			let sbx2 = Value::StringBox(b);
			match (h.get_heap_ref(sbx1), h.get_heap_ref(sbx2)){
				(Some(HeapValue::String(s1)), Some(HeapValue::String(s2))) => {
					let comp_res: isize = match s1.cmp(s2){
						Ordering::Less => -1,
						Ordering::Equal => 0,
						Ordering::Greater => 1,
					};
					s.push(Value::IntSize(comp_res));
					Ok(RetCode::Normal)
				},
				(Some(_), None) => Err(bad_box_error(op_name, sbx2, NULL, false)),
				(None, Some(_)) => Err(bad_box_error(op_name, sbx1, NULL, false)),
				(None, None) => Err(bad_box_error(op_name, sbx1, sbx2, true)),
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("string_compare"))
			}
		},
		(Some(a), Some(b)) => {
			Err(format!("Operator ({}) error! String comparison \
				requires two items of type StringBox on the stack! \
				Attempted values: {} and {}", op_name, a, b))
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func("string_compare")),
	}
}

//Macro for bitwise operations between two operators.
macro_rules! binary_bitwise_op{
	($x:expr, $y:expr, $op:tt, $name:expr, $($type:ident),* $(,)?) => {
		match ($x, $y){
			$((Value::$type(x), Value::$type(y)) => Ok(Value::$type(x $op y)), )*
			(x, y) => Err(numerical_type_error_string($name, x, y)),
		}	
	};
}

//Performs bitwise OR between two integers.
pub fn bit_or(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "bitOr/|";
	let res: Result<Value, String> = match s.pop2(){
		(Some(a), Some(b)) => {
			binary_bitwise_op!(
				a, b, |, op_name, 
				IntSize, UIntSize, 
				Int8, Int16, Int32, Int64, Int128,
				UInt8, UInt16, UInt32, UInt64, UInt128 
			) 	
		},
		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("bit_or")),
	};

	push_val_or_err(res, s)
}

//Performs bitwise AND between two matching integer types.
pub fn bit_and(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "bitAnd/&";
	let res: Result<Value, String> = match s.pop2(){
		(Some(a), Some(b)) => {
			binary_bitwise_op!(
				a, b, &, op_name, 
				IntSize, UIntSize, 
				Int8, Int16, Int32, Int64, Int128,
				UInt8, UInt16, UInt32, UInt64, UInt128 
			) 	
		},
		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("bit_or")),
	};

	push_val_or_err(res, s)
}

//Performs bitwise XOR between two matching integer types.
pub fn bit_xor(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let op_name = "bitXor/^";
	let res: Result<Value, String> = match s.pop2(){
		(Some(a), Some(b)) => {
			binary_bitwise_op!(
				a, b, ^, op_name, 
				IntSize, UIntSize, 
				Int8, Int16, Int32, Int64, Int128,
				UInt8, UInt16, UInt32, UInt64, UInt128 
			) 	
		},
		(None, Some(_)) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "only one"))
		},

		(None, None) => {
			Err(needs_n_args_only_n_provided(op_name, "Two", "none"))
		},

		_ => Err(should_never_get_here_for_func("bit_or")),
	};

	push_val_or_err(res, s)
}

//Performs a bitwise not on an integer on the stack.
pub fn bit_not(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
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
pub fn shift<T>(n: T, shift_n: isize) -> T 
where T: Shl<usize, Output = T> + Shr<usize, Output = T> + Default
{
	let t_bit_count = std::mem::size_of::<T>() * 8;
	let shift_n_abs = shift_n.unsigned_abs() as usize;
	
	if shift_n_abs >= t_bit_count{
		T::default()
	}else{
		match shift_n.cmp(&0){
			Ordering::Greater => n << shift_n_abs,
			Ordering::Equal => n,
			Ordering::Less => n >> shift_n_abs
		}
	}
}

//Macro for bitwise operations between two operators.
macro_rules! shift_macro{
	($x:expr, $y:expr, $($type:ident),* $(,)?) => {
		match ($x, $y){
			$((Value::$type(x), Value::IntSize(y)) => Ok(Value::$type(shift(x, y))), )*
			(x, y) => Err(format!("Operator (bitShift) error! Second to top must \
				be numeric integer type and top must be type isize! \
				Attempted values: {} and {}", x, y)),
		}	
	};
}

//Performs a left or right bitshift by n bits on an integer.
pub fn bit_shift(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	let res = match s.pop2(){
		(Some(a), Some(b)) => {
			shift_macro!(
				a, b,  
				IntSize, UIntSize, 
				Int8, Int16, Int32, Int64, Int128,
				UInt8, UInt16, UInt32, UInt64, UInt128 
			) 	
		},

		(None, Some(_)) => Err(needs_n_args_only_n_provided("bitShift", "Two", "only one")),

		(None, None) => Err(needs_n_args_only_n_provided("bitShift", "Two", "none")),

		_ => Err(should_never_get_here_for_func("bit_shift")),
	};

	push_val_or_err(res, s)

}

//Pushes maximum value for isize datatype to stack.
pub fn max_isize(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::IntSize(isize::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for usize datatype to stack.
pub fn max_usize(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::UIntSize(usize::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for i8 datatype to stack.
pub fn max_i8(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::Int8(i8::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for i16 datatype to stack.
pub fn max_i16(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::Int16(i16::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for i32 datatype to stack.
pub fn max_i32(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::Int32(i32::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for i64 datatype to stack.
pub fn max_i64(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::Int64(i64::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for i128 datatype to stack.
pub fn max_i128(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::Int128(i128::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for u8 datatype to stack.
pub fn max_u8(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::UInt8(u8::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for u16 datatype to stack.
pub fn max_u16(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::UInt16(u16::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for u32 datatype to stack.
pub fn max_u32(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::UInt32(u32::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for u64 datatype to stack.
pub fn max_u64(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::UInt64(u64::MAX));
	Ok(RetCode::Normal)
}

//Pushes maximum value for u128 datatype to stack.
pub fn max_u128(s: &mut Stack, _: &mut Heap, _: Option<&str> ) -> Result<RetCode, String>{
	s.push(Value::UInt128(u128::MAX));
	Ok(RetCode::Normal)
}

fn invalid_cast_type_error(name: &str, attempt: &str) -> String{
	format!("Operator ({}) error! Input type {} is not a valid casting type!", 
		name, attempt)
}

fn invalid_cast_operation_error(name: &str, v: Value, att: &str) -> String{
	format!("Operator ({}) error! Casting {} to type {} is not a valid cast!", 
		name, v, att)
}

fn casting_failed_error(name: &str, v: Value, att: &str) -> String{
	format!("Operator ({}) error! Failed to cast {} to type {}!", 
		name, v, att)
}

fn invalid_box_for_casting_error(name: &str, v: Value, att: &str) -> String{
	format!("Operator ({}) error! Failed to cast {} to type {}! It is an invalid Box!",
		name, v, att)
}

//Performs all valid casts in existence wherein the top 
// of the stack tries to be casted to another data type.
pub fn cast_stuff(s: &mut Stack, h: &mut Heap, c_type: Option<&str>) -> Result<RetCode, String>{
	//Used to make match statements less bloated for conversions.
	macro_rules! cast_match{
		($target:ident, $op_name:ident, $cast_type:ident, $needed:ident, $provided:ident, [$($type:ident),* $(,)?], [$($h_type:ident, $b_type:ident),* $(,)?]) => {
			match s.pop(){
				$(Some(Value::$type(v)) => {
					match SuperValue::try_cast(v, $target){
						Ok(SuperValue::Heap(hval)) => {
							s.push(h.insert_to_heap(*hval));
							Ok(RetCode::Normal)
						},
						Ok(SuperValue::Reg(val)) => {s.push(val); Ok(RetCode::Normal)},
						Err(e) => Err(casting_failed_error(
							$op_name, Value::$type(v), $cast_type))
					}
				},)*
				$(Some(Value::$b_type(bn)) => {
					let bx = Value::$b_type(bn);
					if let Some(HeapValue::$h_type(item)) = h.get_heap_ref(bx){
						match SuperValue::try_cast(item, $target){
							Ok(SuperValue::Heap(hval)) => {
								s.push(h.insert_to_heap(*hval));
								Ok(RetCode::Normal)
							},
							Ok(SuperValue::Reg(val)) => {s.push(val); Ok(RetCode::Normal)}
							Err(_) => Err(casting_failed_error(
								$op_name, bx, $cast_type)),
						}
					}else{
						Err(invalid_box_for_casting_error($op_name, bx, $cast_type)) 
					}
				},)*
				Some(v) => Err(invalid_cast_operation_error($op_name, v, $cast_type)),
				None => Err(needs_n_args_only_n_provided($op_name, $needed, $provided))
			}	
		};
	}
	if let Some(cast_type) = c_type{
		let op_name = "castTo";
		let needed = "One";	
		let provided = "none";
		if let Ok(cast_target) = CastType::try_cast(cast_type, CastType::MiscBox){
			cast_match!{
				cast_target, op_name, cast_type, needed, provided,
				[
					UIntSize, UInt8,	
					UInt16,	UInt32,	
					UInt64,	UInt128,	

					IntSize, Int8,	
					Int16,	Int32,	
					Int64,	Int128,	

					Float32, Float64,

					Char, Boolean
				],
			
				[
					String,	StringBox,
					List, ListBox,
					Object, ObjectBox
				]
			}

		}else{
			Err(invalid_cast_type_error(op_name, cast_type))
		}
	}else{
		let op_name = "cast";
		let needed = "Two";	
		let provided = "only one/none";
		match s.pop(){
			Some(Value::StringBox(n)) => {
				let sbx = Value::StringBox(n);
				match h.get_heap_ref(sbx){
					Some(HeapValue::String(cast_type)) => {
						if let Ok(cast_target) = 
						CastType::try_cast(cast_type, CastType::MiscBox)
						{
							cast_match!{
								cast_target, op_name, cast_type, needed, provided,
								[
									UIntSize, UInt8,	
									UInt16,	UInt32,	
									UInt64,	UInt128,	

									IntSize, Int8,	
									Int16,	Int32,	
									Int64,	Int128,	

									Float32, Float64,

									Char, Boolean
								],
							
								[
									String,	StringBox,
									List, ListBox,
									Object, ObjectBox
								]
							}
						}else{
							Err(invalid_cast_type_error(op_name, cast_type))
						}
					},
					Some(_) => Err(should_never_get_here_for_func("cast_stuff")),
					None => Err(bad_box_error(op_name, sbx, NULL, false)) 
				}

			},
			Some(v) => Err(format!("Operator (cast) error! Top of stack must be a StringBox! Attempted: {}", v)),
			None => Err(needs_n_args_only_n_provided(op_name, needed, provided)),
		}	
	}
}

//Creates an error string that indicates a wrong 
// given type for the various print io functions.
pub fn io_needing_one_item_on_stack_error(op_type: &str, needed_type: &str, attempted_value: Value) -> String{
	format!("Operator ({}) error! Top of stack must be type {}! \
		Attempted value: {}", op_type, needed_type, attempted_value)
}

//Prints contents of a string box and consumes it. 
// Like everything else, the stringbox is not free'd.
pub fn print_line(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "printLine";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(print_str)) = h.get_heap_ref(sbx) {
				println!("{}", print_str); Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false)) 
			}		
		},
		Some(v) => Err(io_needing_one_item_on_stack_error(op_name, "StringBox", v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

pub fn unable_to_read_error(op_type: &str, reason: &str) -> String{
	format!("Operator ({}) error! Unable to read from stdin because: {}", op_type, reason)
}

//Reads a line from stdin and allocates it as 
// a string on the heap, pushing a stringbox to the stack.
pub fn read_line_from_in(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let mut input = String::new();

	match io::stdin().read_line(&mut input){
		Ok(_) => {
			if input.ends_with("\n") {input.pop();}
			input = replace_literals_with_escapes(&input);
			s.push(h.insert_to_heap(HeapValue::String(input)));
			Ok(RetCode::Normal)
		},
		Err(e) => Err(unable_to_read_error("readLine", &e.to_string()))
	}
}

//Prints out a single char to stdout. 
// Top of stack must be a char.
pub fn print_char(s: &mut Stack, _: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "printChar";
	match s.pop(){
		Some(Value::Char(c)) => {
			print!("{}", c); Ok(RetCode::Normal)
		},
		Some(v) => Err(io_needing_one_item_on_stack_error(op_name, "Char", v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Reads in one Char from stdin and pushes it to the stack.
pub fn read_char(s: &mut Stack, _: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
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
				s.push(Value::Char(c));
				return Ok(RetCode::Normal);
			}
		}

		//Errors out if buffer is filled up with no valid Char.
		if buff_collection_length > 4 {
			return Err(format!("readChar buffer overflow!!!"));
		}	

	}

}

//Prints contents of a string box and consumes it. 
// Like everything else, the stringbox is not free'd.
//Unlike printline, this operator doesn't append a newline character.
pub fn print_string(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "print";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(print_str)) = h.get_heap_ref(sbx){
				print!("{}", print_str);
				//Buffer flush to make it print without queueing.
				match io::stdout().flush(){
					Ok(_) => Ok(RetCode::Normal),
					Err(e) => Err(format!("Operator ({}) error! Failed \
						to flush buffer after printing because: {}", 
						op_name, &e.to_string())),
				}
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false)) 
			}
		},
		Some(v) => Err(io_needing_one_item_on_stack_error(op_name, "StringBox", v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Reads the contents of stdin into a string. Basically like readline \
// but doesn't stop reading until stdin is manually closed.
pub fn read_from_in(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
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
			let new_string = HeapValue::String(replace_literals_with_escapes(&st));
			s.push(h.insert_to_heap(new_string));
			Ok(RetCode::Normal)
		},
		Err(e) => {
			Err(format!("Operator (read) error! Unable to \
				convert input to a proper String because: {}", e))
		},
	}

}

//Prints each item on the stack while 
// also indicating if box types are valid or not.
pub fn debug_stack_print(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let filler_str = "--------------------------------";
	let valid_or_invalid = ["", "[INVALID]"];

	println!("{}", filler_str);
	println!("BEGIN STACK PRINT\n{}", filler_str);
	for item in s.read_stack(){
		let val_idx = (h.is_box(*item) && !h.validate_box(*item)) as usize;
		println!("{} {}", item, valid_or_invalid[val_idx]);
	}

	println!("{}", filler_str);
	println!("STACK LENGTH: {}", s.size());
	println!("{}\nEND STACK PRINT", filler_str); 
	println!("{}", filler_str);

	Ok(RetCode::Normal)
}

//Prints the whole heap to stdout for debugging purposes.
//This is something like O(n^2) at least so definitely only use it for debugging!
pub fn debug_heap_print(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let filler_str = "////////////////////////////////";
	let heap_size = h.read_heap().len();
	let free_list_size = h.read_free_list().len();	
	let free_or_no = ["[FREE]", ""];


	println!("{}", filler_str);
	println!("BEGIN HEAP PRINT\n{}", filler_str);
	
	//Iterates through and prints each item on heap for debugging.
	let heap = h.read_heap();
	for bn in 0..heap_size{
		let box_type_str: &str = match heap[bn].0{
			HeapValue::String(_) => "StringBox",
			HeapValue::List(_) => "ListBox",
			HeapValue::Object(_) => "ObjectBox",
			HeapValue::Primitive(_) => "MiscBox",
		};

		let is_valid = h.cell_number_is_valid(bn);
		let cell_ref = &heap[bn].0;
		println!("{} {} {}:\n\t{}", box_type_str, bn, 
			free_or_no[is_valid as usize], cell_ref);
	}

	println!("{}", filler_str);
	print!("FREE'D BOX NUMBERS: [");
	let free_list = h.read_free_list();
	let comma_or_no = ["", ", "];
	for i in 0..free_list_size{
		print!("{}{}", free_list[i], 
			comma_or_no[(i < (free_list_size - 1)) as usize]);
	}
	println!("]\n{}", filler_str);
	println!("FREE'D BOX COUNT: {}\n{}", free_list_size, filler_str);
	println!("TOTAL HEAP ITEM COUNT: {}\n{}", heap_size, filler_str);
	println!("PERCENT OF HEAP FREE'D: {:.2}\n{}", 
		(free_list_size as f32) / (heap_size as f32) * 100f32, filler_str);
	println!("END HEAP PRINT\n{}", filler_str);

	Ok(RetCode::Normal)
}

fn file_open_error(op_name: &str, name: &str, reason: &str) -> String{
	format!("Operator ({}) error! \
		Unable to open file name {} \
		because: {}", 
		op_name, name, reason)
}

//Writes the data of one stringbox to a file with the name held in the other string box. 
// Creates a file if one doesn't exist. 
pub fn write_data_to_file(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "fileWrite";
	match s.pop2(){
		(Some(Value::StringBox(a)), Some(Value::StringBox(b))) => {
			let sbx1 = Value::StringBox(a);
			let sbx2 = Value::StringBox(b);
			match (h.get_heap_ref(sbx1), h.get_heap_ref(sbx2)){
				(Some(HeapValue::String(file_name)), Some(HeapValue::String(string_to_write))) => {

					let file_path = Path::new(file_name);
					let mut file = match OpenOptions::new().write(true).truncate(true).open(file_path){
						Ok(f) => f,
						Err(reason) => {return Err(file_open_error(
							op_name, file_name, reason.to_string().as_str()));
						},
					};

					match file.write_all(string_to_write.as_bytes()){
						Ok(_) => Ok(RetCode::Normal),
						Err(reason) => {
							Err(format!("Operator ({}) error! \
								Unable to write to {} because: {}", 	
								op_name, file_name, reason))
						},
					}
				},
				(Some(_), None) => Err(bad_box_error(op_name, sbx2, NULL, false)),
				(None, Some(_)) => Err(bad_box_error(op_name, sbx1, NULL, false)),
				(None, None) => Err(bad_box_error(op_name, sbx1, sbx2, true)),
				(Some(_), Some(_)) => Err(should_never_get_here_for_func("write_data_to_file")),
			}
		},
		(Some(a), Some(b)) => {
			Err(format!("Operator ({}) error! Second to top \
				and top of stack must both be of type StringBox! \
				Attempted values: {} and {}", op_name, a, b))
		},
		(None, Some(_)) => Err(needs_n_args_only_n_provided(op_name, "Two", "only one")),
		(None, None) => Err(needs_n_args_only_n_provided(op_name, "Two", "none")),
		_ => Err(should_never_get_here_for_func("write_data_to_file")),
	}
}

pub fn single_arg_file_io_type_error(op_type: &str, v: Value) -> String{
	format!("Operator ({}) error! Top of stack must \
	 be type StringBox! Attempted value: {}", op_type, v)
}

//Reads the contents of a file into a string and allocates it on the heap.
pub fn read_data_from_file(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "fileRead";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(file_name)) = h.get_heap_ref(sbx){
				let file_path = Path::new(file_name);
				let mut file = match OpenOptions::new().read(true).open(file_path){
					Ok(f) => f,
					Err(reason) => {
						return Err(file_open_error(
							op_name, file_name, reason.to_string().as_str())); 
					},
				};

				let mut literal_file_string = String::new();
				match file.read_to_string(&mut literal_file_string){
					Ok(_) => (),
					Err(reason) => {
						return Err(format!("Operator ({}) error! Failed \
							to read from file {} because: {}", 
							op_name, file_name, reason));
					},
				}
				
				let file_string = replace_literals_with_escapes(&literal_file_string);
				let new_box = h.insert_to_heap(HeapValue::String(file_string));

				s.push(new_box);
				Ok(RetCode::Normal)

			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => Err(single_arg_file_io_type_error(op_name, v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Creates a file with the desired name. Throws error if the file already exists.
pub fn create_file_based_on_string(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "fileCreate";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(file_name)) = h.get_heap_ref(sbx){
				let file_path = Path::new(file_name);

				//Throws error if file with given name already exists.
				match File::open(file_path){
					Ok(_) => {
						return Err(format!("Operator ({}) error! Unable \
							to create file {} because it already exists!", 
							op_name, file_name));
					},
					Err(_) => (),
				}

				match File::create(file_path){
					Ok(_) => {},
					Err(reason) => {
						return Err(format!("Operator ({}) error! Unable \
							to create file {} because: {}", 
						 	op_name, file_name, reason.to_string()));
					},
				}

				Ok(RetCode::Normal)

			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => Err(single_arg_file_io_type_error(op_name , v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Deletes a file with the input name.
pub fn delete_file_based_on_string(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "fileRemove";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(file_name)) = h.get_heap_ref(sbx){
				let file_path = Path::new(file_name);

				match remove_file(file_path){
					Ok(_) => {},
					Err(reason) => {
						return Err(format!("Operator ({}) error! Unable \
							to remove file {} because: {}", 
							op_name, file_name, reason.to_string()));
					},
				}

				Ok(RetCode::Normal)

			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => Err(single_arg_file_io_type_error(op_name, v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Pushes a boolean based on whether or not the file exists.
pub fn file_exists(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "fileExists";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(file_name)) = h.get_heap_ref(sbx){
				let file_path = Path::new(file_name);

				let exists = match File::open(file_path){
					Ok(_) => true,
					Err(_) => false,
				};

				s.push(Value::Boolean(exists));

				Ok(RetCode::Normal)
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => Err(single_arg_file_io_type_error(op_name, v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Consumes a value and pushes a stringbox whose contents 
// is a string that represents the type of the consumed value.
pub fn query_type(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	match s.pop(){
		Some(v) => {
			let type_str = type_to_string(v);
			let sbx = h.insert_to_heap(HeapValue::String(type_str));
			s.push(sbx);
			Ok(RetCode::Normal)
		},
		None => Err(needs_n_args_only_n_provided("queryType", "One", "none")),
	}
}

//If the top of the stack is a true boolean, the program leaves the current scope.
// This is useful for early function returns and breaking out of loops. 
pub fn leave_scope_if_true(s: &mut Stack, _: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "leaveScopeIfTrue";
	match s.pop(){
		Some(Value::Boolean(b)) => {
			if b {Ok(RetCode::LeavingScopeEarly)} 
			else {Ok(RetCode::Normal)}
		},
		Some(v) => {
			Err(format!("Operator ({}) error! Top of stack \
				must be of type Boolean! Attempted value: {}", op_name, v))
		},
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Throws an error containing a string held by a stringbox at the top of the stack.
pub fn throw_custom_error(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let op_name = "throwCustomError";
	match s.pop(){
		Some(Value::StringBox(bn)) => {
			let sbx = Value::StringBox(bn);
			if let Some(HeapValue::String(err_str)) = h.get_heap_ref(sbx){
				Err(err_str.clone())
			}else{
				Err(bad_box_error(op_name, sbx, NULL, false))
			}
		},
		Some(v) => Err(format!("Operator ({}) error! Top of stack \
				must be of type StringBox! Attempted value: {}", op_name, v)),
		None => Err(needs_n_args_only_n_provided(op_name, "One", "none")),
	}
}

//Fetches arguments passed to program and converts them into a list 
// of stringboxes where each stringbox contains an argument string.
//This basically is like argv in C.
pub fn get_args(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	//Fetches args from environment.
	let all_args: Vec<String> = env::args().collect();
	
	//Creates a list which holds string boxes 
	// pointing to all the given arguments.
	let args: Vec<Value> = all_args[1..]
		.iter()
		.map(|st| h.insert_to_heap(HeapValue::String(st.clone())))
		.collect();

	//Inserts list of arg stringboxes into heap and pushes listbox.
	s.push(h.insert_to_heap(HeapValue::List(args)));

	Ok(RetCode::Normal)
}

fn invalid_type_for_valid_box_check(v: Value) -> String{
	format!("Operator (isValidBox) error! \
		Top of stack must be of type StringBox, ListBox, \
		ObjectBox, MiscBox, or NULLBox! Attempted value: {}", v)         
}

//Consumes top of stack and checks if it's a valid box.
pub fn is_valid_box(s: &mut Stack, h: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	let res = match s.pop(){
		Some(v) => {
			match (h.is_box(v), h.validate_box(v)){
				(true, true) => Ok(Value::Boolean(true)),
				(true, false) => Ok(Value::Boolean(false)),
				(false, _) => Err(invalid_type_for_valid_box_check(v)),
				_ => Err(should_never_get_here_for_func("is_valid_box"))
			}
			
		},
		None => Err(needs_n_args_only_n_provided("isValidBox", "One", "none")),
	};
	push_val_or_err(res, s)
}

//Gets the current unix time as a 64 bit bload 
// and pushes it to the stack as such.
pub fn time_unix_now(s: &mut Stack, _: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	match SystemTime::now().duration_since(UNIX_EPOCH){
		Ok(time) => {
			let secs = time.as_secs();
			let nanos = time.subsec_nanos();

			let time_float: f64 = (secs as f64) + ((nanos as f64) / 1e9f64);
			s.push(Value::Float64(time_float));
			Ok(RetCode::Normal)
		},
		Err(e) => Err(format!("Operator (timeUnixNow) error! Unable to \
			fetch the current Unix time because {}", e)),
	}
}

fn invalid_stack_top_for_time_wait(v: Value) -> String{
	format!("Operator (timeWait) error! Top of stack must be type f32 or f64! Attempted value: {}", v)
}

//Causes the program to pause for a specified number of seconds.
//Accepts either f64 or f32.
pub fn time_wait(s: &mut Stack, _: &mut Heap, _: Option<&str>) -> Result<RetCode, String>{
	macro_rules! wait_match{
		($($type:ty, $var:ident),* $(,)?) => {
			match s.pop(){
				$(Some(Value::$var(t)) => {
					let seconds = t as u64;
					let nanos: u32 = ((t - (seconds as $type)) * (1000000000 as $type)) as u32;
					let sleep_durr = time::Duration::new(seconds, nanos);
					thread::sleep(sleep_durr);
					Ok(RetCode::Normal)
				}, )*
				Some(v) => Err(invalid_stack_top_for_time_wait(v)),
				None => Err(needs_n_args_only_n_provided("timeWait", "One", "none")),
			}	
		};
	}
	wait_match!{f64, Float64, f32, Float32}
}

