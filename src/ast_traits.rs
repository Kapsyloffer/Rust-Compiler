// Extra traits implemented for AST

use crate::ast::*;
use std::fmt;

#[derive(Debug)]
pub enum VmErr {
    Err(String),
}
// Back-port utility functions/traits for your AST here.
impl Literal {
    pub fn get_int(&self) -> Result<i32, VmErr> {
        match self {
            Literal::Int(i) => Ok(*i),
            _ => Err(VmErr::Err(format!("cannot get integer from {:?}", self))),
        }
    }

    pub fn get_bool(&self) -> Result<bool, VmErr> {
        match self {
            Literal::Bool(b) => Ok(*b),
            _ => Err(VmErr::Err(format!("cannot get Bool from {:?}", self))),
        }
    }
}

impl Expr 
{
    pub fn bin_op(o: Op, left: Expr, right: Expr) -> Self 
    {
        Expr::BinOp(o, Box::new(left), Box::new(right))
    }
}

impl From<Literal> for Expr 
{
    fn from(lit: Literal) -> Self 
    {
        Expr::Lit(lit)
    }
}

impl From<i32> for Expr 
{
    fn from(i: i32) -> Self 
    {
        Expr::Lit(Literal::Int(i))
    }
}

impl From<i32> for Literal 
{
    fn from(i: i32) -> Self 
    {
        Literal::Int(i)
    }
}

impl From<Expr> for Literal 
{
    fn from(e: Expr) -> Self 
    {
        match e 
        {
            Expr::Lit(l) => l,
            _ => unreachable!(),
        }
    }
}

// Display implementation
impl fmt::Display for Op 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let s = match self 
        {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::And => "&&",
            Op::Or => "||",
            Op::Eq => "==",
            Op::Lt => "<",
            Op::Gt => ">",
            //Op::Not => "!",
        };
        write!(f, "{}", s)
    }
}

// Back-port your ast Display traits here
// You may want to re-factor tests into module.
// See e.g., vm.rs

impl fmt::Display for Literal 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let s = match self 
        {
            Literal::Bool(b) => b.to_string(),
            Literal::Int(i) => i.to_string(),
            Literal::Unit => "()".to_string(),
            Literal::String(content) => content.to_string(),
        };
        write!(f, "{}", s)
    }
}

#[test]
fn display_literal() 
{
    println!("{}", Literal::Int(3));
    println!("{}", Literal::Bool(false));
    println!("{}", Literal::Unit);
    assert_eq!(format!("{}", Literal::Int(3)), "3");
    assert_eq!(format!("{}", Literal::Bool(false)), "false");
    assert_eq!(format!("{}", Literal::Unit), "()");
}

impl fmt::Display for Type 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let s = match *self{
            Type::I32 => "i32",
            Type::Bool => "bool",
            Type::Unit => "()",
            Type::String => "str",
            Type::Ref(_) => "ref",
        };
        write!(f, "{}", s)
    }
}

#[test]
fn display_type() 
{
    assert_eq!(format!("{}", Type::I32), "i32");
    assert_eq!(format!("{}", Type::Bool), "bool");
    assert_eq!(format!("{}", Type::Unit), "()");
    assert_eq!(format!("{}", Type::String), "String");
}

impl fmt::Display for UnOp 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        todo!()
    }
}

impl fmt::Display for Expr 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let s = match self
        {
            Expr::Call(_,_) => todo!(),
            Expr::Ident(a) => a.to_owned(),
            Expr::Lit(l) => format!("{}", l),
            Expr::BinOp(op, l, r) => format!("{} {} {}", l, op, r),
            Expr::Par(e) => format!("({})", e),
            Expr::IfThenElse(c, f, e) => format!("if {} {{\n{}}}\n else\n {{ {:?} }}", c, &f, e),
            Expr::Block(_) => unimplemented!(),
            Expr::UnOp(_, _) => unimplemented!(),
            //Expr::Not(c) => format!("!{}", c),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Block 
{
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let mut s = String::new();
        for stm in &self.statements
        {
            s.push_str(&stm.to_string());
        }
        write!(f, "{}", s)
    }
}

impl fmt::Display for Mutable 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        todo!()
    }
}

impl fmt::Display for Parameter 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let _s = format!("{}:: {}", self.id, self.ty);
        write!(f, "{}", _s)
    }
}

impl fmt::Display for Parameters 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        todo!()
    }
}

impl fmt::Display for Arguments 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        todo!()
    }
}

impl fmt::Display for FnDeclaration 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        todo!()
    }
}

impl fmt::Display for Prog 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        todo!()
    }
}

impl fmt::Display for Statement 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let s = match self
        {
            Statement::Let(_mut, ex1, _type, ex2) => 
            {
                let t = match _type 
                {
                    Some(t) => format!(": {}", t),
                    None => "".to_string(),
                };
                let re = match ex2 
                {
                    Some(re) => format!(" = {}", ex2.as_ref().unwrap()),
                    None => "".to_string(),
                };

                format!("let {}{}{};", ex1, t, re)
            },
            Statement::Expr(e) => e.to_string(),
            Statement::Assign(expr1, expr2) => format!("{expr1} = {expr2};"),
            Statement::While(expr, block) => format!("while {expr} {{{block}}};"),
            Statement::Fn(decl) => 
            {
                todo!()
            },
        };

        write!(f, "{}\n", s)
    }
}

#[test]
fn display_if_then_else() {
    let ts: proc_macro2::TokenStream = "
    if a {
        let a : i32 = false;
        0
    } else {
        if a == 5 { b = 8 };
        while b {
            e;
        };
        b
    }
    "
    .parse()
    .unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("ast:\n{:?}", e);

    println!("pretty:\n{}", e);
}

#[test]
fn display_while() {
    let ts: proc_macro2::TokenStream = "
    while a == 9 {
        let b : i32 = 7;
    }
    "
    .parse()
    .unwrap();
    let e: Statement = syn::parse2(ts).unwrap();
    println!("ast:\n{:?}", e);

    println!("pretty:\n{}", e);
}

#[test]
fn display_expr() {
    println!("{}", Expr::Ident("a".to_string()));
    println!("{}", Expr::Lit(Literal::Int(7)));
    println!("{}", Expr::Lit(Literal::Bool(false)));
    let e = Expr::BinOp(
        Op::Add,
        Box::new(Expr::Ident("a".to_string())),
        Box::new(Expr::Lit(Literal::Int(7))),
    );
    println!("{}", e);
    assert_eq!(format!("{}", e), "a + 7");
}

// As you see it becomes cumbersome to write tests
// if you have to construct the Expr by hand.
//
// Instead we might use our parser

#[test]
fn parse_display_expr() {
    let ts: proc_macro2::TokenStream = "a + 7".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}

// This one will fail (Display for `if` is not yet implemented).
// Implement it as an optional assignment
//
// Hint: You need to implement Display for Statement and Block

#[test]
fn parse_display_if() {
    let ts: proc_macro2::TokenStream = "if a > 5 {5}".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}
