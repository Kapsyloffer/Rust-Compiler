# Type rules for your Rust in Rust language

See slides Lecture 6 for further details.

Hint: Use inline latex math for GitLab.

# Op

$fn : Add (<a_1:i32>, <a_2:i32>) => i32$

$fn : Sub (<a_1:i32>, <a_2:i32>) => i32$

$fn : Mul (<a_1:i32>, <a_2:i32>) => i32$

$fn : Div (<a_1:i32>, <a_2:i32>) => i32$

$fn : And (<a_1:Bool>, <a_2:Bool>) => Bool$

$fn : Or (<a_1:Bool>, <a_2:Bool>) => Bool$

$fn : Gt (<a_1:i32>, <a_2:i32>) => Bool$

$fn : Lt (<a_1:i32>, <a_2:i32>) => Bool$

<!--$fn : Not (<a_1:Bool>) => Bool$--> 
<!--Not anymore it ain't :(-->

# Expr

$fn : Ident (<a_1:String>) => Type$

$fn : BinOp (<a_1:Expr>, <a_2:Expr>, <a_3:Expr>) => Type$

$fn : IfThenElse (<a_1:Expr>, <a_2:Block>, <a_3:Block>) => Type$

$fn : Par (<a_1:Expr>) => Type$

$fn : Not (<a_1:Expr>) => Type$

# Stmt

$fn : Let (<a_1:Expr>, <a_2:Expr>, <a_3:Expr>) => Unit$

$fn : Assign (<a_1:Expr>, <a_2:Expr>) => Unit$

$fn : Expr (<a_1:Expr>) => Type$

$fn : While (<a_1:Expr>, <a_2:Block>) => Unit$