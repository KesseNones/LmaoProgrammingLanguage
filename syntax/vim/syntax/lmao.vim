"Operators. It's gross but works."
syntax match LmaoOperators /\(debugPrintStack\|debugPrintHeap\|+\|-\|*\|\/\|mod\|pow\|isizeMax\|usizeMax\|i8Max\|i16Max\|i32Max\|i64Max\|i128Max\|u8Max\|u16Max\|u32Max\|u64Max\|u128Max\|swap\|dropStack\|drop\|rot\|dup\|deepDup\|>=\|<=\|==\|!=\|>\|<\|stringCompare\|++\|and\|or\|xor\|not\|push\|pop\|fpush\|fpop\|index\|length\|isEmpty\|clear\|contains\|changeItemAt\|isWhitespaceChar\|isAlphaChar\|isNumChar\|objAddField\|objGetField\|objMutField\|objRemField\|bitOr\|bitAnd\|bitXor\|bitNot\|bitShift\|cast\|printLine\|readLine\|printChar\|readChar\|print\|read\|fileWrite\|fileRead\|fileCreate\|fileRemove\|fileExists\|queryType\|leaveScopeIfTrue\|throwCustomError\|getArgs\|isValidBox\|timeUnixNow\|timeWait\|p\|po\|fp\|fpo\|&&\|||\|!\|len\||\|&\|^\)/

"Function syntax detection" 
syntax match LmaoFunction /\<func\>\s\+\(def\|call\)\s\+[a-zA-Z0-9_]\+/

"Variable highlight."
syntax match LmaoVar /\<var\>\s\+\(mak\|get\|mut\|del\)\s\+[a-zA-Z0-9_]\+\s\+;/

"Local Variable highlight."
syntax match LmaoLocVar /\<loc\>\s\+\(mak\|get\|mut\|del\)\s\+[a-zA-Z0-9_]\+\s\+;/

"Box highlight."
syntax match LmaoBox /\<box\>\s\+\(make\|open\|altr\|free\|null\)\s\+;/

"Cast to highlight."
syntax match LmaoCastTo /\<castTo\>\s\+\(i8\|i16\|i32\|i64\|i128\|u8\|u16\|u32\|u64\|u128\|isize\|usize\|f32\|f64\|Char\|Boolean\|String\|List\)\s\+;/

"Strings" 
syntax match LmaoString /"[^"]*"/

"Char hightlighting"
syntax match LmaoChar /'\(\\.\|.\)'/

"Boolean"
syntax match LmaoBool /\<\(true\|True\|false\|False\)\>/

"Empty Lists/Objects"
syntax match LmaoLsObj /\(\[\]\|{}\)/

"Control keywords" 
syntax match LmaoControlKeys /\<\(if\|else\|while\)\>/

"Numerical Values"
syntax match LmaoNumbers /\(-\)\?[0-9.]\+\(e\(-\)\?[0-9.]\+\)\?\(isize\|usize\|i8\|i16\|i32\|i64\|i128\|u8\|u16\|u32\|u64\|u128\|f32\|f64\)\?/

"Defer/attempt/onError"
syntax match LmaoDefAttErr /\(attempt\|onError\|defer\)/

"Comment syntax"
syntax match LmaoComment /\/\/.*$/

"syntax match LmaoOperators /\<\(debugPrintStack\|debugPrintHeap\|+\|-\|\*\|/\|mod\|%\|\(\<\(i\|u\)\(size|8|16|32|64|128\)Max\>\)\)\>/"
