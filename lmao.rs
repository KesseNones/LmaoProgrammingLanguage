//Jesse A. Jones
//Lmao Programming Language, the Spiritual Successor to EcksDee
//Version: 0.18.0

use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write; 
use std::io;
use std::rc::Rc;
mod parser;
use parser::*;
mod state_and_ops;
use state_and_ops::*;

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

fn variable_already_exists_error(op_name: &str, name: &str) -> String{
	format!("Variable creation ({}) error! Variable {} already exists! Try deleting it using del!", op_name, name)
}

fn variable_nonexist_error(op: &str, name: &str) -> String{
	format!("Variable ({}) error! Variable {} doesn't exist. Try making it first!", op, name)
}

fn box_free_error(v: Value) -> String{
	format!("Box free error! Unable to free {} because it's not a valid box type to free!", v)
}

fn box_alter_type_error(v1: Value, v2: Value) -> String{
	format!("Box altr error! Second to top of stack must be type MiscBox and top of stack type Value! Attempted values: {} and {}", v1, v2)
}

fn invalid_mutation_error(op_name: &str, v1: Value, v2: Value) -> String{
	format!("Operator ({}) error! Unable to mutate {} to {}, as it is an invalid mutation!", op_name, v1, v2)
}

fn run_operator(op: Operator, s: &mut Stack, h: &mut Heap, args: Option<&str>) -> 
Result<RetCode, String>
{
	macro_rules! op_match{
		($(($var:ident, $func:ident)),* $(,)?) => {
			match op{
				$(Operator::$var => $func(s, h, args),)*			
				//External check for unknown should cover this!
				Operator::Unknown => Err(should_never_get_here_for_func("run_operator")),
			}	
		};
	}
	op_match!{
		(Add, add), (Sub, sub), (Mul, mult), (Div, div), 
		(Mod, modulo), (Pow, power),

		(IsizeMax, max_isize), (UsizeMax, max_usize),

		(I8Max, max_i8), (I16Max, max_i16), (I32Max, max_i32), 
		(I64Max, max_i64), (I128Max, max_i128),

		(U8Max, max_u8), (U16Max, max_u16), (U32Max, max_u32), 
		(U64Max, max_u64), (U128Max, max_u128),

		(Swap, swap), (Drop, drop), (DropStack, drop_stack), 
		(Rot, rot), (Dup, dup), (DeepDup, deep_dup), 

		(Equal, is_equal), (NotEqual, is_not_equal), 
		(GreaterThan, is_greater_than), (LessThan, is_less_than), 
		(GreaterThanEqualTo, is_greater_than_equal_to), 
		(LessThanEqualTo, is_less_than_equal_to), 
		(StringCompare, string_compare), (Concat, concat),

		(And, and), (Or, or), (Xor, xor), (Not, not),

		(Push, list_push), (Pop, list_pop), (Fpush, list_front_push), 
		(Fpop, list_front_pop), (Index, index), (Length, length),

		(IsEmpty, is_empty), (Clear, list_clear), 
		(Contains, list_contains), (ChangeItemAt, change_item_at),

		(IsWhitespaceChar, whitespace_detect), 
		(IsAlphaChar, alpha_char_detect), 
		(IsNumChar, num_char_detect),

		(ObjAddField, add_field), (ObjGetField, get_field), 
		(ObjMutField, mut_field), (ObjRemField, remove_field),

		(BitOr, bit_or), (BitAnd, bit_and), 
		(BitXor, bit_xor), (BitNot, bit_not), 
		(BitShift, bit_shift), (Cast, cast_stuff),

		(PrintLine, print_line), (ReadLine, read_line_from_in), 
		(PrintChar, print_char), (ReadChar, read_char), 
		(Print, print_string), (Read, read_from_in), 
		(DebugPrintStack, debug_stack_print), 
		(DebugPrintHeap, debug_heap_print),

		(FileWrite, write_data_to_file), 
		(FileRead, read_data_from_file), 
		(FileCreate, create_file_based_on_string), 
		(FileRemove, delete_file_based_on_string), 
		(FileExists, file_exists),

		(QueryType, query_type), (LeaveScopeIfTrue, leave_scope_if_true), 
		(ThrowCustomError, throw_custom_error), (GetArgs, get_args), 
		(IsValidBox, is_valid_box), (TimeUnixNow, time_unix_now), 
		(TimeWait, time_wait),
	}
}

