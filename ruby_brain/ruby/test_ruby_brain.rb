require 'fiddle'
library = Fiddle::dlopen('../target/release/libruby_brain.dylib')
Fiddle::Function.new(library['initialize'], [], Fiddle::TYPE_VOIDP).call

code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
code.execute
