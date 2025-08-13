# Introduction
If you're using Sublime Text, then you're in luck because I do too! 
Because of my bias, I made a syntax highlighter plugin for Lmao. 
I also made a small preference file that allows you to highlight and comment out code
with `Ctrl` `/`! 

In addition, syntax highlighting for Vim also exists! 

# Getting Sublime Syntax Highlighting for Lmao to Work
- Open Sublime Text.
- Go to `Preferences` then to `Browse Packages...`
- Copy paste the two files from `sublime` into that directory.
- Open a `.lmao` file.
- In the bottom right corner, change the language selected from `Plaintext` to `Lmao`
- You should see the code immediately light up with beautiful colors!
- Also, thanks to that other file you copied over, when you highlight code and press `Ctrl`
  and `/`, it will comment out the highlighted code, which is really cool!
- That's about it. Cool stuff.

# Getting Vim Syntax Highlighting for Lmao to Work:
- Copy the file `vim/ftdetect/lmao.vim` to `~/.vim/ftdetect/lmao.vim`
	- This makes Vim detect Lmao files to then apply highlighting to.
- Copy the file `vim/syntax/lmao.vim` to `~/.vim/syntax/lmao.vim` 
	- This makes Vim able to detect patterns for highlighting.
- Append the lines contained in `vim/vimRcAdditions` to the file `~/.vimrc` 
	- This tells Vim what colors to highlight the Lmao patterns.
- That's all! Vim should now have syntax highlighting for Lmao when opening `.lmao` files.
- If it still isn't working, add `syntax on` to the `~/.vimrc` file if it's not there.

# Conclusion
While not super useful, it does make Lmao much easier to code in Vim or Sublime Text since 
you can see errors more easily and the language looks much prettier. Plus, being able
to comment out a bunch of code at once is really nice.

The color schemes used in the Vim version are a work in progress but at least help in distinguishing Lmao's components.