//Iterates recursively through the AST and effectively runs the program doing so.
fn run_program(ast: &ASTNode, s: &mut Stack, h: &mut Heap, 
vars: &mut Variables, fns: &mut Functions) -> Result<RetCode, String>
{
    let mut deferred: Option<Vec<Rc<ASTNode>>> = None;
	let mut res: Result<RetCode, String> = Ok(RetCode::Normal); 
	vars.add_frame();

	macro_rules! err_break{
		($err:expr) => {
			res = Err($err); break;	
		};
	}

    match ast{
        ASTNode::Expression(nodes) => {
            for node in nodes.iter(){
                match node{
                    ASTNode::Terminal(Token::V(v)) => {
                        match v{
							SuperValue::Heap(hval) => s.push(h.insert_to_heap(hval.clone())),
							SuperValue::Reg(val) => s.push(*val),
                        }
                    },
                    ASTNode::Terminal(Token::Word((op, op_val))) => {
						if let(Operator::Unknown) = op_val{
							err_break!{format!("Unrecognized Operator: {}", op)}
						}else{
							match run_operator(*op_val, s, h, None){
								Ok(RetCode::Normal) => (),
								Ok(RetCode::LeavingScopeEarly) => {
									res = Ok(RetCode::LeavingScopeEarly);
									break;
								},
								Err(e) => {err_break!{e}},
							}									
						}

                    },
                    ASTNode::Variable{var_name: name, cmd: c, var_num: num} => {
                        match c{
                            VarCmd::Make => {
								match s.pop(){
									Some(v) => {
										if !vars.mak_var(name, v){
											err_break!{variable_already_exists_error("var mak", name)}
										}
									},
									None => {
										err_break!{variable_lack_of_args_error("creation (mak)")}
									},
								}
                            },
                            VarCmd::Get => {
								if let Some(v) = vars.get_var(name){
									s.push(v);
								}else{
									err_break!{variable_nonexist_error("var get", name)}
								}
                            },
                            VarCmd::Mutate => {
								match s.pop(){
									Some(new_v) => {
										match vars.mut_var(name, new_v){
											0 => (),	
											1 => {
												err_break!{variable_nonexist_error("var mut", name)}
											},
											2 => {
												let old_v = vars.get_var(name).unwrap();
												err_break!{invalid_mutation_error("var mut", old_v, new_v)}	
											},
											_ => {err_break!{should_never_get_here_for_func("var mut")}}
										}
									},
									None => {
										err_break!{variable_lack_of_args_error("mutation (mut)")}
							
									}
								}
                            },
                            VarCmd::Delete => {
								if !vars.del_var(name){
									err_break!{variable_nonexist_error("var del", name)}
								}
                            },
                            VarCmd::Unknown => {
                                err_break!{"Variable (var) error! Unrecognized variable command! Valid: mak, get, mut, del .".to_string()}
                            },
                        }
                    },
                    ASTNode::BoxOp(box_op) => {
                        match box_op{
                            BoxCmd::Free => {
                                match s.pop(){
                                    Some(v) => {
										if !h.free_heap_cell(v){
											err_break!{box_free_error(v)}
										}
                                    },
                                    None => {
                                        err_break!{needs_n_args_only_n_provided("box free", "One", "none")}
                                    },
                                }
                            },
                            BoxCmd::Null => {
                                s.push(Value::NULLBox);
                            },
                            BoxCmd::Make => {
                                match s.pop(){
                                    Some(v) => {
										let prim = HeapValue::Primitive(v);
										s.push(h.insert_to_heap(prim));
                                    },
                                    None => {
										err_break!{needs_n_args_only_n_provided("box make", "One", "none")}
									}
                                }
                            },
                            BoxCmd::Open => {
                                match s.pop(){
                                    Some(Value::MiscBox(bn)) => {
										let mbx = Value::MiscBox(bn);
										if let Some(HeapValue::Primitive(v)) = 
										h.get_heap_ref(mbx)
										{
											s.push(*v);	
										}else{
											err_break!{bad_box_error("box open", mbx, Value::NULLBox, false)}
										}						
                                    },
                                    Some(v) => {
                                        err_break!{format!("Box open error!\
                                             Top of stack must be type MiscBox! \
                                            Attempted value: {}", v)}
                                    },
                                    None => {
										err_break!{needs_n_args_only_n_provided("box open", "One", "none")}
									}
                                }
                            },
                            BoxCmd::Alter => {
                                match s.pop2(){
                                    (Some(Value::MiscBox(bn)), Some(new_v)) => {
										let mbx = Value::MiscBox(bn);
										if let Some(HeapValue::Primitive(old_v)) = 
										h.get_heap_ref_mut(mbx)
										{
											if is_valid_mutation(*old_v, new_v){
												*old_v = new_v;
											}else{
												err_break!{invalid_mutation_error("box altr", *old_v, new_v)}
											}
										}else{
											err_break!{bad_box_error("box altr", mbx, Value::NULLBox, false)}
										}
                                    },
                                    (Some(a), Some(b)) => {
                                        err_break!{box_alter_type_error(a, b)}
                                    },
                                    (None, Some(_)) => {
										err_break!{needs_n_args_only_n_provided("box altr", "Two", "only one")}
									},
                                    (None, None) => {
										err_break!{needs_n_args_only_n_provided("box altr", "Two", "none")}
									},
                                    _ => {err_break!{should_never_get_here_for_func("box altr")}},
                                }
                            },
                            BoxCmd::Unknown => {
                                err_break!{"Box error! Unrecognized box operation! \
                                    Valid: free, null, make, open, altr".to_string()}
                            },
                        }
                    },
                    ASTNode::If{if_true: true_branch, if_false: false_branch} => {
                        match s.pop(){
                            Some(Value::Boolean(b)) => {
								let branches = [&false_branch, &true_branch];
								let branch_res = 
								run_program(branches[b as usize], s, h, vars, fns);

                                match branch_res{
                                    Ok(_) => (),
                                    Err(e) => {err_break!{e}}
                                }
                            },
                            Some(v) => {
                                err_break!{format!("If statement error! \
                                    Top of stack needs to be type Boolean \
                                    for effective branching to occur! \
                                    Attempted value: {}", v)}
                            },
                            None => {err_break!{needs_n_args_only_n_provided("if", "One", "none")}}
                        }
                    },
                    ASTNode::While(bod) => {
                        loop {
                            match s.pop(){
                                Some(Value::Boolean(b)) => {
                                    if b{
										let body_res = 
										run_program(&bod, s, h, vars, fns);

										match body_res{
											Ok(RetCode::Normal) => (),
											Ok(RetCode::LeavingScopeEarly) => {
												break;	
											},
											Err(e) => {err_break!{e}},
										}
                                    }else{
                                        break;
                                    }
                                },
                                Some(v) => {
                                    err_break!{format!("While loop error! Top of stack needs \
                                        to be of type Boolean to determine if loop needs \
                                        to run/run again! Attempted value: {}", v)}
                                },
                                None => {err_break!{needs_n_args_only_n_provided("while", "One", "none")}},
                            }
                        }
						//If the loop had an error, break out of the main loop.
						if let Err(_) = res {break;}
                    },
                    ASTNode::Function{cmd: c, func_name: name, func_bod: bod} => {
                        match c{
                            FunCmd::Define => {
								if !fns.func_def(name, Rc::clone(&bod)){
									err_break!{format!("Function definition (func def) error!\
									 Function \"{}\" is already defined!", name)}
								}
                            },
                            FunCmd::Call => {
                                let func_body = match fns.get_body(name){
                                    Some(b) => b,
                                    None => {
                                        err_break!{format!("Function call (func call) error! \
                                            Function \"{}\" is not defined! \
                                            Try defining it using func def !", name)}
                                    }, 
                                };
								let bod_res = 
								run_program(&func_body, s, h, vars, fns);

								if let Err(e) = bod_res{err_break!{e}}
                            },
                            FunCmd::Unknown => {
                                err_break!{format!("Function error! Invalid function \
                                    command given! Valid: def, call .")}
                            },
                        }
                    },
                    ASTNode::LocVar{name: nam, cmd: c, num: n} => {
                        match c{
                            VarCmd::Make => {
                                match s.pop(){
                                    Some(v) => {
										if !vars.mak_loc(nam, v){
											err_break!{variable_already_exists_error("loc mak", nam)}
										}
                                    },
                                    None => {
                                        err_break!{local_variable_lack_of_args_error("creation (mak)")}
                                    },
                                }
                            },
                            VarCmd::Get => {
								if let Some(v) = vars.get_loc(nam){
									s.push(v);
								}else{
									err_break!{variable_nonexist_error("loc get", nam)}	
								}
                            },
                            VarCmd::Mutate => {
                                match s.pop(){
                                    Some(new_v) => {
										match vars.mut_loc(nam, new_v){
											0 => (),
											1 => {
												err_break!{variable_nonexist_error("loc mut", nam)}
											},
											2 => {
												let old_v = vars.get_loc(nam).unwrap();	
												err_break!{invalid_mutation_error("loc mut", old_v, new_v)}
											},
											_ => {err_break!{should_never_get_here_for_func("var mut")}}
										}	
                                    },
                                    None => {
                                        err_break!{local_variable_lack_of_args_error("mutation (mut)")}
                                    },
                                }
                            },
                            _ => {
                                err_break!{"Local Variable (loc) error! Unrecognized local variable command! Valid: mak, get, mut".to_string()}
                            },
                        }
                    },
                    ASTNode::AttErr{attempt: att, err: error} => {
						let att_res = 
						run_program(att, s, h, vars, fns);

						match att_res{
							Ok(_) => (),
							Err(e1) => {
								s.push(h.insert_to_heap(HeapValue::String(e1)));
								
								let err_res = 
								run_program(error, s, h, vars, fns);

								match err_res{
									Ok(_) => (),
									Err(e2) => {
										res = Err(e2);
										break;
									},
								}
									
							},
						}
                    },
                    ASTNode::Defer(body) => {
                        if let Some(def) = &mut deferred{
                            def.push(Rc::clone(body));
                        }else{
                            deferred = Some(vec![Rc::clone(body)]);
                        }
                    },
                    ASTNode::CastTo(data_type) => {
						match run_operator(Operator::Cast, s, h, Some(data_type))
						{
							Ok(_) => (),
							Err(e) => {err_break!{e}},
						}
					},
                    _ => {},
                }
            }
        },
        _ => {res = Err("Should never get to this point in running!".to_string())},
    }

	if let (Some(def), Ok(_)) = (deferred, &res){
		for expr in def.iter().rev(){
			let expr_res = 
			run_program(expr, s, h, vars, fns);
			
			match expr_res{
				Ok(_) => (),
				Err(e) => {err_break!{e}}
			}	
		}
	}	

	vars.remove_frame();
	res
}

