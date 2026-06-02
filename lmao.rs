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
                    ASTNode::Terminal(Token::Word((op, n))) => {
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
                                            if let HeapValue::Primitive(v) = &state.heap[bn].0{
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
                                            if let HeapValue::Primitive(old_v) = &mut state.heap[bn].0{
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
                    ASTNode::CastTo(data_type) => {
                        //58 is cast operator. 
                        // I know, I know, hardcoding gross, but it's fast!
                        let cast_index: usize = 58;

                        state.buffer.push_str(&data_type);

                        match state.ops[cast_index](state){
                            Ok(_) => (),
                            Err(e) => {
                                state.buffer.clear(); 
                                return error_and_remove_frame(state, e)
                            } 
                        }

                        state.buffer.clear();
                    }
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

//Given an input program string and args, 
// parses and runs the entire program!
// It returns either the state generated by the program or an error string.
fn run_prog_from_str(argv: &Vec<String>, argc: usize, program_string: String) -> Result<State, String>{
	match parse_string_to_ast(&argv, argc, program_string){
		Ok((ast, num_unique_loc_vars)) => {
			let mut state = State::new(num_unique_loc_vars);

			let result = run_program(&ast, &mut state);

			match result{
				Ok(_) => return Ok(state),
				Err(e) => return Err(e),
			}
		},
		Err(e) => return Err(e),
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

			match run_prog_from_str(&argv, argc, file_string){
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
					match run_prog_from_str(&argv, argc, source_string){
						Ok(mut state) => {
							if print_stack{
								debug_stack_print(&mut state).expect("FAILED TO PRINT STACK!");	
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
				match run_prog_from_str(&argv, argc, single_line_prog_str.clone()){
					Ok(mut state) => {
						if print_stack{
							debug_stack_print(&mut state).expect("FAILED TO PRINT STACK!");		
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

		match run_prog_from_str(&argv, argc, program_string){
			Ok(_) => (),
			Err(e) => println!("{}", e),
		}
    }

}
