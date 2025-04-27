# The Lmao Programming Language - The Spiritual Successor of EcksDee
## By Jesse A. Jones (KesseNones)

# <a name = "toc"></a> Table of Contents
- [0 Introduction](#intro)
- [1 Using a Stack-Based Approach](#stack-based)
- [2 Data Types Used](#data-types)
	- [2.1 Stack Data Types](#stack-types)
	- [2.2 Heap Data Types](#heap-types)
	- [2.3 Box Data Types](#box-types)
- [3 Operators](#ops)
	- [3.1 Math Operators](#math-ops)
	- [3.2 Maximum Integer Operators](#max-ops)
	- [3.3 Stack Operators](#stack-ops)
	- [3.4 Comparison Operators](#comp-ops)
	- [3.5 Logical Operators](#logic-ops)
	- [3.6 List & String Operators](#ls-str-ops)
	- [3.7 Character Operators](#char-ops)
	- [3.8 Object Operators](#obj-ops)
	- [3.9 Bitwise Operators](#bit-ops)
	- [3.10 IO Operators](#io-ops)
		- [3.10.1 Debug IO Operators](#debug-io-ops)
	- [3.11 File Operators](#file-ops)
	- [3.12 Type Operators](#type-ops)
	- [3.13 Meta Operators](#meta-ops)
	- [3.14 Other Operators](#other-ops)
- [4 Fancy Operators](#fancy-ops)
	- [4.1 Box Fancy Operators](#box-fan-ops)
- [5 Conclusion](#conclusion)
## <a name = "intro"></a>0 Introduction
### [**Return to Table of Contents**](#toc)

Welcome to Lmao, the spiritual successor of the EcksDee programming language! This language was made using the general layout of EcksDee as a base but built in Rust instead of Haskell. This resulted in a notably faster language since Rust allows mutable state and has a lot of constant time data structures to take advantage of. Overall, this language aims to do what EcksDee did but better, with more sensible and consistent behavior on the stack with each operator and an overall greater performance.

## <a name = "stack-based"></a>1 Using a Stack-Based Approach
### [**Return to Table of Contents**](#toc)

Like EcksDee, Lmao operates in a similar manner to Forth, with a stack and operators for the stack. However, there is a greater consistency in behavior with the stack and how operators work on it. Combined with the stack is also a built in heap to improve efficiency for more complex data types.

The stack works like this: data types are pushed to the stack and operators consume those data and push new data to the stack after successfully completing the operation. 

For example:
```
2 3 +
```
This code pushes `isize 2` and `isize 3` to the stack and then uses the `+` operator which consumes the two values on the stack and pushes a new `isize 5` that is the result of adding the two values of compatible types together. 

What's an `isize`? What are operators? All in good time. The point is that the programming language works by using operators that take values pushed to the stack which then potentially produce new values.

## <a name = "data-types"></a> 2 Data Types Used
### [**Return to Table of Contents**](#toc)
An important thing to note is that Lmao is statically typed, meaning that types can't easily change into each other as operations occur. This means that an isize can't implicity become an f32, for example. Instead, explicit casting must be used. This just makes code more consistent and less prone to bugs. 

Listed below is all data types, also known as values, employed by Lmao both on the stack and the heap. The main difference between stack and heap values is that the stack values are pushed directly to the stack while heap values are allocated on the heap and a box type is pushed instead. The following sections explain both main types and what boxes are.

### <a name = "stack-types"></a> 2.1 Stack Data Types
#### [**Return to Table of Contents**](#toc)
Stack data types are data types directly pushed to the stack and not tied to the heap in any way, either to live on the heap or as box data types. These data types are purely primitives. 

Here are all of them:
Signed Integers:
- isize
	- A signed integer optimized to the word size of the given machine's CPU architecture. On most modern machines, isize would be a signed 64 bit integer but earlier systems might have it be a signed 32 bit integer. 
	- Thus, on modern systems, isize ranges from -2^31 to 2^31 - 1 or `-9223372036854775808` to `9223372036854775807`.
	- To push an isize to the stack, simply type a positive or negative whole number (integer) in the given range. This is also the default type for integers so you can just type the number alone and it will read it in as an integer. Example `42` is read as an isize. If you want to guarantee that the given number is an isize, you can push it like so: `42isize` which explicitly states that the value pushed is of type isize.  
- i8
	- A signed eight bit integer.  
	- Ranges from -2^7 to 2^7 - 1 or `-128` to `127`.
	- To push an i8 to the stack, you write an integer in the given range and indicate clearly that it's an i8 like so: `23i8`. Unlike isize, i8 is not the default and so must be explicitly stated that it's an i8 like in the given example.
- i16
	- A signed sixteen bit integer.
	- Ranges from -2^15 to 2^15 - 1 or `-32768` to `32767`.
	- To push an i16 to the stack, type out an integer and concatenate its data type like so: `5040i16`.
- i32
	- A signed thirty two bit integer.
	- Ranges from -2^31 to 2^31 - 1 or `-2147483648` to `2147483647` 
	- To push an i32 to the stack, type out an integer and concatenate its data type like so: `1000000000i32`.
- i64
	- A signed sixty four bit integer.
	- Ranges from -2^63 to 2^63 - 1 or `-9223372036854775808` to `9223372036854775807`. 
	- To push an i64 to the stack, type out an integer and concatenate its data type like so: `29998559671349i64`.
- i128
	- A signed one hundred and twenty eight bit integer.
	- Ranges from -2^127 to 2^127 - 1 or `-170141183460469231731687303715884105728` to `170141183460469231731687303715884105727`. 
	- To push an i128 to the stack, type out an integer and concatenate its data type like so: `999888777666555444333222111000i128`.
	- Such a colossal number is rarely needed but it's nice to have. 

Unsigned Integers:
- usize
	- An unsigned integer optimized to the word size of the given machine's CPU architecture. On most modern machines, usize would be an unsigned 64 bit integer but earlier systems might have it be an unsigned 32 bit integer. 
	- Thus, on modern systems, usize ranges from 0 to 2^32 - 1 or `0` to `18446744073709551615`.
	- To push a usize to the stack, simply type a positive whole number (integer) in the given range like so: `2319usize`. Unlike isize, usize is not the default integer to be interpreted so you have to explicitly say that the value being pushed is a usize, as demonstrated.
- u8
	- An unsigned eight bit integer.  
	- Ranges from 0 to 2^8 - 1 or `0` to `255`.
	- To push a u8 to the stack, you write an integer in the given range and indicate clearly that it's a u8 like so: `225u8`.
- u16
	- An unsigned sixteen bit integer.
	- Ranges from 0 to 2^16 - 1 or `0` to `65535`.
	- To push a u16 to the stack, type out a positive integer and concatenate its data type like so: `49999u16`.
- u32
	- An usigned thirty two bit integer.
	- Ranges from 0 to 2^32 - 1 or `0` to `4294967295` 
	- To push a u32 to the stack, type out a positive integer and concatenate its data type like so: `3297486222u32`.
- u64
	- An unsigned sixty four bit integer.
	- Ranges from 0 to 2^64 - 1 or `0` to `18446744073709551615`. 
	- To push a u64 to the stack, type out an integer and concatenate its data type like so: `5040u64`.
- u128
	- An unsigned one hundred and twenty eight bit integer.
	- Ranges from 0 to 2^128 - 1 or `0` to `340282366920938463463374607431768211455`. 
	- To push an i128 to the stack, type out an integer and concatenate its data type like so: `999888777666555444333222111000i128`.
	- Such a colossal number is rarely needed but it's nice to have. 

Floating Points:
- f32
	- A thirty-two bit floating point number.
	- With 1 bit for the sign, 8 bits for the exponent (biased by 127 for special values), and 23 bits for the mantissa, a range of `-3.4028235e+38` to `3.4028235e+38` is possible.
	- To push an f32 to the stack, type out a floating point number. Since f32 is the default data type for floats, you can just type it without indicating the type, unless you're typing an integer, then it will think it's an isize so indication is required then. 
	- Valid ways to push: `3.14` `6f32` `3.14f32` `6.02e23` `1e30f32`
	- Generally, if there's no decimal point, add an `f32` to the end of the number just to be sure.
	- You can also do two non-number types like `inf` and `NaN`
		- To push these, you have to put `f32` at the end, otherwise it'll just think you're trying to type an operator.
		- The case doesn't actually matter, as long as you spell it right and have `f32` at the end.
		- Valid examples: `INFf32` `NANf32` `inff32` `nanf32` `Inff32` `Nanf32` `InFf32` `NaNf32`, and more.
- f64
	- A sixty-four bit floating point number.
	- With 1 bit for the sign, 11 bits for the exponent (biased by 1023 for special values), and 52 bits for the mantissa, a range of `-1.7976931348623157e+308` to `1.7976931348623157e+308` is possible.
	- To push an f64 to the stack, type out a number that can include a decimal point and concatenate `f64` to it. Since f64 isn't the default float, you have to indicate that you're pushing an f64 using `f64` at the end.
	- Valid ways to push: `42f64` `1e+100f64` `6.02e23f64` `0.0000000002f64` `1.5f64`
	- You can also do two non-number types like `inf` and `NaN`
		- To push these, you have to put `f64` at the end, otherwise it'll just think you're trying to type an operator.
		- The case doesn't actually matter, as long as you spell it right and have `f64` at the end.
		- Valid examples: `INFf64` `NANf64` `inff64` `nanf64` `Inff64` `Nanf64` `InFf64` `NaNf64`, and more.

- Char
	- An unsigned 32 bit integer encoded into a specific Unicode character value. 
	- Though an integer under the hood, it looks like a single character when pushed to and shown on the stack. 
	- To push a Char to the stack, you encase a valid character inside two apostrophes like so: `'9'`.
	- To push whitespace Chars, the escape character "\" can be used like so: `'\n'` `'\t'` `'\r'` `'\0'`, and more.
	- Since Char supports full Unicode, non-ascii symbols can also be in characters like emojis, foreign characters, etc. For example: `'😄'` `'十'`, etc.
- Boolean
	- An unsinged eight bit integer under the hood but only occupies two possible values of `true` or `false`. 
	- To push a Boolean to the stack, simply push like so: `true` or `false`.
	- Capitalization also doesn't matter so you can push a Boolean like this too: `True` or `False`.
	- This is just to have more options, they both mean the same.

### <a name = "heap-types"></a> 2.2 Heap Data Types
#### [**Return to Table of Contents**](#toc)

Instead of being allocated on the stack directly, heap values are written to Lmao's heap, which is an arena, and pushes a box data type pointing to the heap allocated value. Box data types will be explained in the following section. 

These are all the heap data types:
- String
	- A collection of bytes encoded in UTF-8 and displayed as text when printed. 
	- Used in reading and writing user input, and reading and writing files.
	- To allocate a string to be used, type out a set of text in double-quotes like so: `"Hello, world!"`
	- Be aware that to type double quotes inside a string, you have to use the backslash escape character to make it work.
	- Following pushing such a string, the stack will be holding a StringBox pointing to the String on the heap. 
- List
	- A collection of stack data types and box data types stored contiguously. 
	- Lists can be efficiently indexed.
	- This data type is like Python's lists.
	- At present, there's no way to push a list with stuff on it so an empty list is the only option to allocate a new list. From there, operators can be used to push data to the list.
	- To allocate a new list, simply type out the empty list construction like so: `[]`. This creates a brand-new empty list that can store whatever you want. It doesn't care about mixtures of data types. The final stack will then have a ListBox on it.
- Object
	- Used to store key-value pairs of Strings to stack data types or box data types. 
	- These work like JavaScript's objects and can be used for some object oriented programming.
	- Like Lists, there is no way to create an Object with stuff on it, so an empty object has to be pushed and special operators are needed to then add and alter fields in it.
	- To create an empty object, simply push the empty Object constructor like so: `{}`.
	- The empty Object will then be placed on the heap with an ObjectBox on the stack.
- Primitive
	- Used by MiscBoxes to store stack data types and box data types on the heap rather than on the stack. 
	- This is useful for nested boxes that allow for the equivalent of multi-level pointers seen in other languages. 
	- A Primitive heap value is only allocated after using the `box make ;` fancy operator with something on the stack to encapsulate on the heap.
### <a name = "box-types"></a> 2.3 Box Data Types
#### [**Return to Table of Contents**](#toc)

Box data types are values that live on the stack but point to values held by the heap. Thus, box types serve as a sort of middle-man, allowing the stack to stay lightweight but still have access to more complex heap values. The general format of a box on the stack is: (SOMETHING)Box n, where n represents the index the data stored by the box lives at on the stack. For instance, after pushing the String `"Hello, world!"`, the stack will have `StringBox 0`, this means that the heap is holding a String at index 0 on the heap. It is possible for boxes to be invalid since freeing is a thing that can happen. In that case, printing from the stack will indicate invalidity like so: `StringBox 0 [INVALID]`. Freeing and box operators in general will be discussed in their own section later on. 

Here are all of the box types:
- StringBox
	- Is used to hold Strings.
	- Is pushed to the stack after a String is given to be stored on the heap.
- ListBox
	- Used to hold Lists.
	- Pushed after empty List construct is given.
- ObjectBox
	- Used to hold Objects. 
	- Pushed after empty Object construct is given.
- MiscBox
	- Used to hold whatever is at the top of the stack.
	- Created using the `box make ;` fancy operator.
	- Useful for creating nested pointers and other custom boxes.
- NULLBox
	- A placeholder box that can be stored in places to indicate that a box will go there just not right now.
	- Can turn into StringBox, ListBox, ObjectBox, MiscBox, and even itself. 
	- All other box types can turn into NULLBox.
	- Created using `box null ;` fancy operator.

## <a name = "ops"></a> 3 Operators
### [**Return to Table of Contents**](#toc)
Operators are how Lmao gets things done. Operators take values held on the stack, consume them, and produce new ones. They're kind of like functions in other languages. Some operators take no operands and produce nothing on the stack while some only push things to the stack. Generally though, it's an input output scheme.  

### <a name = "math-ops"></a>3.1 Math Operators
#### [**Return to Table of Contents**](#toc)
These operators are used to perform calculations in Lmao, from the four basic arithmetic operators to data type specific maxima, and beyond.

These are the operators involved:
- `+`
	- Performance: O(1)
	- Consumes two matching types that are integers or floats and pushes a value on the stack that is the result of the two being added together.
	- For integers, the `+` operator does allow overflow so be careful about that.

	- Example Program:
	
		```
		2 3 +
		3.14 2.718 +
		5e100f64 6e100f64 +
		22u8 1u8 +
		//Overflow!
		255u8 1u8 +
	
		//Floats being funny.
		0.1f64 0.2f64 +
	
		//Indirect subtraction.
		1i32 -1i32 +
		
		//Displays the stack to stdout.
		debugPrintStack
		```
	- Resulting Output: 
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 5
		f32 5.858
		f64 1.1e101
		u8 23
		u8 0
		f64 0.30000000000000004
		i32 0
		--------------------------------
		STACK LENGTH: 7
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `-`
	- Performance: O(1)
	- Consumes two matching types that are integers or floats subtracts the top from the second to top item and pushes the resulting subtracted number.
	- For integers, the `-` operator does allow underflow so be careful about that.

	- Example Program:
	
		```
		2 3 -
		3.14 2.718 -
		5e100f64 6e100f64 -
		22u8 1u8 -
		//Underflow!
		0u8 1u8 -
	
		//Indirect addition.
		1i32 -1i32 +
	
		//Displays the stack to stdout.
		debugPrintStack
		```
	- Resulting Output: 
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize -1
		f32 0.42200017
		f64 -9.999999999999998e99
		u8 21
		u8 255
		i32 0
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `*`
	- Performance: O(1)
	- Consumes two matching types that are integers or floats on the stack, multiplies them, and pushes the result.
	- For integers, the `*` operator does allow overflow so be careful about that. 

	- Example Program:
	
		```
		6 7 *
		7 6 5 4 3 2 1 * * * * * * //Seven factorial
	
		3.14f64 3.14f64 *
	
		//Overflow!
		128u8 2u8 * 
		
		//Displays the stack to stdout.
		debugPrintStack
		```
	- Resulting Output: 
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 42
		isize 5040
		f64 9.8596
		u8 0
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `/`
	- Performance: O(1)
	- Consumes two matching types that are integers or floats on the stack, divides the second to top item by the top and pushes the result. 
	- Overflow can probably still happen though is less likely.

	- Example Program:
	
		```
		//Integer division.
		3 2 / 
	
		//Floating point division.
		3.0 2.0 / 
	
		3.14159265358979323f64 2.718f64 / 
	
		5040.0 12.0 / 
	
		//Displays the stack to stdout.
		debugPrintStack
		```
	- Resulting Output: 
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 1
		f32 1.5
		f64 1.1558471867512117
		f32 420
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `mod` or `%`
	- Performance: O(1)
	- Consumes two items from the stack of matching integer types, uses the top item to perform modulo on the second to top item, and pushes the result.
	- Modulo is the process of performing an integer division and focusing only on the remainder, rather than the resulting whole number, i.e. 5 mod 2 is 1 since 1 is the remainder.

	- Example Program:
	
		```
		5usize 2usize mod
	
		//Basic leap year test for 2025.
		// (It's not a leap year)
		2025 4 mod
	
		//% is also a valid symbol for mod operator.
		931 27 %
	
		23u8 23u8 mod
	
		5040 33 %
	
		2048 8 mod
	
		//Displays the stack to stdout.
		debugPrintStack
		```
	- Resulting Output: 
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 1
		isize 1
		isize 13
		u8 0
		isize 24
		isize 0
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `pow`
	- Performance: O(1)
	- Consumes two matching floating point types from the stack, exponentiates the second to top value by the top value and pushes the result.
	- In other words, this is what can be used to raise a number to the power of something. This also works for taking square roots.

	- Example Program:
	
		```
		2.0 3.0 pow
	
		2.0f64 10f64 pow
	
		4761f32 0.5 pow
	
		2025f32 0.5 pow
	
		//Cubic root.
		8.0 0.3333333333333 pow
	
		//Displays the stack to stdout.
		debugPrintStack
		```
	- Resulting Output: 
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		f32 8
		f64 1024
		f32 69
		f32 45
		f32 2
		--------------------------------
		STACK LENGTH: 5
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

### <a name = "max-ops"></a>3.2 Maximum Integer Operators
#### [**Return to Table of Contents**](#toc)
- These operators are useful for finding the exact parameters maximum values of integers. These operators are very simple, pushing the max value of their given data type to the stack. This can then be used to find the minimum value of the given data type by doing a bitwise not of the max using  the `bitNot` operator.
- For the sake of conciseness, all max operators work the same so they're all shown in the example program below. Otherwise it's a repetition of a lot of the same information.
- Example Program:

	```
	isizeMax
	i8Max
	i16Max
	i32Max
	i64Max
	i128Max
	
	usizeMax
	u8Max
	u16Max
	u32Max
	u64Max
	u128Max

	debugPrintStack
	```
- Resulting Output:

	```
	--------------------------------
	BEGIN STACK PRINT
	--------------------------------
	isize 9223372036854775807
	i8 127
	i16 32767
	i32 2147483647
	i64 9223372036854775807
	i128 170141183460469231731687303715884105727
	usize 18446744073709551615
	u8 255
	u16 65535
	u32 4294967295
	u64 18446744073709551615
	u128 340282366920938463463374607431768211455
	--------------------------------
	STACK LENGTH: 12
	--------------------------------
	END STACK PRINT
	--------------------------------
	```

- isize and usize would be smaller on your machine if you use a 32-bit architecture.

### <a name = "stack-ops"></a>3.3 Stack Operators
#### [**Return to Table of Contents**](#toc)
Stack operators are the operators you use when directly manipulating the stack itself. Data types don't matter here, only the number of operands needed for each operator. 

These are all of them:
- `swap` 
	- Performance: O(1)
	- Given two items at the second-to-top and top of the stack, pops them both and pushes them to the stack in reverse-order. 
	- General form: stack `x y` after `swap` becomes stack `y x`
	- Example Program:

		```
		'a' 'b' debugPrintStack
		swap debugPrintStack
		
		"foo" "bar" debugPrintStack
		swap debugPrintStack
		
		[] {} debugPrintStack
		swap debugPrintStack
		```

	- Resulting Output: 

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'a'
		Char 'b'
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'b'
		Char 'a'
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'b'
		Char 'a'
		StringBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'b'
		Char 'a'
		StringBox 1
		StringBox 0
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'b'
		Char 'a'
		StringBox 1
		StringBox 0
		ListBox 2
		ObjectBox 3
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'b'
		Char 'a'
		StringBox 1
		StringBox 0
		ObjectBox 3
		ListBox 2
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `drop` 
	- Performance: O(1)
	- Given an item at the top of the stack, removes it.
	- Unlike other operators, this just erases a value from the stack, so be sure you either didn't need that value anymore or you save it in a variable or something.
	- General form: stack `x` after `drop` becomes stack ``
	- Example Program:

		```
		1 2 3 4 5 666 debugPrintStack
		drop debugPrintStack
		```

	- Resulting Output: 

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 1
		isize 2
		isize 3
		isize 4
		isize 5
		isize 666
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 1
		isize 2
		isize 3
		isize 4
		isize 5
		--------------------------------
		STACK LENGTH: 5
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `dropStack` 
	- Performance: O(1)
	- Clears the stack, regardless of its original state.
	- This means that you can have anything or nothing on the stack and it will clear it all the same.
	- Be aware that any boxes removed from the stack through this operation means that those boxes are lost and the memory is still on the heap. To remedy this, free boxes before doing this operation or save the respective box values to variables to be freed later.
	- Example Program:

		```
		//Clears stack with stuff on it.
		1 2 3 4 5 666 debugPrintStack
		dropStack debugPrintStack
		
		//Does nothing to empty stack.
		debugPrintStack dropStack debugPrintStack
		
		//Different set of stuff cleared off.
		"foo" "bar" "baz" {} [] debugPrintStack
		dropStack debugPrintStack
		```

	- Resulting Output: 

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 1
		isize 2
		isize 3
		isize 4
		isize 5
		isize 666
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		--------------------------------
		STACK LENGTH: 0
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		--------------------------------
		STACK LENGTH: 0
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		--------------------------------
		STACK LENGTH: 0
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		StringBox 1
		StringBox 2
		ObjectBox 3
		ListBox 4
		--------------------------------
		STACK LENGTH: 5
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		--------------------------------
		STACK LENGTH: 0
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `rot` 
	- Performance: O(1)
	- Given the existence of items in the top three elements of the stack, rot pops all three items, pushes the former top item, then pushes the other two in the same order they were in previously. 
	- General form: stack `x y z` after `rot` becomes stack `z x y`
	- Example Program:

		```
		'x' 'y' 'z' debugPrintStack
		rot debugPrintStack
		
		1 2.718 3.14 debugPrintStack
		rot debugPrintStack
		
		"foo" "bar" "baz" debugPrintStack
		rot debugPrintStack
		
		```

	- Resulting Output: 

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'y'
		Char 'z'
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'z'
		Char 'x'
		Char 'y'
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'z'
		Char 'x'
		Char 'y'
		isize 1
		f32 2.718
		f32 3.14
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'z'
		Char 'x'
		Char 'y'
		f32 3.14
		isize 1
		f32 2.718
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'z'
		Char 'x'
		Char 'y'
		f32 3.14
		isize 1
		f32 2.718
		StringBox 0
		StringBox 1
		StringBox 2
		--------------------------------
		STACK LENGTH: 9
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'z'
		Char 'x'
		Char 'y'
		f32 3.14
		isize 1
		f32 2.718
		StringBox 2
		StringBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 9
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `dup` 
	- Performance: O(1)
	- Given one item at the top of the stack, pops the item, duplicates it, and pushes both the original and clone to the stack. 
	- General form: stack `x` after `dup` becomes stack `x x'` where `x'` is the same as `x`.
	- For box types be aware that it doesn't copy the memory underneath; it only duplicates the box itself which is a constant time copy. Both the box and the copied box still have the same number and thus point to the same memory. The `dup` operator can also copy invalid boxes since all it does is just copy the box itself.
	- Example Program:

		```
		'x' debugPrintStack dup debugPrintStack
		
		1 2 3 debugPrintStack dup debugPrintStack
		
		//Only copies box and not underlying memory!
		"foo" debugPrintStack dup debugPrintStack
		
		//Can copy invalid boxes too!
		[] dup box free ; debugPrintStack 
		dup debugPrintStack
		```

	- Resulting Output: 

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		isize 1
		isize 2
		isize 3
		--------------------------------
		STACK LENGTH: 5
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		isize 1
		isize 2
		isize 3
		isize 3
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		isize 1
		isize 2
		isize 3
		isize 3
		StringBox 0
		--------------------------------
		STACK LENGTH: 7
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		isize 1
		isize 2
		isize 3
		isize 3
		StringBox 0
		StringBox 0
		--------------------------------
		STACK LENGTH: 8
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		isize 1
		isize 2
		isize 3
		isize 3
		StringBox 0
		StringBox 0
		ListBox 1 [INVALID]
		--------------------------------
		STACK LENGTH: 9
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		isize 1
		isize 2
		isize 3
		isize 3
		StringBox 0
		StringBox 0
		ListBox 1 [INVALID]
		ListBox 1 [INVALID]
		--------------------------------
		STACK LENGTH: 10
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `deepDup` 
	- Performance: 
		- O(1) for stack data types and `NULLBox`
		- O(n) for valid box data types
	- Given one item at the top of the stack, pops the item, duplicates it, and pushes both the original and clone to the stack. 
	- General form: 
		- Stack value or `NULLBox`: stack `x` after `deepDup` becomes stack `x x'` where `x'` is the same as `x`.
		- Valid box value: stack `x` after `deepDup` becomes stack `x y` where `y` holds new memory allocated on the heap that is identical to `x`'s contents.
	- If you try to use `deepDup` on an invalid box, it throws an error since it can't copy the underlying data the box holds.
	- Example Program:

		```
		//Same as dup for regular values.
		'x' debugPrintStack
		deepDup debugPrintStack
		
		//Same as dup for NULLBox
		box null ; debugPrintStack
		deepDup debugPrintStack
		
		//Memory duplication.
		"foo" debugPrintStack
		deepDup debugPrintStack
		//Shows off heap.
		debugPrintHeap
		```

	- Resulting Output: 
		
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		NULLBox
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		NULLBox
		NULLBox
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		NULLBox
		NULLBox
		StringBox 0
		--------------------------------
		STACK LENGTH: 5
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'x'
		Char 'x'
		NULLBox
		NULLBox
		StringBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "foo"
		StringBox 1:
		        String "foo"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

### <a name = "comp-ops"></a>3.4 Comparison Operators
#### [**Return to Table of Contents**](#toc)
Comparison operators are used for exactly what they're called: comparison. These are used largely for branching and loop determinations but are also useful in other ways. These operators are used for testing equality, less than, greater than, and more. With the exception of `stringCompare`, the other operators listed also work with box types. Box types are included to make pointer-like comparisons possible, which can be useful for seeing if a chunk of memory is allocated later than another or if two boxes point to the same memory. 

Listed are all comparison operators:
- `==` 
	- Performance: O(1)
	- Given two items on the stack of the same data type, pops both of them and performs an equality comparison between them, pushing a Boolean to the stack based on whether or not the two consumed items are equal to each other. 
	- General form: 
		- Stack `x y` where both `x` and `y` are matching types of type `t` where `t` can be any type. After applying `==` stack `x y` becomes stack `b` where `b` is a Boolean that states whether or not `x` is equal to `y`.
		- A notable exception to the general form is that if `x` and `y` are box types, then `x` or `y` can be a NULLBox, meaning that NULLBox can be compared with itself, a StringBox, ListBox, ObjectBox, or a MiscBox. This exception is made so that it's possible to check if something holding a box is a NULLBox.  
	- The following example program has some advanced code that hasn't been discussed yet but the output is formatted so that it's human-readable. The resulting stack is also still printed too.
	- Example Program:

		```
		//Takes in two operands and checks for equality between them, 
		// printing the result in a nice formatted print statement 
		// and pushing the original operands 
		// and the comparison result 
		// to the stack for the later stack print.
		//This function makes no effort to free memory used 
		// but it's fine since it's a small program overall.
		func def compareAndPrint
			loc mak y ;
			loc mak x ;
		
			//Compares two inputs and saves result.
			loc get x ; loc get y ; ==
			loc mak b ;
		
			//Fetches the inputs, converts them to strings,
			// puts them into a new string that gets formatted
			// with output of comparison and prints is as one
			// big old string.
			""
			loc get x ; "String" cast ++ 
			" equals " ++
			loc get y ; "String" cast ++
			' ' push '?' push ' ' push
			loc get b ; "String" cast ++
			printLine
		
			//Puts original values back on stack, 
			// as well as comparison.
			loc get x ; 
			loc get y ;
			loc get b ;
		;
		
		2 3 func call compareAndPrint ;
		6 6 func call compareAndPrint ;
		3.14 3.14 func call compareAndPrint ;
		
		//This will be false since floats use binary.
		0.1f64 0.2f64 + 0.3f64 func call compareAndPrint ;
		
		'a' 'b' func call compareAndPrint ;
		'a' 'a' func call compareAndPrint ;
		'\n' '
		' func call compareAndPrint ;

		//Is equal because the box numbers are the same.
		"foo" dup func call compareAndPrint ;
		
		//False because the contents may be the same 
		// but the box numbers aren't equal, 
		// which is what it's actually comparing.
		"bar" "bar" func call compareAndPrint ; 

		//This more clearly shows the StringBox comparisons.
		debugPrintStack
		
		```

	- Resulting Output: 
		
		```
		2 equals 3 ? false
		6 equals 6 ? true
		3.14 equals 3.14 ? true
		0.30000000000000004 equals 0.3 ? false
		a equals b ? false
		a equals a ? true
		
		 equals
		 ? true
		foo equals foo ? true
		bar equals bar ? false
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2
		isize 3
		Boolean false
		isize 6
		isize 6
		Boolean true
		f32 3.14
		f32 3.14
		Boolean true
		f64 0.30000000000000004
		f64 0.3
		Boolean false
		Char 'a'
		Char 'b'
		Boolean false
		Char 'a'
		Char 'a'
		Boolean true
		Char '\n'
		Char '\n'
		Boolean true
		StringBox 56
		StringBox 56
		Boolean true
		StringBox 63
		StringBox 64
		Boolean false
		--------------------------------
		STACK LENGTH: 27
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `!=` 
	- Performance: O(1)
	- Given two items on the stack of the same data type, pops both of them and performs an inequality comparison between them, pushing a Boolean to the stack based on whether or not the two consumed items are not equal to each other. 
	- General form: 
		- Stack `x y` where both `x` and `y` are matching types of type `t` where `t` can be any type. After applying `!=` stack `x y` becomes stack `b` where `b` is a Boolean that states whether or not `x` is not equal to `y`.
		- A notable exception to the general form is that if `x` and `y` are box types, then `x` or `y` can be a NULLBox, meaning that NULLBox can be compared with itself, a StringBox, ListBox, ObjectBox, or a MiscBox. This exception is made so that it's possible to check if something isn't a NULLBox. This is good for null checking boxes in data structures.
	- The following example code is advanced but what matters is the comparison itself and the outcomes.
	- Example Program:

		```
		//Takes in two operands and checks 
		// for inequality between them, 
		// printing the result in a nice formatted print statement 
		// and pushing the original operands 
		// and the comparison result 
		// to the stack for the later stack print.
		//This function makes no effort to free memory used 
		// but it's fine since it's a small program overall.
		func def compareAndPrint
			loc mak y ;
			loc mak x ;
		
			//Compares two inputs and saves result.
			loc get x ; loc get y ; !=
			loc mak b ;
		
			//Fetches the inputs, converts them to strings,
			// puts them into a new string that gets formatted
			// with output of comparison and prints is as one
			// big old string.
			""
			loc get x ; "String" cast ++ 
			" doesn't equal " ++
			loc get y ; "String" cast ++
			' ' push '?' push ' ' push
			loc get b ; "String" cast ++
			printLine
		
			//Puts original values back on stack, 
			// as well as comparison.
			loc get x ; 
			loc get y ;
			loc get b ;
		;
		
		2 3 func call compareAndPrint ;
		6 6 func call compareAndPrint ;
		3.14 3.14 func call compareAndPrint ;
		
		//This will be true since floats use binary.
		0.1f64 0.2f64 + 0.3f64 func call compareAndPrint ;
		
		'a' 'b' func call compareAndPrint ;
		'a' 'a' func call compareAndPrint ;
		'\n' '
		' func call compareAndPrint ;
		
		//Isn't inequal because the box numbers are the same.
		"foo" dup func call compareAndPrint ;
		
		//True because the contents may be the same 
		// but the box numbers aren't equal, 
		// which is what it's actually comparing.
		"bar" "bar" func call compareAndPrint ; 
		
		//This more clearly shows the StringBox comparisons.
		debugPrintStack
		
		```

	- Resulting Output: 
		
		```
		2 doesn't equal 3 ? true
		6 doesn't equal 6 ? false
		3.14 doesn't equal 3.14 ? false
		0.30000000000000004 doesn't equal 0.3 ? true
		a doesn't equal b ? true
		a doesn't equal a ? false
		
		 doesn't equal
		 ? false
		foo doesn't equal foo ? false
		bar doesn't equal bar ? true
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2
		isize 3
		Boolean true
		isize 6
		isize 6
		Boolean false
		f32 3.14
		f32 3.14
		Boolean false
		f64 0.30000000000000004
		f64 0.3
		Boolean true
		Char 'a'
		Char 'b'
		Boolean true
		Char 'a'
		Char 'a'
		Boolean false
		Char '\n'
		Char '\n'
		Boolean false
		StringBox 56
		StringBox 56
		Boolean false
		StringBox 63
		StringBox 64
		Boolean true
		--------------------------------
		STACK LENGTH: 27
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `>` 
	- Performance: O(1)
	- Given two non-NULLBox items on the stack, pops both of them and compares the former second-to-top with the former top of the stack and determines if the second-to-top is greater than the top, pushing a Boolean to the stack to reflect the results. 
	- General form: 
		- Stack `x y` where both `x` and `y` are matching types of type `t` where `t` can be any type that isn't NULLBox. After applying `>` stack `x y` becomes stack `b` where `b` is a Boolean that states whether or not `x` is greater than `y`.
	- The following example code is advanced but what matters is the comparison itself and the outcomes.
	- Example Program:

		```
		//Takes in two operands and checks 
		// to see if the second-to-top item 
		// is greater than the top item, 
		// printing the result in a nice formatted print statement 
		// and pushing the original operands 
		// and the comparison result 
		// to the stack for the later stack print.
		//This function makes no effort to free memory used 
		// but it's fine since it's a small program overall.
		func def compareAndPrint
			loc mak y ;
			loc mak x ;
		
			//Compares two inputs and saves result.
			loc get x ; loc get y ; >
			loc mak b ;
		
			//Fetches the inputs, converts them to strings,
			// puts them into a new string that gets formatted
			// with output of comparison and prints is as one
			// big old string.
			""
			loc get x ; "String" cast ++ 
			" is greater than " ++
			loc get y ; "String" cast ++
			' ' push '?' push ' ' push
			loc get b ; "String" cast ++
			printLine
		
			//Puts original values back on stack, 
			// as well as comparison.
			loc get x ; 
			loc get y ;
			loc get b ;
		;
		
		2 3 func call compareAndPrint ;
		6 6 func call compareAndPrint ;
		3.14 3.14 func call compareAndPrint ;
		
		//This will be true since floats use binary.
		0.1f64 0.2f64 + 0.3f64 func call compareAndPrint ;
		
		'a' 'b' func call compareAndPrint ;
		'a' 'a' func call compareAndPrint ;
		'z' 'a' func call compareAndPrint ;
		'\n' '
		' func call compareAndPrint ;
		
		//False since it's the same box.
		"foo" dup func call compareAndPrint ;
		
		//False because first "bar" is allocated earlier, 
		// making its box number lower and thus not greater.
		"bar" "bar" func call compareAndPrint ; 
		
		//True because the second ListBox was swapped, 
		// making it greater than the top since it's at the top.
		[] deepDup swap func call compareAndPrint ;
		
		debugPrintStack
		
		```

	- Resulting Output: 
		
		```
		2 is greater than 3 ? false
		6 is greater than 6 ? false
		3.14 is greater than 3.14 ? false
		0.30000000000000004 is greater than 0.3 ? true
		a is greater than b ? false
		a is greater than a ? false
		z is greater than a ? true
		
		 is greater than
		 ? false
		foo is greater than foo ? false
		bar is greater than bar ? false
		[] is greater than [] ? true
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2
		isize 3
		Boolean false
		isize 6
		isize 6
		Boolean false
		f32 3.14
		f32 3.14
		Boolean false
		f64 0.30000000000000004
		f64 0.3
		Boolean true
		Char 'a'
		Char 'b'
		Boolean false
		Char 'a'
		Char 'a'
		Boolean false
		Char 'z'
		Char 'a'
		Boolean true
		Char '\n'
		Char '\n'
		Boolean false
		StringBox 64
		StringBox 64
		Boolean false
		StringBox 71
		StringBox 72
		Boolean false
		ListBox 80
		ListBox 79
		Boolean true
		--------------------------------
		STACK LENGTH: 33
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `<` 
	- Performance: O(1)
	- Given two non-NULLBox items on the stack, pops both of them and compares the former second-to-top with the former top of the stack and determines if the second-to-top is less than the top, pushing a Boolean to the stack to reflect the results. 
	- General form: 
		- Stack `x y` where both `x` and `y` are matching types of type `t` where `t` can be any type that isn't NULLBox. After applying `<` stack `x y` becomes stack `b` where `b` is a Boolean that states whether or not `x` is less than `y`.
	- The following example code is advanced but what matters is the comparison itself and the outcomes.
	- Example Program:

		```
		//Takes in two operands and checks 
		// to see if the second-to-top item 
		// is less than the top item, 
		// printing the result in a nice formatted print statement 
		// and pushing the original operands 
		// and the comparison result 
		// to the stack for the later stack print.
		//This function makes no effort to free memory used 
		// but it's fine since it's a small program overall.
		func def compareAndPrint
			loc mak y ;
			loc mak x ;
		
			//Compares two inputs and saves result.
			loc get x ; loc get y ; <
			loc mak b ;
		
			//Fetches the inputs, converts them to strings,
			// puts them into a new string that gets formatted
			// with output of comparison and prints is as one
			// big old string.
			""
			loc get x ; "String" cast ++ 
			" is less than " ++
			loc get y ; "String" cast ++
			' ' push '?' push ' ' push
			loc get b ; "String" cast ++
			printLine
		
			//Puts original values back on stack, 
			// as well as comparison.
			loc get x ; 
			loc get y ;
			loc get b ;
		;
		
		2 3 func call compareAndPrint ;
		6 6 func call compareAndPrint ;
		3.14 3.14 func call compareAndPrint ;
		
		//This will be false since floats use binary.
		0.1f64 0.2f64 + 0.3f64 func call compareAndPrint ;
		
		'a' 'b' func call compareAndPrint ;
		'a' 'a' func call compareAndPrint ;
		'z' 'a' func call compareAndPrint ;
		'\n' '
		' func call compareAndPrint ;
		
		//False since it's the same box.
		"foo" dup func call compareAndPrint ;
		
		//True because first "bar" is allocated earlier, 
		// making its box number lower and thus less than.
		"bar" "bar" func call compareAndPrint ; 
		
		//False because the second ListBox was swapped, 
		// making it not less than the top.
		[] deepDup swap func call compareAndPrint ;
		
		debugPrintStack
		
		```

	- Resulting Output: 
		
		```
		2 is less than 3 ? true
		6 is less than 6 ? false
		3.14 is less than 3.14 ? false
		0.30000000000000004 is less than 0.3 ? false
		a is less than b ? true
		a is less than a ? false
		z is less than a ? false
		
		 is less than
		 ? false
		foo is less than foo ? false
		bar is less than bar ? true
		[] is less than [] ? false
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2
		isize 3
		Boolean true
		isize 6
		isize 6
		Boolean false
		f32 3.14
		f32 3.14
		Boolean false
		f64 0.30000000000000004
		f64 0.3
		Boolean false
		Char 'a'
		Char 'b'
		Boolean true
		Char 'a'
		Char 'a'
		Boolean false
		Char 'z'
		Char 'a'
		Boolean false
		Char '\n'
		Char '\n'
		Boolean false
		StringBox 64
		StringBox 64
		Boolean false
		StringBox 71
		StringBox 72
		Boolean true
		ListBox 80
		ListBox 79
		Boolean false
		--------------------------------
		STACK LENGTH: 33
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `>=` 
	- Performance: O(1)
	- Given two non-NULLBox items on the stack, pops both of them and compares the former second-to-top with the former top of the stack and determines if the second-to-top is greater than or equal to the top, pushing a Boolean to the stack to reflect the results. 
	- General form: 
		- Stack `x y` where both `x` and `y` are matching types of type `t` where `t` can be any type that isn't NULLBox. After applying `>=` stack `x y` becomes stack `b` where `b` is a Boolean that states whether or not `x` is greater than or equal to `y`.
	- Example Program:

		```
		//Takes in two operands and checks 
		// to see if the second-to-top item 
		// is greater than or equal to the top item, 
		// printing the result in a nice formatted print statement 
		// and pushing the original operands 
		// and the comparison result 
		// to the stack for the later stack print.
		//This function makes no effort to free memory used 
		// but it's fine since it's a small program overall.
		func def compareAndPrint
			loc mak y ;
			loc mak x ;
		
			//Compares two inputs and saves result.
			loc get x ; loc get y ; >=
			loc mak b ;
		
			//Fetches the inputs, converts them to strings,
			// puts them into a new string that gets formatted
			// with output of comparison and prints is as one
			// big old string.
			""
			loc get x ; "String" cast ++ 
			" is greater than or equal to " ++
			loc get y ; "String" cast ++
			' ' push '?' push ' ' push
			loc get b ; "String" cast ++
			printLine
		
			//Puts original values back on stack, 
			// as well as comparison.
			loc get x ; 
			loc get y ;
			loc get b ;
		;
		
		2 3 func call compareAndPrint ;
		6 6 func call compareAndPrint ;
		3.14 3.14 func call compareAndPrint ;
		
		//This will be true since floats use binary.
		0.1f64 0.2f64 + 0.3f64 func call compareAndPrint ;
		
		'a' 'b' func call compareAndPrint ;
		'a' 'a' func call compareAndPrint ;
		'z' 'a' func call compareAndPrint ;
		'\n' '
		' func call compareAndPrint ;
		
		//True since it's the same box.
		"foo" dup func call compareAndPrint ;
		
		//False because first "bar" is allocated earlier, 
		// making its box number lower and thus not greater.
		"bar" "bar" func call compareAndPrint ; 
		
		//True because the second ListBox was swapped, 
		// making it greater than the top since it's at the top.
		[] deepDup swap func call compareAndPrint ;
		
		debugPrintStack
		
		```

	- Resulting Output: 
		
		```
		2 is greater than or equal to 3 ? false
		6 is greater than or equal to 6 ? true
		3.14 is greater than or equal to 3.14 ? true
		0.30000000000000004 is greater than or equal to 0.3 ? true
		a is greater than or equal to b ? false
		a is greater than or equal to a ? true
		z is greater than or equal to a ? true
		
		 is greater than or equal to
		 ? true
		foo is greater than or equal to foo ? true
		bar is greater than or equal to bar ? false
		[] is greater than or equal to [] ? true
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2
		isize 3
		Boolean false
		isize 6
		isize 6
		Boolean true
		f32 3.14
		f32 3.14
		Boolean true
		f64 0.30000000000000004
		f64 0.3
		Boolean true
		Char 'a'
		Char 'b'
		Boolean false
		Char 'a'
		Char 'a'
		Boolean true
		Char 'z'
		Char 'a'
		Boolean true
		Char '\n'
		Char '\n'
		Boolean true
		StringBox 64
		StringBox 64
		Boolean true
		StringBox 71
		StringBox 72
		Boolean false
		ListBox 80
		ListBox 79
		Boolean true
		--------------------------------
		STACK LENGTH: 33
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `<=` 
	- Performance: O(1)
	- Given two non-NULLBox items on the stack, pops both of them and compares the former second-to-top with the former top of the stack and determines if the second-to-top is less than or equal to the top, pushing a Boolean to the stack to reflect the results. 
	- General form: 
		- Stack `x y` where both `x` and `y` are matching types of type `t` where `t` can be any type that isn't NULLBox. After applying `<=` stack `x y` becomes stack `b` where `b` is a Boolean that states whether or not `x` is less than or equal to `y`.
	- Example Program:

		```
		//Takes in two operands and checks 
		// to see if the second-to-top item 
		// is less than or equal to the top item, 
		// printing the result in a nice formatted print statement 
		// and pushing the original operands 
		// and the comparison result 
		// to the stack for the later stack print.
		//This function makes no effort to free memory used 
		// but it's fine since it's a small program overall.
		func def compareAndPrint
			loc mak y ;
			loc mak x ;
		
			//Compares two inputs and saves result.
			loc get x ; loc get y ; <=
			loc mak b ;
		
			//Fetches the inputs, converts them to strings,
			// puts them into a new string that gets formatted
			// with output of comparison and prints is as one
			// big old string.
			""
			loc get x ; "String" cast ++ 
			" is less than or equal to " ++
			loc get y ; "String" cast ++
			' ' push '?' push ' ' push
			loc get b ; "String" cast ++
			printLine
		
			//Puts original values back on stack, 
			// as well as comparison.
			loc get x ; 
			loc get y ;
			loc get b ;
		;
		
		2 3 func call compareAndPrint ;
		6 6 func call compareAndPrint ;
		3.14 3.14 func call compareAndPrint ;
		
		0.1f64 0.2f64 + 0.3f64 func call compareAndPrint ;
		
		'a' 'b' func call compareAndPrint ;
		'a' 'a' func call compareAndPrint ;
		'z' 'a' func call compareAndPrint ;
		'\n' '
		' func call compareAndPrint ;
		
		//True since it's the same box.
		"foo" dup func call compareAndPrint ;
		
		//True because first "bar" is allocated earlier, 
		// making its box number lower and thus less than.
		"bar" "bar" func call compareAndPrint ; 
		
		//False because the second ListBox was swapped, 
		// making it not less than the top.
		[] deepDup swap func call compareAndPrint ;
		
		debugPrintStack
		
		```

	- Resulting Output: 
		
		```
		2 is less than or equal to 3 ? true
		6 is less than or equal to 6 ? true
		3.14 is less than or equal to 3.14 ? true
		0.30000000000000004 is less than or equal to 0.3 ? false
		a is less than or equal to b ? true
		a is less than or equal to a ? true
		z is less than or equal to a ? false
		
		 is less than or equal to
		 ? true
		foo is less than or equal to foo ? true
		bar is less than or equal to bar ? true
		[] is less than or equal to [] ? false
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2
		isize 3
		Boolean true
		isize 6
		isize 6
		Boolean true
		f32 3.14
		f32 3.14
		Boolean true
		f64 0.30000000000000004
		f64 0.3
		Boolean false
		Char 'a'
		Char 'b'
		Boolean true
		Char 'a'
		Char 'a'
		Boolean true
		Char 'z'
		Char 'a'
		Boolean false
		Char '\n'
		Char '\n'
		Boolean true
		StringBox 64
		StringBox 64
		Boolean true
		StringBox 71
		StringBox 72
		Boolean true
		ListBox 80
		ListBox 79
		Boolean false
		--------------------------------
		STACK LENGTH: 33
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `stringCompare`
	- Performance:
		- O(n)
		- Where n is the length of the shorter String.
	- Given a stack with two valid `StringBox`es, `stringCompare` pops the two `StringBox`es and compares their contents to each other, pushing an isize to the stack based on the result. 
		- If the second-to-top string is less than the top String, a -1 is pushed.
		- If the Strings are equal, a 0 is pushed.
		- If the second-to-top String is greater than the top String, a 1 is pushed.
	- General form: stack `x y` where `x` and `y` are type StringBox, `stringCompare` yields stack `c` where `c` is an isize that represents the comparison result.
	- The result from `stringCompare` can be used in other comparison operators to determine if Strings are equal or other.
	- The example code shows instances of branching but also demonstrates how `stringCompare` works.
	- Example Program:

		```
		//Takes two StringBoxes, 
		// displays a debug output to see 
		// which one is bigger or not, 
		// then puts it all back on stack, 
		// including the comparisons.
		//This function will be careful about extra memory.
		func def strCmpPrint
			loc mak str1 ;
			loc mak str2 ;
		
			loc get str1 ; loc get str2 ;
			stringCompare loc mak cmp ;
		
			loc get cmp ; 0 >
			if
				loc get str1 ; print
				" is bigger than " dup print box free ;
				loc get str2 ; print 
				'!' printChar '\n' printChar
			;
		
			loc get cmp ; 0 <
			if
				loc get str1 ; print
				" is smaller than " dup print box free ;
				loc get str2 ; print 
				'!' printChar '\n' printChar
			;
		
			loc get cmp ; 0 ==
			if
				loc get str1 ; print
				" is equal to " dup print box free ;
				loc get str2 ; print 
				'!' printChar '\n' printChar
			;
		
			loc get str1 ;
			loc get str2 ;
			loc get cmp ;
		;
		
		"foo" "bar" func call strCmpPrint ;
		
		"foo" "foo" func call strCmpPrint ;
		
		"bar" "foo" func call strCmpPrint ;
		
		debugPrintStack
		
		```

	- Resulting Output: 

		```
		bar is smaller than foo!
		foo is equal to foo!
		foo is bigger than bar!
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 1
		StringBox 0
		isize -1
		StringBox 3
		StringBox 2
		isize 0
		StringBox 5
		StringBox 4
		isize 1
		--------------------------------
		STACK LENGTH: 9
		--------------------------------
		END STACK PRINT
		--------------------------------
		```
	

### <a name = "logic-ops"></a>3.5 Logical Operators
#### [**Return to Table of Contents**](#toc)
Logical operators perform operations of Boolean logic. These operators are useful to determine how if statements branch and how loops run. They have other uses too. Anything in Lmao that involves Boolean logic is covered by these operators.

These are the operators:
- `and` or `&&`
	- Performance: O(1)
	- Pops two booleans from the stack, performs a logical AND on them, and pushes a resulting boolean based on the result.
	- General form: Given stack `x` `y` where `x` and `y` are both type `t` where `t` is type `Boolean`,  applying `and` results in stack `z` where `z` is the `Boolean` result of the operation.
	- The logical AND itself is an expression where the result is only `true` if both items are also `true`, otherwise the result is `false`.
	- Example Program:
	
		```
		false false and
		false true and
		true false and
		true true and
		
		//Can also use && instead of and
		false false &&
		false true &&
		true false &&
		true true &&
		
		//Final output
		debugPrintStack
		```
	
	- Resulting Output:
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean false
		Boolean false
		Boolean false
		Boolean true
		Boolean false
		Boolean false
		Boolean false
		Boolean true
		--------------------------------
		STACK LENGTH: 8
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `or` or `||`
	- Performance: O(1)
	- Pops two `Boolean`s from the stack, performs a logical OR on them, and pushes a resulting `Boolean` based on the result.
	- General form: Given stack `x` `y` where `x` and `y` are both type `t` where `t` is type `Boolean`,  applying `or` results in stack `z` where `z` is the `Boolean` result of the operation.
	- The logical OR works in such a way that it's `true` as long as at least one of the items is `true`, and is `false` if both are `false`.
	- Example Program:
	
		```
		false false or
		false true or
		true false or
		true true or
		
		//Can also use || instead of or
		false false ||
		false true ||
		true false ||
		true true ||
		
		//Displays stack for example output.
		debugPrintStack
		```
	
	- Resulting Output:
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean false
		Boolean true
		Boolean true
		Boolean true
		Boolean false
		Boolean true
		Boolean true
		Boolean true
		--------------------------------
		STACK LENGTH: 8
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `xor`
	- Performance: O(1)
	- Pops two `Boolean`s from the stack, performs a logical XOR (e**X**clusive **OR**) on them, and pushes a resulting `Boolean` based on the result.
	- General form: Given stack `x` `y` where `x` and `y` are both type `t` where `t` is type `Boolean`,  applying `xor` results in stack `z` where `z` is the `Boolean` result of the operation.
	- Logical XOR is `true` when exactly one item is `true` and one item is `false`, order irrelevant. This means that if both values are `true` or both values are `false`, the result is false. This makes it a more exclusive or than the regular or, hence the name.
	- It's also possible to just use the `!=` operator between the two items on the stack and get the same result, but having a formal `xor` operator is nice and improves readability. 
	- Example Program:
	
		```
		false false xor
		false true xor
		true false xor
		true true xor
		
		//Displays stack for example output.
		debugPrintStack
		```
	
	- Resulting Output:
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean false
		Boolean true
		Boolean true
		Boolean false
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `not` or `!`
	- Performance: O(1)
	- Pops one `Boolean` from the stack and performs a logical NOT operation on it, pushing the resulting inverted `Boolean` to the stack.
	- General form: Given stack `x` where `x` is type `Boolean`, applying `not` results in stack `y` where `y` is a `Boolean` with the opposite value.
	- Logical NOT simply flips the value of the `Boolean` to the opposing value.
	- Example Program:
	
		```
		true not
		false not
		
		//Can also use ! instead of not
		true !
		false !
		
		//Displays stack for example output.
		debugPrintStack
		```
	
	- Resulting Output:
	
		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean false
		Boolean true
		Boolean false
		Boolean true
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

### <a name = "ls-str-ops"></a>3.6 List & String Operators
#### [**Return to Table of Contents**](#toc)
These operators are used to operate on Lists and Strings held by `ListBox`es and `StringBox`es respectively. These operators are how you mutate the underlying data held by these boxes and how you do some pretty neat stuff.

These are all of the operators associated with this category:
- `push` or `p`
	- Performance:
		- O(1) average case
		- O(n) worse case during resizes
	- For Lists:
		- With the second-to-top of the stack being a ListBox and the top of the stack being any other value, the two values are consumed and the top value is pushed to the end of the List, mutating the contents. The ListBox holding the mutated List is then pushed back to the stack for more potential mutation.
		- General form: given stack `l` `v` where `l` is of type `ListBox` and `v` is any data type, applying `push` results in stack `l'` where `l'` is a `ListBox` with the same box number as before but now holding a `List` that has `v` added to the end.
		- Since Lists can hold any mixture of data types, it doesn't matter what value is involved when using `push`
	- For Strings:
		- With the second-to-top of the stack being a StringBox and the top of the stack being a Char, consumes both the StringBox and Char, pushes the Char to the end of the String held by the StringBox, mutating it, and pushes the StringBox back to the stack where it holds the mutated String.
		- General form: given stack `s` `c` where `s` is type `StringBox`and `c` is type `Char`, applying `push` results in stack `s'` where `s'` is a `StringBox` with the same box number as `s` but holds a mutated string with `c` added to the end of it.
		- Unlike for Lists, the item being pushed to the end of a String must be a Char, since a String is a list of Chars.
	- Example Program:

		```
		//List actions.
		[]
		debugPrintStack
		debugPrintHeap
		1 push 
		debugPrintStack
		debugPrintHeap
		//The operator p can also be used as an alias for push
		// This is easier when building lists 
		// instead of having to type push over and over.
		2u8 p 
		debugPrintHeap
		3.14 push
		debugPrintHeap
		
		//String Actions.
		"The first three alphabet letters are: "
		debugPrintStack
		debugPrintHeap
		'A' p
		debugPrintStack
		debugPrintHeap
		'B' p
		debugPrintHeap
		'C' p
		debugPrintHeap
		debugPrintStack
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, u8 2]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, u8 2, f32 3.14]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, u8 2, f32 3.14]
		StringBox 1:
		        String "The first three alphabet letters are: "
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, u8 2, f32 3.14]
		StringBox 1:
		        String "The first three alphabet letters are: A"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, u8 2, f32 3.14]
		StringBox 1:
		        String "The first three alphabet letters are: AB"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, u8 2, f32 3.14]
		StringBox 1:
		        String "The first three alphabet letters are: ABC"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `pop` or `po`
	- Performance: O(1)
	- For Lists:
		- Given a valid non-empty ListBox on the stack, consumes it, pops the last item from the list, and pushes the ListBox and the popped item to the stack, with the ListBox holding a List that's one item smaller.
		- General form: given stack `l` where `l` is type `ListBox` and is not empty, applying `pop` results in stack `l' v` where `l'` is a `ListBox` with the same box number as `l` but with mutated contents and `v` is the item popped from the end of the List with whatever data type it has.
	- For Strings:
		- Given a valid non-empty StringBox on the stack, consumes it, pops the last Char from the String, and pushes the StringBox holding the mutated String and the Char that was popped to the stack.
		- General form: given stack `s` where `s` is type `StringBox` and is not empty, applying `pop` results in stack `s'` `c` where `s'` is a `StringBox` with the same box number as `s` but holds the mutated `String`, and `c` is the `Char` popped from the end of the `String`.
	- Be aware that an error will be thrown for both `ListBox`es and `StringBox`es if you try to use `pop` when they're empty, since there's nothing to pop from the List or String. It's just like how `drop` needs something on the stack to drop.
	- Example Program for Lists:

		```
		//Builds initial List with stuff to pop from the end.
		[] 1 p 2 p 3 p 666 p 42 p
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 42 from List.
		pop swap 
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 666 from List.
		pop swap
		debugPrintStack
		debugPrintHeap
		
		```

	- Resulting Output of List Program:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 666, isize 42]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 42
		ListBox 0
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 666]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 42
		isize 666
		ListBox 0
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```
	
	- Example Program for Strings:

		```
		//Created basic hello world string 
		// with extra stuff at the end that must leave. 
		"Hello, world!42😂"
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of emoji.
		pop swap
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 2.
		pop swap
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 4.
		pop swap
		debugPrintStack
		debugPrintHeap
		```

	- Resulting Output of String Program:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "Hello, world!42😂"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char '\u{1f602}'
		StringBox 0
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "Hello, world!42"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char '\u{1f602}'
		Char '2'
		StringBox 0
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "Hello, world!4"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char '\u{1f602}'
		Char '2'
		Char '4'
		StringBox 0
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "Hello, world!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `fpush` or `fp`
	- Performance: O(n)
	- For Lists:
		- With the second-to-top of the stack being a ListBox and the top of the stack being any other value, the two values are consumed and the top value is pushed to the front of the List, mutating the contents. The ListBox holding the mutated List is then pushed back to the stack for more potential mutation.
		- General form: given stack `l` `v` where `l` is of type `ListBox` and `v` is any data type, applying `fpush` results in stack `l'` where `l'` is a `ListBox` with the same box number as before but now holding a `List` that has `v` added to the front.
		- Since Lists can hold any mixture of data types, it doesn't matter what value is involved when using `fpush`
	- For Strings:
		- With the second-to-top of the stack being a StringBox and the top of the stack being a Char, consumes both the StringBox and Char, pushes the Char to the front of the String held by the StringBox, mutating it, and pushes the StringBox back to the stack where it holds the mutated String.
		- General form: given stack `s` `c` where `s` is type `StringBox`and `c` is type `Char`, applying `push` results in stack `s'` where `s'` is a `StringBox` with the same box number as `s` but holds a mutated string with `c` added to the front of it.
		- Unlike for Lists, the item being pushed to the front of a String must be a Char, since a String is a list of Chars.
	- Be aware that pushing to the front costs linear time complexity since every subsequent item in the List or String must be shifted forward by one, which is a linear operation. It's often more efficient to build a list backwards using `push` and then reverse it by popping from it and pushing to a new List. 
	- The following example program is the same as for `push` but it can be seen in the output how the List built is reversed to the previous example since items are pushed to the front instead of the back.
	- Example Program:

		```
		//List actions.
		[]
		debugPrintStack
		debugPrintHeap
		1 fpush 
		debugPrintStack
		debugPrintHeap
		//The operator p can also be used as an alias for push
		// This is easier when building lists 
		// instead of having to type push over and over.
		2u8 fp 
		debugPrintHeap
		3.14 fpush
		debugPrintHeap
		
		//String Actions.
		" Are the first three letters of the alphabet."
		debugPrintStack
		debugPrintHeap
		'C' fp
		debugPrintStack
		debugPrintHeap
		'B' fp
		debugPrintHeap
		'A' fp
		debugPrintHeap
		debugPrintStack
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [u8 2, isize 1]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [f32 3.14, u8 2, isize 1]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [f32 3.14, u8 2, isize 1]
		StringBox 1:
		        String " Are the first three letters of the alphabet."
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [f32 3.14, u8 2, isize 1]
		StringBox 1:
		        String "C Are the first three letters of the alphabet."
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [f32 3.14, u8 2, isize 1]
		StringBox 1:
		        String "BC Are the first three letters of the alphabet."
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [f32 3.14, u8 2, isize 1]
		StringBox 1:
		        String "ABC Are the first three letters of the alphabet."
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `fpop` or `fpo`
	- Performance: O(n)
	- For Lists:
		- Given a valid non-empty ListBox on the stack, consumes it, pops the first item from the list, and pushes the ListBox and the popped item to the stack, with the ListBox holding a List that's one item smaller.
		- General form: given stack `l` where `l` is type `ListBox` and is not empty, applying `fpop` results in stack `l' v` where `l'` is a `ListBox` with the same box number as `l` but with mutated contents and `v` is the item popped from the front of the List with whatever data type it has.
	- For Strings:
		- Given a valid non-empty StringBox on the stack, consumes it, pops the first Char from the String, and pushes the StringBox holding the mutated String and the Char that was popped to the stack.
		- General form: given stack `s` where `s` is type `StringBox` and is not empty, applying `pop` results in stack `s'` `c` where `s'` is a `StringBox` with the same box number as `s` but holds the mutated `String`, and `c` is the `Char` popped from the front of the `String`.
	- Be aware that an error will be thrown for both `ListBox`es and `StringBox`es if you try to use `fpop` when they're empty, since there's nothing to pop from the List or String. It's just like how `drop` needs something on the stack to drop.
	- Example Program for Lists:

		```
		//Builds initial List with stuff to pop from the front.
		[] 'A' p 6.9 p 1 p 2 p 3 p 
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 'A' from List.
		fpop swap 
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 6.9 from List.
		fpop swap
		debugPrintStack
		debugPrintHeap
		
		```

	- Resulting Output of List Program:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [Char 'A', f32 6.9, isize 1, isize 2, isize 3]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'A'
		ListBox 0
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [f32 6.9, isize 1, isize 2, isize 3]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'A'
		f32 6.9
		ListBox 0
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```
	
	- Example Program for Strings:

		```
		//Created basic hello world string 
		// with extra stuff at the front that must leave. 
		"42😂Hello, world!"
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 4.
		fpop swap
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of 2.
		fpop swap
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of emoji.
		fpop swap
		debugPrintStack
		debugPrintHeap
		```

	- Resulting Output of String Program:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "42😂Hello, world!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char '4'
		StringBox 0
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "2😂Hello, world!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char '4'
		Char '2'
		StringBox 0
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "😂Hello, world!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char '4'
		Char '2'
		Char '\u{1f602}'
		StringBox 0
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "Hello, world!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `index`
	- Performance: 
		- Lists: O(1)
		- Strings: O(n)
	- Lists:
		- Given a stack where the second-to-top element is a valid ListBox and the top is a usize, pops both items from the stack, uses the usize to index into the List held by the ListBox, and pushes the item held at the index to the stack.
		- General Form: Given stack `l` `i` where `l` is type `ListBox` with at least one item in it and `i` is type `usize` and is a value smaller than the length of `l`, applying `index` yields stack `v` where `v` is a value of any data type held in the `List` held by `ListBox` `l` at position `i`. 
	- Strings:
		- Given a stack where the second-to-top element is a valid StringBox and the top is a usize, pops both items from the stack, uses the usize to index into the String held by the StringBox, and pushes the Char held at the usize index to the stack.
		- General Form: Given stack `s` `i` where `s` is type `StringBox` with at least one `Char` in it and `i` is type `usize` and is a value smaller than the length of `s`, applying `index` yields stack `c` where `c` is a `Char` held in the `String` held by `StringBox` `s` at position `i`. 
	- **Be aware!** The index must be within the value range 0 to length of the List/String inclusive, or else an error is thrown.
	- The performance difference between Lists and Strings is due to the fact that a List just directly indexes at the given index, which is constant time, while the String needs to travel to that Char using an iterator approach, since the String is a Vec of u8 under the hood. 
	- The index itself is a usize instead of an isize, to ensure that it is at least a non-negative integer. 
	- Example Program:

		```
		//LIST PART
		
		//Builds example list and shows debug output 
		// of it on stack and in heap.
		[] 1 p 2 p 3 p 5040 p 2319 p 666 p 42 p
		debugPrintStack
		debugPrintHeap
		
		//Indexes to List at position 3.
		// This does cause the ListBox 
		// to be lost but it's fine for this little example.
		// To save it, you'd want to use 
		// the dup operator before 
		// then pushing the usize to index.
		3usize debugPrintStack index
		debugPrintStack
		
		//STRING PART
		
		//Creates example String.
		"This is a sentence!"
		debugPrintStack
		debugPrintHeap
		
		//Indexes into string at position 4, 
		// showing space Char support. 
		//The memory is lost here too but again 
		// it's just a little example so who cares.
		4usize debugPrintStack index
		debugPrintStack
		
		```
	
	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 5040, isize 2319, isize 666, isize 42]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		usize 3
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 5040
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 5040
		StringBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 5040, isize 2319, isize 666, isize 42]
		StringBox 1:
		        String "This is a sentence!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 5040
		StringBox 1
		usize 4
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 5040
		Char ' '
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `length` or `len`
	- Performance: O(1)
	- Given a stack with the top being either a valid ListBox or a valid StringBox, consumes the ListBox or StringBox and pushes a usize to the stack representing the length of the List/String held by the ListBox/StringBox. 
	- General form: given a stack 'v' where `v` is either a valid `ListBox` or a valid `StringBox`, applying `length` results in stack `l` where `l` is a `usize` representing the number of items/`Char`s in `v`.
	- Example Program:

		```
		func def dispCollection
			loc mak i ;
			//Defer explained later!
			defer loc get i ; box free ; ;
			
			loc get i ; 
			debugPrintStack
			debugPrintHeap
		
			//Can also use len here!
			length
			debugPrintStack
		
			drop
		;
		
		//Lists!
		
		"\t\t\tLIST OUTPUT:" dup printLine box free ;
		
		//Empty List case.
		[] func call dispCollection ;
		
		//Non-empty List case.
		[] 'x' p func call dispCollection ;
		
		//Multi-item List case.
		[] 1 p 2 p 3 p func call dispCollection ;
		
		//Strings!
		
		"\t\t\tSTRING OUTPUT:" dup printLine box free ;
		
		//Empty String case.
		"" func call dispCollection ;
		
		//Single-Char String case.
		"A" func call dispCollection ;
		
		//Multi-Char String case.
		"This is a longer String!" func call dispCollection ;
		
		//This is printed to show 
		// that the heap is totally free'd!
		debugPrintHeap
		
		```

	- Resulting Output:

		```
		                        LIST OUTPUT:
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [Char 'x']
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 1
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 3
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		                        STRING OUTPUT:
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String ""
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "A"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 1
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "This is a longer String!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 24
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0 [FREE]:
		        String "This is a longer String!"
		////////////////////////////////
		FREE'D BOX NUMBERS: [0]
		////////////////////////////////
		FREE'D BOX COUNT: 1
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 100.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `isEmpty`
	- Performance: O(1)
	- Given a stack with the top being either a valid ListBox or a valid StringBox, consumes the ListBox or StringBox and pushes a Boolean that indicates whether or not the List/String contained within is empty.
	- General form: given a stack 'v' where `v` is either a valid `ListBox` or a valid `StringBox`, applying `isEmpty` results in stack `b` where `b` is a `Boolean` indicating whether or not the `List`/`String` contained by the `ListBox`/`StringBox` is empty.
	- Example Program:

		```
		//Lists!
		[] dup isEmpty
		[] 3.14 push dup isEmpty
		
		//Strings!
		"" dup isEmpty
		"" 'A' p 'B' p 'C' p dup isEmpty
		
		debugPrintStack
		debugPrintHeap
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		Boolean true
		ListBox 1
		Boolean false
		StringBox 2
		Boolean true
		StringBox 3
		Boolean false
		--------------------------------
		STACK LENGTH: 8
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		ListBox 1:
		        List [f32 3.14]
		StringBox 2:
		        String ""
		StringBox 3:
		        String "ABC"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 4
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `clear`
	- Performance: O(1)
	- Given a stack where the top item is a valid ListBox or valid StringBox, pops the item from the stack, clears its contents, and pushes it back to the stack.
	- General Form: given stack `v` where `v` is a valid box of type `ListBox` or `StringBox`, applying `clear` yields stack `v'` where `v'` has the same box number as `v` but holds the updated `List`/`String` with the contents cleared.
	- This operates in constant time because Lists only hold primitives, meaning the underlying Vec can be cleared by its length being set to zero, without needing to iterate through to clear memory, same for Strings which have a Vec of u8 as the underlying data structure.
	- Example Program:

		```
		//List created and stuff put into it.
		[] "wan" p "tu" p "mute" p
		debugPrintStack
		debugPrintHeap
		
		//Clears list, causing the Strings 
		// held by the StringBoxes to be lost 
		// but it's fine because it's an example 
		// and the heap gets cleaned up at the end anyway.
		clear
		debugPrintStack
		debugPrintHeap
		
		//Creates a string with stuff in it to be cleared.
		"Please don't clear me! I want to live!"
		debugPrintStack
		debugPrintHeap
		
		//Clears the string; no memory lost here 
		// since no boxes are contained in a String.
		clear
		debugPrintStack
		debugPrintHeap
		
		```
	
	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [StringBox 1, StringBox 2, StringBox 3]
		StringBox 1:
		        String "wan"
		StringBox 2:
		        String "tu"
		StringBox 3:
		        String "mute"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 4
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		StringBox 1:
		        String "wan"
		StringBox 2:
		        String "tu"
		StringBox 3:
		        String "mute"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 4
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 4
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		StringBox 1:
		        String "wan"
		StringBox 2:
		        String "tu"
		StringBox 3:
		        String "mute"
		StringBox 4:
		        String "Please don't clear me! I want to live!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 5
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 4
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		StringBox 1:
		        String "wan"
		StringBox 2:
		        String "tu"
		StringBox 3:
		        String "mute"
		StringBox 4:
		        String ""
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 5
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `changeItemAt`
	- Performance: O(1)
	- Given a stack with the third-to-top item being a valid ListBox, the second-to-top item being a usize, and the top item being any data type, pops the three items and alters the value held at the position indicated by usize to the new value, provided the usize index points to a valid index in the List held by the ListBox, pushing the ListBox back to the stack with the mutated List held by it.
	- General Form: given stack `l` `i` `v` where `l` is a valid `ListBox`, `i` is type `usize` and is smaller than the length of `l`, and `v` is any data type, applying `changeItemAt` produces stack `l'` where `l'` holds the same box number as `l` but contains a new `List` that has position `i` updated to `v`.
	- Example Program:

		```
		//Creates a List where the third position is wrong 
		// and must be fixed.
		[] 1 p 2 p 5 p 4 p 5 p 6 p
		debugPrintStack
		debugPrintHeap
		
		//Fixes value to the correct one.
		2usize 3 changeItemAt
		debugPrintStack
		debugPrintHeap
		
		```
	
	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 5, isize 4, isize 5, isize 6]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 4, isize 5, isize 6]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `++`
	- Performance: O(n) where `n` is the length of the List/String at the top of the stack.
	- Given a stack with two matching valid ListBoxes or StringBoxes, consumes both of them, copies the contents of the top List/String to the second-to-top List/String, and pushes the StringBox/ListBox holding the mutated collection to the stack.
	- General form: given stack `x y` where `x` and `y` are type `t` which is either type `ListBox` or `StringBox` and is valid, applying `++` results in stack `x'` where `x` is type `t` and holds a mutated collection that is the result of appending the contents of `y`'s collection to `x`'s collection.
	- Be aware that `y`'s contents aren't freed, merely copied and appended to `x`. If you don't need `y` anymore, be sure to have it saved elsewhere before using `++` and then free `y` afterwards.
	- Example Program:

		```
		//Initial list.
		[] 1 p 2 p 3 p
		//List to add to initial.
		[] 4 p 5 p 6 p 
		
		debugPrintStack
		debugPrintHeap
		++
		debugPrintStack
		debugPrintHeap
		
		//String version of example.
		"My name is " 
		"Egarakth the Destroyer."
		
		debugPrintStack
		debugPrintHeap
		++
		debugPrintStack
		debugPrintHeap
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		ListBox 1
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3]
		ListBox 1:
		        List [isize 4, isize 5, isize 6]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 4, isize 5, isize 6]
		ListBox 1:
		        List [isize 4, isize 5, isize 6]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 2
		StringBox 3
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 4, isize 5, isize 6]
		ListBox 1:
		        List [isize 4, isize 5, isize 6]
		StringBox 2:
		        String "My name is "
		StringBox 3:
		        String "Egarakth the Destroyer."
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 4
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		StringBox 2
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List [isize 1, isize 2, isize 3, isize 4, isize 5, isize 6]
		ListBox 1:
		        List [isize 4, isize 5, isize 6]
		StringBox 2:
		        String "My name is Egarakth the Destroyer."
		StringBox 3:
		        String "Egarakth the Destroyer."
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 4
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

### <a name = "char-ops"></a>3.7 Character Operators
#### [**Return to Table of Contents**](#toc)
These three operators are quite simple. They are used to operate on the Char data type for useful checks to occur.

These are the operators below:
- `isWhitespaceChar`
	- Performance: O(1)
	- Given a stack where the top is of type Char, consumes the Char and pushes a Boolean to the stack based on whether the Char is a white space character like `\n`, ` `, `\t`, etc.
	- General form: given stack `c` where `c` is type `Char`, applying `isWhitespaceChar` results in stack `b` where `b` is type `Boolean` and holds a value based on whether `c` is a white space character or not.
	- Example Program:

		```
		' ' isWhitespaceChar
		'\t' isWhitespaceChar
		'A' isWhitespaceChar
		
		debugPrintStack
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean true
		Boolean true
		Boolean false
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `isAlphaChar`
	- Performance: O(1)
	- Given a stack where the top is of type Char, consumes the Char and pushes a Boolean to the stack based on whether the Char is an English alphabetical character like `A`, `b`, `C`, etc.
	- General form: given stack `c` where `c` is type `Char`, applying `isAlphaChar` results in stack `b` where `b` is type `Boolean` and holds a value based on whether `c` is an English alphabetical character or not.
	- Example Program:

		```
		'A' isAlphaChar
		'B' isAlphaChar
		'C' isAlphaChar
		'd' isAlphaChar
		'e' isAlphaChar
		'f' isAlphaChar
		'9' isAlphaChar
		' ' isAlphaChar
		'😂' isAlphaChar
		
		debugPrintStack
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean true
		Boolean true
		Boolean true
		Boolean true
		Boolean true
		Boolean true
		Boolean false
		Boolean false
		Boolean false
		--------------------------------
		STACK LENGTH: 9
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `isNumChar`
	- Performance: O(1)
	- Given a stack where the top is of type Char, consumes the Char and pushes a Boolean to the stack based on whether the Char is a numerical character like `0`, `1`, `2`, etc.
	- General form: given stack `c` where `c` is type `Char`, applying `isNumChar` results in stack `b` where `b` is type `Boolean` and holds a value based on whether `c` is a numerical character or not.
	- Example Program:

		```
		'0' isNumChar
		'1' isNumChar
		'2' isNumChar 
		'A' isNumChar
		'B' isNumChar
		' ' isNumChar
		
		debugPrintStack
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean true
		Boolean true
		Boolean true
		Boolean false
		Boolean false
		Boolean false
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

### <a name = "obj-ops"></a>3.8 Object Operators
#### [**Return to Table of Contents**](#toc)
These operators are used expressly to operate on the Object data type and change what a given object contains.

These are the operators:
- `objAddField`
	- Performance: O(n) where n is the number of Chars in the field name.
	- Given a stack where the third-to-top item is an ObjectBox, the second-to-top is a StringBox, and the top is any data type, consumes the three items on the stack, adds the field with the name held by the String in the StringBox with a value of the top item on the stack, pushing the same ObjectBox as before but now holding an Object with a new field added to it. 
	- General form: given stack `o` `s` `v` where `o` is a valid `ObjectBox`, `s` is a valid `StringBox`, and `v` is any data type, applying `objAddField` results in stack `o'` where `o'` has the same box number as before but holds the mutated `Object` with the field of name `s` and a value of `v`.
	- **NOTE:** Be aware that the field being added to the Object must not already exist, or an error is thrown. Also, be aware that even though the StringBox is consumed, it is not free'd. This must be done yourself later! 
	- Example Program:

		```
		//Creates empty Object on heap and shows that it's empty.
		{}
		debugPrintStack
		debugPrintHeap
		
		//Creates String on heap that will 
		// end up being used as the field name. 
		"foo"
		
		//Value of field.
		42
		
		//Stack and heap before adding field to Object.
		debugPrintStack
		debugPrintHeap
		
		//Adds field to Object held in ObjectBox.
		objAddField
		
		//Stack and heap after adding field to Object.
		debugPrintStack
		debugPrintHeap
		
		//Adding more values to showcase object with multiple fields.
		"bar" 3.14 objAddField
		"baz" "This is a String held by the object!" objAddField
		"qux" [] objAddField
		
		//Heap after more stuff was added to Object.
		debugPrintHeap
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0 
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
			Object {}
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0 
		StringBox 1 
		isize 42
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
			Object {}
		StringBox 1:
			String "foo"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0 
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
			Object {foo: isize 42}
		StringBox 1:
			String "foo"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
			Object {foo: isize 42, baz: StringBox 4, qux: ListBox 6, bar: f32 3.14}
		StringBox 1:
			String "foo"
		StringBox 2:
			String "bar"
		StringBox 3:
			String "baz"
		StringBox 4:
			String "This is a String held by the object!"
		StringBox 5:
			String "qux"
		ListBox 6:
			List []
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 7
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `objGetField`
	- Performance: O(n) where n is the number of Chars in the field name.
	- Given a stack where the second-to-top item is a valid ObjectBox and the top item is a valid StringBox, consumes the two items, indexes into the ObjectBox by the String held in the StringBox and pushes the value of the field to the stack.
	- General form: given stack `o` `s` where `o` is a valid `ObjectBox` and `s` is a valid `StringBox` where the `String` it holds exists in the `Object` held by the `ObjectBox`. Applying `objGetField` yields stack `v` where `v` is the value held by the `String` in the `Object`.
	- Be aware that the field must exist in the Object, if not an error is thrown. 
	- Example Program:

		```
		//Be aware that this program is incredibly wasteful for memory usage.
		// Since it's just a little example, it's fine 
		// but in a real program you'd absolutely want to free 
		// what you don't need anymore.
		
		//Creates Object with some fields.
		{} 
		"foo" 42 objAddField 
		"bar" 3.14 objAddField
		"baz" "String, wow!" objAddField
		
		debugPrintStack
		debugPrintHeap
		
		//Queries each field in constructed object. 
		// Dup is used to preserve Object for more lookups. 
		dup "foo" objGetField swap
		dup "bar" objGetField swap
		dup "baz" objGetField swap
		
		debugPrintStack
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
		        Object {baz: StringBox 4, bar: f32 3.14, foo: isize 42}
		StringBox 1:
		        String "foo"
		StringBox 2:
		        String "bar"
		StringBox 3:
		        String "baz"
		StringBox 4:
		        String "String, wow!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 5
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 42
		f32 3.14
		StringBox 4
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `objMutField`
	- Performance: O(n) where n is the number of Chars in the field name.
	- Given a stack where the third-to-top item is a valid ObjectBox, the second-to-top item is a valid StringBox, and the top is an item with a datatype matching what is stored in the ObjectBox with the field name held by the StringBox, pops all three items, changes the item at the StringBox to hold the new item, pushing the ObjectBox holding the mutated Object to the stack.
	- General form: given stack `o` `s` `v` where `o` is a valid `ObjectBox`, `s` is a valid `StringBox`, and `v` is a datatype matching the value stored at `s`, applying `objMutField` results in stack `o'` where `o'` is an `ObjectBox` with the same box number as `o` but contains the mutated `Object`.
	- Be aware that the desired field must exist in the given object and be the same data type as the value held at the top of the stack.
	- Example Program:

		```
		//Nothing in this program is freed 
		// but that's fine since it's small.
		{} "foo" 41 objAddField
		debugPrintStack
		debugPrintHeap
		"foo" 42 
		debugPrintStack
		objMutField
		debugPrintStack
		debugPrintHeap
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
		        Object {foo: isize 41}
		StringBox 1:
		        String "foo"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		StringBox 2
		isize 42
		--------------------------------
		STACK LENGTH: 3
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
		        Object {foo: isize 42}
		StringBox 1:
		        String "foo"
		StringBox 2:
		        String "foo"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 3
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `objRemField`
	- Performance: O(n) where n is the number of Chars of the desired field name.
	- Given a stack where the second-to-top item is a valid ObjectBox and the top is a valid StringBox holding a String that exists in the ObjectBox, consumes the two items and removes the field from the Object with the desired name contained in the String, pushing the same ObjectBox back to the stack holding the now mutated Object.
	- General form: given stack `o` `s` where `o` is a valid `ObjectBox` and `s` is a valid `StringBox` holding a `String` that is the name of an existing field in `o`, applying `objRemField` results in `o'` where `o'` is an `ObjectBox` with the same number as `o` but holds the updated `Object`
	- Example Program:

		```
		//Constructs test Object, not bothering 
		// to free the heap-allocated values 
		// because it's just a demo.
		{} 
		"foo" 42 objAddField
		"bar" 3.14 objAddField
		"baz" {} objAddField
		"qux" [] objAddField
		"REMOVE ME" "I SHOULDN'T BE IN AN OBJECT!!!" objAddField
		debugPrintStack
		debugPrintHeap
		
		//Gets rid of bad field.
		"REMOVE ME" debugPrintStack objRemField
		debugPrintStack
		debugPrintHeap
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
		        Object {foo: isize 42, bar: f32 3.14, qux: ListBox 6, REMOVE ME: StringBox 8, baz: ObjectBox 4}
		StringBox 1:
		        String "foo"
		StringBox 2:
		        String "bar"
		StringBox 3:
		        String "baz"
		ObjectBox 4:
		        Object {}
		StringBox 5:
		        String "qux"
		ListBox 6:
		        List []
		StringBox 7:
		        String "REMOVE ME"
		StringBox 8:
		        String "I SHOULDN'T BE IN AN OBJECT!!!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 9
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		StringBox 9
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
		        Object {foo: isize 42, bar: f32 3.14, qux: ListBox 6, baz: ObjectBox 4}
		StringBox 1:
		        String "foo"
		StringBox 2:
		        String "bar"
		StringBox 3:
		        String "baz"
		ObjectBox 4:
		        Object {}
		StringBox 5:
		        String "qux"
		ListBox 6:
		        List []
		StringBox 7:
		        String "REMOVE ME"
		StringBox 8:
		        String "I SHOULDN'T BE IN AN OBJECT!!!"
		StringBox 9:
		        String "REMOVE ME"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 10
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

### <a name = "bit-ops"></a> 3.9 Bitwise Operators
#### [**Return to Table of Contents**](#toc)
Bitwise operators are used to make alterations to the bits of integers directly. These are useful in complex hashing algorithms, flipping the value of a number quickly, checking for particular bitwise flags, etc. 

These are the operators employed below:
- `bitOr`
	- Performance: O(1)
	- Given a stack where the top two elements are matching integer types, consumes both of them and performs a bitwise OR operation on them, pushing the result to the stack. A bitwise OR involves comparing each of the bits between two integers which determines the bit at the same position in the new integer. As long as at least one of the bits is a 1, the result is 1, otherwise it's 0. Because this operation is practically a CPU instruction and it's done on fixed-width integers, it's constant time.
	- General form: given stack `x` `y` where `x` and `y` are both type `t` which can be type `isize`, `usize`, `i8`, `i16`. `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, or `u128`, applying `bitOr` results in stack `z` where `z` is also type `t` and is the result of a bitwise OR between `x` and `y`.
	- Example Program:

		```
		1 2 bitOr
		1 1 bitOr
		23u8 64u8 bitOr
		64usize 1usize bitOr
		debugPrintStack
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 3
		isize 1
		u8 87
		usize 65
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `bitAnd`
	- Performance: O(1)
	- Given a stack where the top two elements are matching integer types, consumes both of them and performs a bitwise AND operation on them, pushing the result to the stack. A bitwise AND involves comparing each of the bits between two integers which determines the bit at the same position in the new integer. For a bitwise AND, both bits have to be 1 for the resulting bit to be 1, otherwise it's 0. Because this operation is practically a CPU instruction and it's done on fixed-width integers, it's constant time.
	- General form: given stack `x` `y` where `x` and `y` are both type `t` which can be type `isize`, `usize`, `i8`, `i16`. `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, or `u128`, applying `bitAnd` results in stack `z` where `z` is also type `t` and is the result of a bitwise AND between `x` and `y`.
	- Example Program:

		```
		1 2 bitAnd
		1 1 bitAnd
		23u8 64u8 bitAnd
		64usize 1usize bitAnd
		259 3 bitAnd
		debugPrintStack
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 0
		isize 1
		u8 0
		usize 0
		isize 3
		--------------------------------
		STACK LENGTH: 5
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `bitXor`
	- Performance: O(1)
	- Given a stack where the top two elements are matching integer types, consumes both of them and performs a bitwise XOR operation on them, pushing the result to the stack. A bitwise XOR involves comparing each of the bits between two integers which determines the bit at the same position in the new integer. For a bitwise XOR, exactly one bit has to be 1 and one has to be 0 for the result to be 1, otherwise it's 0. Because this operation is practically a CPU instruction and it's done on fixed-width integers, it's constant time.
	- General form: given stack `x` `y` where `x` and `y` are both type `t` which can be type `isize`, `usize`, `i8`, `i16`. `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, or `u128`, applying `bitXor` results in stack `z` where `z` is also type `t` and is the result of a bitwise XOR between `x` and `y`.
	- Example Program:

		```
		1 2 bitXor
		1 1 bitXor
		23u8 64u8 bitXor
		64usize 1usize bitXor
		259 3 bitXor
		15i128 14i128 bitXor
		debugPrintStack
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 3
		isize 0
		u8 87
		usize 65
		isize 256
		i128 1
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `bitNot`
	- Performance: O(1)
	- Given a stack with the top being an integer type, consumes the item, pushing a version with the bitwise NOT operation applied to it. The bitwise NOT operation simply inverts the bits of a given integer when creating the new integer. 
	- General form: given stack `x` where `x` is type `isize`, `usize`, `i8`, `i16`. `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, or `u128`, applying `bitNot` results in stack `y` where `y` is the same data type as `x` but with inverted bits thanks to bitwise NOT.
	- When combined with one of the max operators such as `usizeMax`, `bitNot` can be used to find the minimum value of the various kinds of integers.
	- Example Program:

		```
		//Signed maxes paired with their mins.
		isizeMax dup bitNot
		i8Max dup bitNot
		i16Max dup bitNot
		i32Max dup bitNot
		i64Max dup bitNot
		i128Max dup bitNot
		
		//Unsigned maxes paired with their mins.
		usizeMax dup bitNot
		u8Max dup bitNot
		u16Max dup bitNot
		u32Max dup bitNot
		u64Max dup bitNot
		u128Max dup bitNot
		
		//Other examples
		64 dup bitNot
		0u8 dup bitNot
		
		debugPrintStack
		
		```

	- Resulting Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 9223372036854775807
		isize -9223372036854775808
		i8 127
		i8 -128
		i16 32767
		i16 -32768
		i32 2147483647
		i32 -2147483648
		i64 9223372036854775807
		i64 -9223372036854775808
		i128 170141183460469231731687303715884105727
		i128 -170141183460469231731687303715884105728
		usize 18446744073709551615
		usize 0
		u8 255
		u8 0
		u16 65535
		u16 0
		u32 4294967295
		u32 0
		u64 18446744073709551615
		u64 0
		u128 340282366920938463463374607431768211455
		u128 0
		isize 64
		isize -65
		u8 0
		u8 255
		--------------------------------
		STACK LENGTH: 28
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `bitShift`
	- Performance: O(1)
	- Given a stack where the second to top item is an integer and the top item is an isize, consumes the two items and pushes an integer based on the original integer shifted by n bits forward or backwards based on the isize.
	- General form: given stack `x` `y` where `x` is type `isize`, `usize`, `i8`, `i16`. `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, or `u128`, and `y` is type `isize`, applying `bitShift` results in stack `z` where `z` is the same data type as `x` but is shifted by `y` bits forward or backward.
	- The sign of the shift argument determines which direction the number is bit-shifted:
		- A positive number shifts the number to the left by n bits, making it bigger by some number of factors of two.
		- Zero does nothing to the number, pushing a number unchanged, effectively acting as a no-op.
		- A negative number shifts the number to the right n bits, making it smaller by some number of factors of two.
	- Be aware that there might be some edge cases not addressed with overflows with `bitShift`. Generally, the idea is that if you shift a number really far one way or another, it becomes zero because you shifted all of the bits off the map one way or another.
	- Example Program:

		```
		//An interesting way of multiplying 8 by 3.
		8 dup 1 bitShift +

		//:)
		315 4 bitShift
		
		//Just chops off any bits excluded.
		2025 -1 bitShift
		
		//Really far bit shift to right.
		42 -999 bitShift
		
		//Really far bit shift to left.
		666usize 666 bitShift
		
		//Easy way of constructing limits 
		// to integers within bigger integers.
		//In this case, it's a display 
		// of the 32 bit signed integer limit, 
		// contained inside a signed 128 bit integer.
		1i128 31 bitShift 1i128 -
		
		//Of course, there are many more fun ways to use this.
		
		debugPrintStack
		
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 24
		isize 5040
		isize 1012
		isize 0
		usize 0
		i128 2147483647
		--------------------------------
		STACK LENGTH: 6
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

### <a name = "io-ops"></a> 3.10 IO Operators
#### [**Return to Table of Contents**](#toc)
Input-Output operators, or IO operators, are operators that either write data in the form of Strings to stdout, the area in a terminal or otherwise that displays output, or read data from stdin, the file that holds input from users or other programs. Chars also make an appearance in IO but that's separate. Lmao also has operators that generate output in regular files and work for debugging purposes, but those will be covered in a later section. 

Listed below are all of the existing regular IO operators.
- `print`
	- Performance: O(n) where `n` is the number of Chars in the String being printed.
	- Given a stack where the top item is a valid StringBox, consumes the StringBox and writes the contents of its String to stdout, flushing stdout after completion.
	- General form: given stack `s` where `s` is a valid `StringBox`, applying `print` results in stack ` ` where there's nothing because `s` was consumed for printing.
	- Be aware that `print` doesn't free the `StringBox` involved however! Be sure to save that StringBox before printing it, either in a variable or using `dup` so it can be freed later on.
	- To output anything that isn't a StringBox, casting is required via the `cast` operator which will be covered later.
	- Example Program:

		```
		//Newline needs to be manually added since 
		// it just outputs all of the String's Chars.
		"Hello, World!\n"
		print
		```

	- Program Output:

		```
		Hello, World!
		```
- `read`
	- Performance: O(n) where `n` is the number of Chars in stdin being read.
	- Given a stack with nothing on it required, reads in stdin and allocates a StringBox to hold the String that was read in from user input in stdin, pushing the StringBox to the stack.
	- General form: given stack ` `, applying `read` results in stack `s` where `s` is a `StringBox` holding a `String` matching the contents of stdin consumed by `read`.
	- Be aware that this allocates memory on the heap via the StringBox.
	- Also know that to complete the read, `CTRL-D` or `CMD-D`, depending on the system, will have to be pressed up to a couple times to close stdin to allow `read` to consume the contents and allocate the String. However, the key combo really only needs to be pressed once if a newline is given at the end of the input which flushes the buffer and leads to a more easy close of stdin.
	- Example Program:

		```
		read
		debugPrintStack
		debugPrintHeap
		```

	- Program Input:

		```
		This is some text to be read in, wow!
		```
	
	- Program Output (Following pressing CTRL-D a couple times):

		```
		This is some text to be read in, wow!--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "This is some text to be read in, wow!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		
		```

	- Second Program with Newline at End:

		```
		read
		debugPrintStack
		debugPrintHeap
		
		//Output is same as input.
		print
		```

	- Second Input:

		```
		This is some more text!
		This time it's multi-line text :O
		Wowzers! 
		This time I'll be good and end it with a newline.
		
		```

	- Second Output (After pressing CTRL-D once):

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "This is some more text!\nThis time it's multi-line text :O\nWowzers! \nThis time I'll be good and end it with a newline.\n"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		This is some more text!
		This time it's multi-line text :O
		Wowzers!
		This time I'll be good and end it with a newline.
		```

- `printChar`
	- Performance: O(1)
	- Given a stack with a Char on the top of the stack, consumes the Char and writes it to stdout. 
	- General form: given stack `c` where `c` is type `Char`, applying `printChar` yields stack ` `  with `c` being written to stdout.
	- This is basically the same as print except it's just one Char. 
	- This also doesn't need any heap allocations to run since it's just spitting out a Char which is a primitive.
	- Example Program:

		```
		//Prints out abc and a newline.
		'a' printChar 'b' printChar 'c' printChar '\n' printChar
		```

	- Program Output:

		```
		abc
		```

- `readChar`
	- Performance: O(1)
	- Given a stack with nothing on it, reads in a Char from stdin and pushes it to the stack.
	- General form: given stack ` `, applying `readChar` results in stack `c` where `c` is a `Char` read in from stdin.
	- Since this operator merely reads in a Char, nothing is allocated on the heap.
	- `readChar` merely consumes the Char at the start of stdin. If more input is typed, future calls consume Chars of that input, not new stuff, which can lead to some unexpected results at times.
	- Example Program:

		```
		readChar
		readChar 
		readChar
		readChar
		debugPrintStack
		```

	- Program Input:

		```
		abc
		
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Char 'a'
		Char 'b'
		Char 'c'
		Char '\n'
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `printLine`
	- Performance: O(n) where `n` is the number of Chars in the output String.
	- Given a stack where the top item is a valid StringBox, consumes the StringBox and writes its contents to stdout plus a newline Char. 
	- General form: Given stack `s` where `s` is a valid `StringBox`, applying `printLine` results in stack ` ` where the contents of `s` where written to stdout as well as a newline `Char \n`.
	- This operator is basically the same as `print` except that it also outputs a newline at the end, meaning you don't have to. This is a good operator for regular print operations that don't need specific endpoints to be specified.
	- Example Program:

		```
		//No newline needs to be specified at the end!
		"Hello, World!" printLine
		```

	- Example Output:

		```
		Hello, World!
		```

- `readLine`
	- Performance: O(n) where `n` is the number of Chars being read from a line of stdin.
	- Given a stack needing nothing on it, reads a line ending with a `\n` Char from stdin and allocates the String on the heap, pushing a StringBox pointing to it.
	- General form: given stack ` `, applying `readLine` results in stack `s` where `s` is a valid StringBox holding a `String` representing the line of input read in from stdin.
	- Unlike `read`, simply pressing enter at the end of the input line is enough to close stdin and write that line to the file, since enter puts a `\n` Char into it which flushes stdin. 
	- Like read, this allocates something on the heap, so be sure to save the StringBox in a variable or via `dup` if you want to free it later.
	- Example Program:

		```
		"Enter your age: " print
		readLine
		"You are " print print
		" years old!" printLine
		```

	- Program Input:

		```
		Enter your age: 42
		
		```

	- Program Output:

		```
		You are 42 years old!
		```

	- Program Explanation:
		- This program takes in `42` as user input and writes it out with some strings to indicate the user's input age. Since this input wasn't actually cast to an integer, any input would've worked. For instance, it could've said as output: `You are mute years old!`.

#### <a name = "debug-io-ops"></a> 3.10.1 Debug IO Operators
##### [**Return to Table of Contents**](#toc)
Debug IO operators are operators that are in the category of IO but only because they write to stdout when called. These operators are useful for debugging purposes and not meant for typical use beyond debugging.

Below are the operators that fall into this category:
- `debugPrintStack`
	- Performance: O(n) where `n` is the number of items on the stack.
	- Given a stack with anything on it, writes the contents of the stack to stdout in a debug format readable by humans. 
	- General form: given stack `x` where `x` is a stack composed of any combination of values, applying `debugPrintStack` yields stack `x` where `x` is the same stack with nothing changed where the contents of `x` are written to stdout.
	- This operator involves many string allocations and system calls under the hood, so only use this when trying to see what the stack is doing for debugging or if you just want to show off the final stack like in many of the previous example programs.
	- Example Program:

		```
		//Can print an empty stack.
		debugPrintStack
		
		//Can print a stack with anything else!
		1 2usize 3i8 5040i128 3.14 6.28e23f64 'a' false 
		"foo" [] {} box null ;
		debugPrintStack
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		--------------------------------
		STACK LENGTH: 0
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 1
		usize 2
		i8 3
		i128 5040
		f32 3.14
		f64 6.28e23
		Char 'a'
		Boolean false
		StringBox 0
		ListBox 1
		ObjectBox 2
		NULLBox
		--------------------------------
		STACK LENGTH: 12
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `debugPrintHeap`
	- Performance: 
		- O(n^2)
		- Prints worst case n items on heap containing n items.
	- Given any stack, prints the contents of the heap at that moment.
	- General form isn't needed because it doesn't even involve the stack.
	- This is a debug operator that allows you to gaze into the heap and understand its workings. 
	- Beyond learning and debugging, `debugPrintHeap` is also useful for seeing how free or not free the heap is with statistics on free cells versus not and a percentage given at the bottom. This is useful for tracking memory leaks and what blobs of data actually get freed. 
	- Example Program:

		```
		//Empty heap
		debugPrintHeap
		
		//Some stuff!
		"foo" "bar" "baz"
		debugPrintHeap
		
		//Heap with item freed
		box free ;
		debugPrintHeap
		
		//Memory cell re-use.
		[] debugPrintHeap
		
		//Freeing everything!
		box free ; box free ; box free ;
		debugPrintHeap
		```
	
	- Program Output:

		```
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 0
		////////////////////////////////
		PERCENT OF HEAP FREE'D: NaN
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "foo"
		StringBox 1:
		        String "bar"
		StringBox 2:
		        String "baz"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 3
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "foo"
		StringBox 1:
		        String "bar"
		StringBox 2 [FREE]:
		        String "baz"
		////////////////////////////////
		FREE'D BOX NUMBERS: [2]
		////////////////////////////////
		FREE'D BOX COUNT: 1
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 3
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 33.33
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "foo"
		StringBox 1:
		        String "bar"
		ListBox 2:
		        List []
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 3
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0 [FREE]:
		        String "foo"
		StringBox 1 [FREE]:
		        String "bar"
		ListBox 2 [FREE]:
		        List []
		////////////////////////////////
		FREE'D BOX NUMBERS: [2, 1, 0]
		////////////////////////////////
		FREE'D BOX COUNT: 3
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 3
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 100.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

### <a name = "file-ops"></a> 3.11 File Operators
#### [**Return to Table of Contents**](#toc)
File Operators are operators involving files. Who would've guessed that? Jokes aside, these are used to perform file system operations on the system. Through these operators, it's possible to do things like save files from programs, allowing for games that survive a reboot and other fun things. 

Below are all the existing file operators:
- `fileExists` 
	- Performance: O(n) where `n` is the number of Chars in the file name.
	- Given a stack where the top item is a valid StringBox, `fileExists` consumes the StringBox and checks to see if the file exists within the current program directory or if the file exists as per a provided absolute directory, pushing a Boolean based on the result.
	- General form: given stack `s` where `s` is a valid `StringBox`, applying `fileExists` results in stack `b` where `b` is a `Boolean` that is based on whether or not the file with name `s` exists.
	- Be aware that a file will be deemed non-existent if it has no access permissions, even though it does exist. 
	- Example Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  2 02:53 .
		drwxr-xr-x 5 janJesi users    4096 Apr  2 02:49 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		```

	- Example Program:

		```
		//These exist
		"lmao" fileExists
		"foo" fileExists
		
		//This doesn't exist.
		"bar" fileExists
		
		//This exists but is marked as non-existent 
		// due to lack of permissions.
		"NuclearCodes.txt" fileExists
		
		debugPrintStack
		
		```
	
	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean true
		Boolean true
		Boolean false
		Boolean false
		--------------------------------
		STACK LENGTH: 4
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `fileCreate`
	- Performance: O(n) where `n` is the number of Chars of the file name.
	- Given a stack where the top item is a valid StringBox, consumes the StringBox and creates a file with the name held by the String in the StringBox.
	- General form: given stack `s` where `s` is a valid `StringBox`, applying `fileCreate` yields stack ` ` with the `String` held by `s` being used as the basis for the file name.
	- Be aware that an error will be thrown if the file already exists!
	- Example Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  2 02:53 .
		drwxr-xr-x 5 janJesi users    4096 Apr  2 02:49 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		```

	- Example Program:
	
		```
		"toki.txt" fileCreate
		```
	
	- Updated Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  3 06:00 .
		drwxr-xr-x 5 janJesi users    4096 Apr  2 02:49 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		-rw-r--r-- 1 janJesi users       0 Apr  3 06:00 toki.txt
		```

- `fileWrite`
	- O(n) where `n` is the number of Chars in the String to be written to a file or the name, whichever is bigger.
	- Given a stack with two valid StringBoxes on top, consumes the two and writes the top StringBox's String to a file by the name held in the String of the second-to-top StringBox.
	- General form: given stack `x y` where `x` and `y` are type `t` where `t` is type `StringBox` and is valid, applying `fileWrite` results in stack ` ` where the contents of the `String` held by `y` were written to an existing file by the name of the `String` held by `x`.
	- Be aware that `fileWrite` needs the file to exist to write to, otherwise it throws an error!
	- Also know that even though the two StringBoxes are consumed, their contents aren't freed, so be sure to save the boxes if you intend on freeing them.
	- Example Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  3 06:00 .
		drwxr-xr-x 5 janJesi users    4096 Apr  4 02:03 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		-rw-r--r-- 1 janJesi users       0 Apr  3 06:00 toki.txt
		```

	- Example Program:
	
		```
		"toki.txt" 
		"toki! mi jan Jesi. mi ken toki kepeken toki pona. sina ken ala ken pali e ni?\n"
		fileWrite
		```
	
	- Updated Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  3 06:00 .
		drwxr-xr-x 5 janJesi users    4096 Apr  4 02:03 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		-rw-r--r-- 1 janJesi users      78 Apr  4 02:09 toki.txt
		```

	- Notice that the size of `toki.txt` increased and the date modified changed. This means the String in the example was written to the file.

- `fileRead`
	- Performance: O(n) where `n` is the number of Chars in the file being read in.
	- Given a stack where the top item is a valid StringBox, consumes the StringBox, reads the String it holds, opens a file based on that String, and reads the contents into a new allocated String that's then contained by a new StringBox which is pushed to the stack.
	- General form: given stack: `x` where `x` is a valid `StringBox`, applying `fileRead` results in stack `y` where `y` is a `StringBox` holding a `String` based on the contents of file `x`.
	- Just because `fileRead` consumes the StringBox doesn't mean that it frees it. Be sure to save that box if you want to free it, either by using `dup` or a variable.
	- `fileRead` also allocates new data on the heap that can potentially be massive if the file is large.
	- Of course, the file being read from needs to exist and have necessary permissions. For instance, trying to read `NuclearCodes.txt` would throw an error even though it exists.
	- Example Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  3 06:00 .
		drwxr-xr-x 5 janJesi users    4096 Apr  4 02:03 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		-rw-r--r-- 1 janJesi users      78 Apr  4 02:09 toki.txt
		```

	- Example Program:
	
		```
		//Reads in file data to new StringBox.
		"toki.txt" fileRead
		
		//Debugs to show file contents are allocated.
		debugPrintStack
		debugPrintHeap
		
		//Prints file contents to stdout, following indicator.
		"FILE CONTENTS:" printLine
		print
		```
	
	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 1
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "toki.txt"
		StringBox 1:
		        String "toki! mi jan Jesi. mi ken toki kepeken toki pona. sina ken ala ken pali e ni?\n"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		FILE CONTENTS:
		toki! mi jan Jesi. mi ken toki kepeken toki pona. sina ken ala ken pali e ni?
		```

- `fileRemove`
	- Performance: O(n) where `n` is the number of Chars in the file name.
	- Given a stack with a valid StringBox on the top, consumes the StringBox and removes the file at the path given by the file name String held by the StringBox.
	- General form: given stack `s` where `s` is a valid `StringBox`, applying `fileRemove` results in stack ` ` where the `String` held by `s` was used to remove a file, provided it exists and the permissions were allowed.
	- Be aware that the file must exist to be removed, otherwise an error is thrown.
	- Example Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  3 06:00 .
		drwxr-xr-x 5 janJesi users    4096 Apr  4 02:03 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		-rw-r--r-- 1 janJesi users      78 Apr  4 02:09 toki.txt
		```

	- Example Program:

		```
		//File exists.
		"toki.txt" fileExists
		//Removing file.
		"toki.txt" fileRemove
		//File doesn't exist after deletion.
		"toki.txt" fileExists
		debugPrintStack
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean true
		Boolean false
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

	- Updated Current Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr  3 06:00 .
		drwxr-xr-x 5 janJesi users    4096 Apr  4 02:03 ..
		-rw-r--r-- 1 janJesi users       0 Apr  2 02:50 foo
		-rwxr-xr-x 1 janJesi users 1429848 Apr  2 02:51 lmao
		---------- 1 janJesi users      15 Apr  2 02:53 NuclearCodes.txt
		```

### <a name = "type-ops"></a> 3.12 Type Operators
#### [**Return to Table of Contents**](#toc)
These operators are employed for matters related to datatypes specifically. These are useful for changing or looking at the data types of items on the stack.

Below are all operators of this category:
- `queryType`
	- Performance: O(1)
	- Given a stack with the top being any data type, consumes the item, allocates a String based on its data type and pushes a StringBox holding said String.
	- General form: given stack `v` where `v` is any data type, applying `queryType` results in stack `d` where `d` is a `StringBox` holding a `String` representation of `v`'s data type.
	- Example Program:

		```
		//An exhaustive list of all types in Lmao.
		
		//CPU-optimal integers 
		2319 dup queryType dup printLine
		666usize dup queryType dup printLine
		
		//Signed integers
		42i8 dup queryType dup printLine
		5040i16 dup queryType dup printLine
		2025i32 dup queryType dup printLine
		8000000000i64 dup queryType dup printLine
		i128Max dup queryType dup printLine
		
		//Unsigned integers
		215u8 dup queryType dup printLine
		52375u16 dup queryType dup printLine
		3000000000u32 dup queryType dup printLine
		89u64 dup queryType dup printLine
		999999999999999999999999999999999u128 dup queryType dup printLine
		
		//Floats
		3.14 dup queryType dup printLine
		6.02e23f64 dup queryType dup printLine
		
		//Char and Boolean
		'A' dup queryType dup printLine
		false dup queryType dup printLine
		
		//Heap allocated stuff
		"This is some text!" dup queryType dup printLine
		[] dup queryType dup printLine
		{} dup queryType dup printLine
		
		//Extra boxes
		35235 box make ; dup queryType dup printLine
		box null ; dup queryType dup printLine
		
		debugPrintStack
		debugPrintHeap
		
		```

	- Program Output:

		```
		isize
		usize
		i8
		i16
		i32
		i64
		i128
		u8
		u16
		u32
		u64
		u128
		f32
		f64
		Char
		Boolean
		StringBox
		ListBox
		ObjectBox
		MiscBox
		NULLBox
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		isize 2319
		StringBox 0
		usize 666
		StringBox 1
		i8 42
		StringBox 2
		i16 5040
		StringBox 3
		i32 2025
		StringBox 4
		i64 8000000000
		StringBox 5
		i128 170141183460469231731687303715884105727
		StringBox 6
		u8 215
		StringBox 7
		u16 52375
		StringBox 8
		u32 3000000000
		StringBox 9
		u64 89
		StringBox 10
		u128 999999999999999999999999999999999
		StringBox 11
		f32 3.14
		StringBox 12
		f64 6.02e23
		StringBox 13
		Char 'A'
		StringBox 14
		Boolean false
		StringBox 15
		StringBox 16
		StringBox 17
		ListBox 18
		StringBox 19
		ObjectBox 20
		StringBox 21
		MiscBox 22
		StringBox 23
		NULLBox
		StringBox 24
		--------------------------------
		STACK LENGTH: 42
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
				String "isize"
		StringBox 1:
				String "usize"
		StringBox 2:
				String "i8"
		StringBox 3:
				String "i16"
		StringBox 4:
				String "i32"
		StringBox 5:
		        String "i64"
		StringBox 6:
				String "i128"
		StringBox 7:
				String "u8"
		StringBox 8:
				String "u16"
		StringBox 9:
				String "u32"
		StringBox 10:
				String "u64"
		StringBox 11:
				String "u128"
		StringBox 12:
				String "f32"
		StringBox 13:
				String "f64"
		StringBox 14:
				String "Char"
		StringBox 15:
				String "Boolean"
		StringBox 16:
		        String "This is some text!"
		StringBox 17:
				String "StringBox"
		ListBox 18:
				List []
		StringBox 19:
				String "ListBox"
		ObjectBox 20:
				Object {}
		StringBox 21:
				String "ObjectBox"
		MiscBox 22:
				isize 35235
		StringBox 23:
				String "MiscBox"
		StringBox 24:
				String "NULLBox"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 25
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `cast`
	- Performance: O(n) -> linear cast of linear data structures like String to List.
	- Given a stack where the second-to-top element can be any data type and the top element is a valid StringBox, consumes both items and attempts to cast the second-to-top item to the data type specified by the String held by the top `StringBox` or throws an error on failure.
	- General form: given stack `v s` where `v` is any data type and `s` is a valid `StringBox`, applying `cast` results in `x` where `x` is the same data type as what is specified by the `String` held by `s`. If the cast is invalid, an error is thrown, i.e. a `ListBox` to an `isize`. 
	- The following example program is nowhere near exhaustive but showcases some common and interesting uses for `cast`.
	- Like other operators, `cast` doesn't actually free any memory of consumed boxes, it simply uses the data. Be sure to save and free any unneeded boxes.
	- Example Program:

		```
		//Useful for Boolean-based array indexing.
		false "usize" cast
		
		//Necessary for input parsing.
		"3.14" "f32" cast
		"666" "i16" cast
		
		//Necessary for showing computation results.
		2 3 + "String" cast
		
		//Cool way to turn a String into a list of Chars.
		"This is a String? NO!" "List" cast
		
		//Useful for turning a List to a String.
		[] 1 p 2 p 2.718 p [] p {} p 
		"String" cast
		
		//Object stringification.
		{} "foo" 42 objAddField 
		"bar" box null ; objAddField
		"baz" false objAddField
		"String" cast
		
		debugPrintStack
		debugPrintHeap
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 0
		f32 3.14
		i16 666
		StringBox 6
		ListBox 9
		StringBox 14
		StringBox 20
		--------------------------------
		STACK LENGTH: 7
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "usize"
		StringBox 1:
		        String "3.14"
		StringBox 2:
		        String "f32"
		StringBox 3:
		        String "666"
		StringBox 4:
		        String "i16"
		StringBox 5:
		        String "String"
		StringBox 6:
		        String "5"
		StringBox 7:
		        String "This is a String? NO!"
		StringBox 8:
		        String "List"
		ListBox 9:
		        List [Char 'T', Char 'h', Char 'i', Char 's', Char ' ', Char 'i', Char 's', Char ' ', Char 'a', Char ' ', Char 'S', Char 't', Char 'r', Char 'i', Char 'n', Char 'g', Char '?', Char ' ', Char 'N', Char 'O', Char '!']
		ListBox 10:
		        List [isize 1, isize 2, f32 2.718, ListBox 11, ObjectBox 12]
		ListBox 11:
		        List []
		ObjectBox 12:
		        Object {}
		StringBox 13:
		        String "String"
		StringBox 14:
		        String "[isize 1, isize 2, f32 2.718, ListBox 11, ObjectBox 12]"
		ObjectBox 15:
		        Object {bar: NULLBox, baz: Boolean false, foo: isize 42}
		StringBox 16:
		        String "foo"
		StringBox 17:
		        String "bar"
		StringBox 18:
		        String "baz"
		StringBox 19:
		        String "String"
		StringBox 20:
		        String "{bar: NULLBox, baz: Boolean false, foo: isize 42}"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 21
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

### <a name = "meta-ops"></a>3.13 Meta Operators
#### [**Return to Table of Contents**](#toc)
These operators are ones that break the control flow of a program, causing the program to no longer run from top to bottom.

- `throwCustomError`
	- Performance: O(n) where `n` is the number of Chars in the error message String.
	- Given a stack where the top is a valid StringBox, consumes the StringBox throws an error with the contents being the same as the given StringBox.
	- General form: given stack `s` where `s` is a valid `StringBox`, applying `throwCustomError` results in stack ` ` where `s` was consumed for the error's contents. 
	- Obviously, this operator always throws errors, even if used correctly. These errors thus can be let free to stop the program or caught by an `attempt` `onError` fancy operator. 
	- This operator is largely useful for things akin to asserts in other languages, deliberately throwing an error if some condition is not met. It can also just be used for a more violent form of print debugging.
	- The error being printed itself works like `printLine` where it adds a `\n` character at the end of the String automatically. 
	- Example Program:

		```
		"ERROR! Something broke! AAAHHHGHH!" throwCustomError
		
		"This shouldn't print if there's an error!" printLine
		"Other thing that shouldn't print!" printLine
		```

	- Program Output:

		```
		ERROR! Something broke! AAAHHHGHH!
		```

- `leaveScopeIfTrue`
	- Performance: O(1)
	- Given a stack where the top item is a Boolean, consumes the Boolean and leaves the present program scope if the value is `True`, otherwise does nothing.
	- General form: given stack `b` where `b` is type `Boolean`, applying `leaveScopeIfTrue` results in stack ` ` where `b` was consumed to determine if the current scope needs to be left.
	- If this operator is used at the global scope, the program stops running early if the Boolean is `true`.
	- If this operator is used in the scope of a fancy operator, it leaves that scope early if `true`, skipping to whatever's next for the code to run.
	- Since this operator breaks control flow, it's useful for exiting things early, like `while` loops, `func`tions, and more.
	- Example Program:

		```
		true
		if
			"THIS PRINTS!"
			printLine
		
			//Leaves if statement early.
			true
			leaveScopeIfTrue
		
			"THIS SHOULD NEVER PRINT!"
			printLine
		
			"ERROR THAT SHOULD NEVER HAPPEN!"
			throwCustomError
		;
		
		"AFTER IF STATEMENT" printLine
		
		true leaveScopeIfTrue
		
		"FINAL FUNNY ERROR THAT'S SKIPPED!"
		throwCustomError
		```

	- Program Output:

		```
		THIS PRINTS!
		AFTER IF STATEMENT
		```

### <a name = "other-ops"></a>3.14 Other Operators
#### [**Return to Table of Contents**](#toc)
These operators are simply ones that don't fit in any of the previous categories quite right, so they go here. For instance: `contains` might seem like a List and String operator until you realize that it also works with Objects but it's not an Object operator either. So it goes here. 

Listed below are all the operators of this category:
- `contains`
	- Performance: O(n)
	- Given a stack where the second-to-top item is a ListBox/StringBox/ObjectBox and the top item is a Value/Char/StringBox, consumes the two items and pushes a Boolean based on whether or not the ListBox/StringBox/ObjectBox contains the Value/Char/StringBox (as a field). 
	- General form: given stack `c v` where `c` is a valid `ListBox`, `StringBox`, or `ObjectBox`, and `v` is a `Value`, `Char`, or `StringBox` respectively, applying `contains` results in stack `b` where `b` is a `Boolean` based on whether `c` contains `v`.
	- Regarding Objects, the StringBox that is being searched for is the name of a field in the Object. If the Object doesn't have the field, the Boolean is `False`.
	- Example Program:

		```
		//Constructs List example.
		[] 'a' p 42 p 666 push 5040 p
		
		//Tests for containment of three possible values.
		dup 'a' contains
		swap dup 42 contains
		swap dup 6.02e23f64 contains
		swap
		
		//Constructs example String.
		"This is a sentence!"
		
		//Tests for existence of two different Chars in String.
		dup 'i' contains
		swap dup 'Z' contains
		swap
		
		//Example Object construction.
		{} "foo" 42 objAddField
		"bar" box null ; objAddField
		"baz" 3.14 objAddField
		
		//Checks for membership of two field names.
		dup "foo" contains
		swap dup "qux" contains
		swap 
		
		debugPrintStack
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		Boolean true
		Boolean true
		Boolean false
		ListBox 0
		Boolean true
		Boolean false
		StringBox 1
		Boolean true
		Boolean false
		ObjectBox 2
		--------------------------------
		STACK LENGTH: 10
		--------------------------------
		END STACK PRINT
		--------------------------------
		```

- `isValidBox`
	- Performance: O(1)
	- Given a stack where the top is a box type, consumes the box and pushes a Boolean based on whether or not the box is valid. 
	- General form: given stack `v` where `v` is type `StringBox`/`ListBox`/`ObjectBox`/`MiscBox`/`NULLBox`, applying `isValidBox` results in stack `b` where `b` is a `Boolean` that expresses whether or not `v` is a valid (non-freed) box.
	- This operator is useful for checking if a box is useable, like in a data structure. This is equivalent to a pointer validity check in other languages.
	- Example Program:

		```
		//Valid and invalid StringBoxes.
		"foo" dup isValidBox
		"bar" dup box free ; dup isValidBox
		
		//Valid and invalid ListBoxes.
		[] dup isValidBox
		[] 1 p 2 p 3 p dup box free ; dup isValidBox
		
		//Valid and invalid ObjectBoxes.
		{} dup isValidBox
		{} dup box free ; dup isValidBox
		
		//Valid and invalid MiscBoxes.
		7 box make ; dup isValidBox
		666i16 box make ; dup box free ; dup isValidBox
		
		//NULLBox is NEVER valid!
		box null ; dup isValidBox
		
		debugPrintStack
		debugPrintHeap
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		Boolean true
		StringBox 1 [INVALID]
		Boolean false
		ListBox 1
		Boolean true
		ListBox 2 [INVALID]
		Boolean false
		ObjectBox 2
		Boolean true
		ObjectBox 3 [INVALID]
		Boolean false
		MiscBox 3
		Boolean true
		MiscBox 4 [INVALID]
		Boolean false
		NULLBox
		Boolean false
		--------------------------------
		STACK LENGTH: 18
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "foo"
		ListBox 1:
		        List []
		ObjectBox 2:
		        Object {}
		MiscBox 3:
		        isize 7
		MiscBox 4 [FREE]:
		        i16 666
		////////////////////////////////
		FREE'D BOX NUMBERS: [4]
		////////////////////////////////
		FREE'D BOX COUNT: 1
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 5
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 20.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `getArgs`
	- Performance: O(n) where `n` is the number of Chars that make up the command-line arguments. 
	- Given a stack with anything on it, the operator pulls the command-line arguments for the program and allocates them on the heap as a ListBox containing a List of StringBoxes holding the arguments, pushing this ListBox to the stack.
	- General form: given stack ` `, applying `getArgs` results in stack `A` where `A` is type `ListBox` that holds all `StringBox`es from the arguments.
	- This operator is effectively Lmao's equivalent of something like `argv` or `String[] args` from C or Java respectively. In other words, when calling the program to run, any extra arguments provided are allocated from a call of `getArgs`.
	- Be aware that if many command-line arguments were provided, a massive amount of space will be allocated on the heap, both for the ListBox itself as well as all of the StringBoxes inside it holding the Strings representing the command-line arguments. This means that a loop will be needed to go through and free all the items of the ListBox and then free the ListBox itself.
	- Like the idea of something like argv, the first item in the ListBox is the name of the program.
	- Example Directory:

		```
		drwxr-xr-x 2 janJesi users    4096 Apr 18 16:10 .
		drwxr-xr-x 6 janJesi users    4096 Apr 18 16:06 ..
		-rw-r--r-- 1 janJesi users      23 Apr 18 16:10 example.lmao
		-rwxr-xr-x 1 janJesi users 1429848 Apr 18 16:07 lmao
		```

	- Example Program (`example.lmao`):
	
		```
		//Gets arguments and shows them allocated on the heap
		// and the ListBox on the stack.
		getArgs
		debugPrintStack
		debugPrintHeap
		```
	
	- Example Program Command-Line:
	
		```
		./lmao example.lmao foo bar baz
		```
	
	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 4
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "example.lmao"
		StringBox 1:
		        String "foo"
		StringBox 2:
		        String "bar"
		StringBox 3:
		        String "baz"
		ListBox 4:
		        List [StringBox 0, StringBox 1, StringBox 2, StringBox 3]
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 5
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

## <a name = "fancy-ops"></a> 4 Fancy Operators
### [**Return to Table of Contents**](#toc)
Fancy operators are the backbone of more advanced programs in Lmao. 
Fancy operators are the operators that are employed to create the functionality 
of Lmao beyond merely interfacing with the stack. Such functionality includes if statements, while loops, functions, variables, and more.
These operators are called "Fancy Operators" because they require more than one word to invoke them. 
A regular operator like `dup` needs just the one word but something like a `box` operator requires several to make
such as `box free ;`. The primary difference between regular operators and fancy operators is the presence of the semi-colon 
at the end of the operator, to signify completion of the operator invocation.

**NOTE**: All fancy operators need at least a space *and* a semi-colon at their end, so `box free;` isn't valid but `box free ;` is.
Even though one space is needed before the semi-colon, more space can be added. For instance, `box free                          ; ` is also valid.

### <a name = "box-fan-ops"></a>4.1 Box Fancy Operators
#### [**Return to Table of Contents**](#toc)
These are the operators that not only allow for the freeing of heap-allocated memory to be freed but also allow for the creation and interfacing with `MiscBox`es, boxes that can contain primitives from the stack, which allows for fun things like nested boxes and other powerful features.

Below are all of the kinds of fancy box operators.
- `box free ;`
	- Performance: O(1)
	- Given a stack with any kind of valid non-null box on top, consumes the box and frees it on the heap, allowing that memory cell to be reused, freeing up the box number to be used by another kind of box or even the same one.
	- General form: given stack `b` where `b` is valid and type `StringBox`, `ListBox`, `ObjectBox`, or `MiscBox`, applying `box free ;` results in stack ` ` where the data held by `b` on the heap is freed. 
	- If the box in question has already been freed, an error is thrown.
	- This fancy operator is incredibly useful for maintaining programs that allocate a large amount of data on the heap, as this is how it's freed. Think of `box free ;` like the `free()` function in C.
	- Example Program:

		```
		"This is some data allocated on the heap!" 
		debugPrintStack
		debugPrintHeap

		//Freeing StringBox.
		box free ;
		debugPrintStack
		debugPrintHeap

		//Allocating new data.
		[]
		debugPrintStack
		debugPrintHeap

		``` 

	- Example Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		StringBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0:
		        String "This is some data allocated on the heap!"
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		--------------------------------
		STACK LENGTH: 0
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		StringBox 0 [FREE]:
		        String "This is some data allocated on the heap!"
		////////////////////////////////
		FREE'D BOX NUMBERS: [0]
		////////////////////////////////
		FREE'D BOX COUNT: 1
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 100.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		ListBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ListBox 0:
		        List []
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `box null ;`
	- Performance: O(1)
	- Given a stack with anything on it, calling this operator pushes a NULLBox, a box that points nowhere. 
	- General form: given stack ` `, applying `box null ;` results in stack `n` where `n` is type `NULLBox`.
	- This program is useful for giving Objects fields meant for boxes a default value that points nowhere. This is equivelent to nullpointers in other languages where the NULLBox acts purely as a placeholder for a real box. It's an IOU for a box.
	- Example Program:

		```
		//Pure example of pushing NULLBox to stack.
		box null ; 

		//Example of it being used in an Object.
		// In this case, it's a BST node.
		//This looks weird because I'm freeing the memory 
		// of the Strings being used for the field names.
		{} 
		"val" dup rot 0 objAddField swap box free ;
		"left" dup rot box null ; objAddField swap box free ;
		"right" dup rot box null ; objAddField swap box free ;

		debugPrintStack
		debugPrintHeap
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		NULLBox
		ObjectBox 0
		--------------------------------
		STACK LENGTH: 2
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		ObjectBox 0:
		        Object {val: isize 0, left: NULLBox, right: NULLBox}
		StringBox 1 [FREE]:
		        String "right"
		////////////////////////////////
		FREE'D BOX NUMBERS: [1]
		////////////////////////////////
		FREE'D BOX COUNT: 1
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 2
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 50.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

	- This program featured the `box free ;` operator to showcase how memory can be reused. Instead of wasting heap cells on each of the field names for the Object, one cell could be reused.

- `box make ;`
	- Performance: O(1)
	- Given a stack with at least one item contained therein, consumes the top item, allocates space for it on the heap, and pushes a MiscBox pointing to that item. 
	- General form: given stack `v` where `v` is any data type, applying `box make ;` results in stack `m` where `m` is type `MiscBox` and contains `v` on the heap.
	- This fancy operator is really useful for things like nested-boxes, where a MiscBox can contain a MiscBox that contains a MiscBox and so on, allowing for more interesting data structures to be created.
	- This also allows for the equivalent of things like integer pointers in other languages to be created through MiscBoxes.
	- Example Program:

		```
		//Integer pointer equivalent.
		42 box make ;

		//Can hold things like Booleans or Chars.
		'A' box make ;
		false box make ;

		//Can hold other kinds of Boxes!
		box null ; box make ;
		"foo" box make ;
		[] box make ;

		//Can even hold itself!
		1.616 box make ; box make ;

		debugPrintStack
		debugPrintHeap
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		MiscBox 0
		MiscBox 1
		MiscBox 2
		MiscBox 3
		MiscBox 5
		MiscBox 7
		MiscBox 9
		--------------------------------
		STACK LENGTH: 7
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		MiscBox 0:
		        isize 42
		MiscBox 1:
		        Char 'A'
		MiscBox 2:
		        Boolean false
		MiscBox 3:
		        NULLBox
		StringBox 4:
		        String "foo"
		MiscBox 5:
		        StringBox 4
		ListBox 6:
		        List []
		MiscBox 7:
		        ListBox 6
		MiscBox 8:
		        f32 1.616
		MiscBox 9:
		        MiscBox 8
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 10
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

- `box open ;`
	- Performance: O(1)
	- Given a stack with a valid MiscBox on top, consumes the MiscBox and pushes its contents to the stack.
	- General form: given stack `m` where `m` is type `MiscBox`, applying `box open ;` yields stack `v` where `v` is any stack-based data type. 
	- This program only works for MiscBoxes and no other boxes because since MiscBoxes are directly allocated, they are directly dereferenced too, where other Box types are implicitly dereferenced in their operation.
	- Be aware that `box open ;` does not give the MiscBox back, even though it's still allocated on the heap, so be sure to save it in a variable or using the `dup` operator.
	- Example Program:

		```
		42usize box make ;
		debugPrintStack

		box open ;
		debugPrintStack
		debugPrintHeap
		```

	- Program Output:

		```
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		MiscBox 0
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		--------------------------------
		BEGIN STACK PRINT
		--------------------------------
		usize 42
		--------------------------------
		STACK LENGTH: 1
		--------------------------------
		END STACK PRINT
		--------------------------------
		////////////////////////////////
		BEGIN HEAP PRINT
		////////////////////////////////
		MiscBox 0:
		        usize 42
		////////////////////////////////
		FREE'D BOX NUMBERS: []
		////////////////////////////////
		FREE'D BOX COUNT: 0
		////////////////////////////////
		TOTAL HEAP ITEM COUNT: 1
		////////////////////////////////
		PERCENT OF HEAP FREE'D: 0.00
		////////////////////////////////
		END HEAP PRINT
		////////////////////////////////
		```

### [**Return to Table of Contents**](#toc)

## <a name = "conclusion"></a> 5 Conclusion 




Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.
Aut soluta alias est quis. Quisquam cum omnis est earum ipsum. Qui occaecati eum aut explicabo aut voluptas. Id labore sit eius. Aut consequuntur officiis omnis et aliquam repudiandae.

Sit quia cum tempora natus commodi deserunt sunt et. Sed unde voluptatibus impedit eaque dolorem quisquam perspiciatis. Ullam voluptas alias praesentium libero assumenda occaecati ut. Nesciunt consectetur sunt enim quia assumenda. In iusto error et delectus officiis.

Voluptatibus architecto sed recusandae laboriosam voluptatem harum voluptates facilis. Quis error fugiat tempore vel perspiciatis laborum voluptatem. Qui est cupiditate repellat autem. Nisi commodi odio est laudantium. In earum cum ipsam voluptas iure ad.

Perspiciatis deserunt voluptas libero voluptatibus et dolorem. Delectus facilis doloremque quaerat. Maxime quia rem provident quia natus velit architecto amet. Eos fuga sit ducimus aliquid voluptatem rerum aspernatur.

Officiis sequi accusamus illo aut aut incidunt iusto. Et sit et blanditiis neque aliquam ut iure. Omnis sed sed quos dolor asperiores voluptate ut veritatis. Officiis qui illum et sed dolores minus distinctio. Et aspernatur numquam illum odit molestiae labore.


### [**Return to Table of Contents**](#toc)