//Given an input program string and args, 
// parses and runs the entire program!
// It returns either the state generated by the program or an error string.
fn run_prog_from_str(argv: &Vec<String>, argc: usize, program_string: String,
prev_state: Option<(Stack, Heap, Variables, Functions)>
) -> 
(Result<RetCode, String>, Stack, Heap, Variables, Functions)
{
	//Clones previous state to be used in run_program, or makes new one.
	let (mut s, mut h, mut vs, mut fs);
	if let Some((ps, ph, pvs, pfs)) = prev_state{
		(s, h, vs, fs) = 
		(ps.clone(), ph.clone(), pvs.clone(), pfs.clone());	
	}else{
		(s, h, vs, fs) = 
		(Stack::new(), Heap::new(), Variables::new(), 
		Functions::new());
	}
	
	//Kicks back a parse error with the state 
	// or runs the program and updates the state.
	match parse_string_to_ast(&argv, argc, program_string){
		Ok((ast, num_unique_loc_vars)) => {
			let res = run_program(&ast, &mut s, &mut h, &mut vs, &mut fs);
			(res, s, h, vs, fs)
		},
		Err(e) => (Err(e), s, h, vs, fs)
	}
}

fn main(){
    //Creates argv and argc for finding file paths and stuff.
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    

    //Reads in data from file or from stdin, 
    // depending on inputs or lack thereof.
    if argc > 1{
		//If regular file name is given, run the program from file.
		// Otherwise activate the REPL.
		if argv[1] != "--repl"{
			//Used to hold a string read in from an input file.
			let mut file_string = String::new();
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

			let (res, _, _, _, _) = 
			run_prog_from_str(&argv, argc, file_string, None);

			match res{
				Ok(_) => (),
				Err(e) => println!("{}", e),
			}
		}else{
			let mut source_code: BTreeMap<usize, String> = BTreeMap::new();			
			let mut source_included = false;
			let mut print_stack = true;
        	let sep_str = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
			println!("Lmao REPL:\n{}", sep_str);
			let mut single_line_prog_str = String::new();
			let mut command_str = String::new();
			loop{
				println!("Enter code or command below:\n{}", sep_str);
				io::stdin().read_line(&mut single_line_prog_str).expect("FAILED TO READ!");

				//Sees if there's a valid command contained by the first token.	
				//Executes if so else treats like normal single-line program.
				for c in single_line_prog_str.chars(){
					if !c.is_whitespace(){
						command_str.push(c);
					}else{
						break;	
					}	
				}

				//Given the command string, executes the command.
				if command_str == "EXIT"{
					break;
				}

				//Running source code.
				if command_str == "RUN"{
					let mut source_string = String::new();
					for kv in source_code.iter(){
						source_string.push_str(&format!("{}\n", kv.1));
					}

					println!("\n{}\nProgram result:\n", sep_str);
					let (res, mut s, mut h, _, _) = 
					run_prog_from_str(&argv, argc, source_string, None);
					match res{
						Ok(_) => {
							if print_stack{
								debug_stack_print(&mut s, &mut h, None)
								.expect("FAILED TO PRINT STACK!");	
							}
						},
						Err(e) => println!("{}", e),
					}
					command_str.clear();
					single_line_prog_str.clear();
					continue;
				}

				//Code line case.
				if command_str.len() > 1 && 
					command_str.chars().nth(0).unwrap() == 'L' &&
					command_str.chars().nth(1).unwrap() != 'I' &&
					command_str.chars().nth(1).unwrap() != 'O' 
				{
					let mut code_str = String::new();
					let mut i = 0;
					for c in single_line_prog_str.chars(){
						if i >= command_str.len(){
							code_str.push(c)	
						}
						i += 1;	
					}	
					let mut is_first = true;	
					let mut line_num_str = String::new();
					for c in command_str.chars(){
						if !is_first{
							line_num_str.push(c)
						}
						is_first = false;
					}
					match line_num_str.parse::<usize>(){
						Ok(n) => {
							if code_str.len() > 2{
								source_code.insert(n, code_str);	
							}else{
								source_code.remove(&n);
							}
						},
						Err(_) => println!("Error! Invalid line number provided! Attempted line number: {}", line_num_str),
					}
					command_str.clear();
					single_line_prog_str.clear();
					continue;
				}
				
				//INCLUDE command makes source code part of line code 
				// or disables it.
				if command_str == "INCLUDE"{
					let en_or_dis = ["disabled", "enabled"];
					source_included = !source_included;
					println!("Source code inclusion {}.", en_or_dis[source_included as usize]);
										
					command_str.clear();
					single_line_prog_str.clear();
					continue;
				}

				//STACK command toggles the printing of the stack.
				if command_str == "STACK"{
					let en_or_dis = ["disabled", "enabled"];
					print_stack = !print_stack;
					println!("Stack printing {}.", en_or_dis[print_stack as usize]);
										
					command_str.clear();
					single_line_prog_str.clear();
					continue;
					
				}

				//Resets the state of the REPL, 
				// clearing source code and resetting bools.
				if command_str == "NEW"{
					println!("REPL State Reset!");	
					command_str.clear();
					single_line_prog_str.clear();
					source_code.clear();
					source_included = false;
					print_stack = true;
					continue;
				}
		
				//LIST command lists the source code.
				if command_str == "LIST"{
					println!("Current Written Program:\n{}", sep_str);	
					for kv in source_code.iter(){
						print!("{} {}", kv.0, kv.1);
					}
	
					command_str.clear();
					single_line_prog_str.clear();
					continue;
				}

				if command_str == "SAVE"{
					let mut file_name = String::new();
					print!("Enter file name: ");
                    io::stdout().flush().expect("FAILED TO FLUSH");
					io::stdin().read_line(&mut file_name).expect("FAILED TO READ");	

					file_name.pop().unwrap();
					let save_path = Path::new(&file_name);
                    match OpenOptions::new().write(true).truncate(true).create(true).open(save_path)
					{
						Ok(mut file) => {
							let mut string_to_write = String::new();
							for kv in source_code.iter(){
								string_to_write.push_str(kv.1);
							}
							match file.write_all(string_to_write.as_bytes()){
								Ok(_) => println!("File {} written successfully!", file_name),
								Err(reason) => {
									println!("Failed to save to file {} because {}", file_name, reason);
									
								},
							}
						},
						Err(reason) => println!("Failed to open file {} for saving because {}", file_name, reason),
					}

					command_str.clear();
					single_line_prog_str.clear();
					continue;
				}

				if command_str == "LOAD"{
					let mut file_name = String::new();
					let mut line_mul_str = String::new();

					print!("Enter file name: ");
                    io::stdout().flush().expect("FAILED TO FLUSH");
					io::stdin().read_line(&mut file_name).expect("FAILED TO READ");	
					file_name.pop().unwrap();
		
					print!("Enter line number multiple (default=1): ");
                    io::stdout().flush().expect("FAILED TO FLUSH");
					io::stdin().read_line(&mut line_mul_str).expect("FAILED TO READ");	
					line_mul_str.pop().unwrap();
	
					//Goes with default value of 1 if invalid number is provided.
					let line_number_mul: usize = line_mul_str.parse().unwrap_or(1);

					//Reads in hashmap and overwrites old source code contents with new ones.
					match OpenOptions::new().read(true).open(Path::new(&file_name)){
						Ok(mut file) => {
							let mut read_source = String::new();
							match file.read_to_string(&mut read_source) {
								Ok(_) => {
									println!("Successfully read in file {}", file_name);
									source_code.clear();
									let mut line_num: usize = 1;
									for line in read_source.split("\n").into_iter(){
										if line.len() > 0{
											source_code.insert(line_num * line_number_mul, format!("{}\n", line));
										}
										line_num += 1;	
									}	
								}
								Err(reason) => println!("Failed to read file {} because {}", file_name, reason),	
							}
						},
						Err(reason) => {
							println!("Failed to open file {} for reading because {}", file_name, reason);
						}	
					}
					
					command_str.clear();
					single_line_prog_str.clear();
					continue;
				}

        		println!("\n{}\nProgram result:\n", sep_str);
				if source_included{
					let mut code_with_include = String::new();
					for kv in source_code.iter(){
						code_with_include.push_str(&format!("{}\n", kv.1));
					}	
					code_with_include.push_str(&single_line_prog_str);
					single_line_prog_str = code_with_include;
				}
				let (res, mut s, mut h, _, _) = 
				run_prog_from_str(&argv, argc, single_line_prog_str.clone(), None);

				match res{
					Ok(_) => {
						if print_stack{
							debug_stack_print(&mut s, &mut h, None)
							.expect("FAILED TO PRINT STACK!");		
						}
					},
					Err(e) => println!("{}", e),
				}
				single_line_prog_str.clear();
				command_str.clear();
			}
		}
    }else{
        let sep_str = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
		let mut program_string = String::new();
        println!("Enter Lmao code below:\n{}", sep_str);
        io::stdin().read_to_string(&mut program_string)
            .expect("Stdin read error! Failed to read from stdin!");
        println!("\n{}\nProgram result:\n", sep_str);

		let (res, _, _, _, _) = 
		run_prog_from_str(&argv, argc, program_string, None);

		match res{
			Ok(_) => (),
			Err(e) => println!("{}", e),
		}
    }

}
