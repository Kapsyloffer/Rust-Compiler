use crate::ast::*;
use crate::ast_traits::*;
use crate::common::Eval;
use crate::env::{Env, Ref};
use crate::error::Error;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Val { //Value
    Lit(Literal),
    Ref(Ref),
    UnInit,
    Mut(Box<Val>),
}

type VarEnv = VecDeque<HashMap<String, (Option<Literal>, Option<FnDeclaration>)>>;

// Helpers for Val
// Alternatively implement the TryFrom trait
impl Val {
    pub fn get_bool(&self) -> Result<bool, Error> {
        match self {
            Val::Lit(Literal::Bool(b)) => Ok(*b),
            _ => Err(format!("cannot get Bool from {:?}", self)),
        }
    }

    pub fn get_int(&self) -> Result<i32, Error> {
        match self {
            Val::Lit(Literal::Int(i)) => Ok(*i),
            _ => Err(format!("cannot get integer from {:?}", self)),
        }
    }
}

// Helper for Op
impl Op 
{
    // Evaluate operator to literal
    pub fn eval(&self, left: Literal, right: Literal) -> Result<Literal, VmErr> 
    {
        use Literal::{Bool, Int};
        match self 
        {
            Op::Add => Ok(Int(left.get_int()? + right.get_int()?)),
            Op::Sub => Ok(Int(left.get_int()? - right.get_int()?)),
            Op::Mul => Ok(Int(left.get_int()? * right.get_int()?)),
            Op::Div => Ok(Int(left.get_int()? / right.get_int()?)),
            Op::And => Ok(Bool(left.get_bool()? && right.get_bool()?)),
            Op::Or => Ok(Bool(left.get_bool()? || right.get_bool()?)),
            Op::Eq => Ok(Bool(left == right)), // overloading
            Op::Lt => Ok(Bool(left.get_int()? < right.get_int()?)),
            Op::Gt => Ok(Bool(left.get_int()? > right.get_int()?)),
        }
    }
   /* pub fn notEval(&self, arg:Literal) -> Result<Literal, VmErr>
    {
        match self
        {
            //Negates the bool of argument
            //Op::Not => Literal::Bool(!arg.get_bool()),
            Op::Not => Ok(Literal::Bool(!arg.get_bool()?)),
            _=> unimplemented!(),
        }
    }*/ 
}


impl Expr {
    pub fn get_id(&self) -> Result<String, VmErr> {
        match self {
            Expr::Ident(s) => Ok(s.to_owned()),
            _ => Err(VmErr::Err(format!("cannot get id from {:?}", self))),
        }
    }

    pub fn eval(&self, env: &mut VarEnv) -> Result<Literal, VmErr> {
        match self {
            Expr::Ident(id) => {
                let l : Option<Literal> = None;
                for cur_env in env.iter_mut() 
                {
                    if cur_env.contains_key(id)
                    {
                        //l = cur_env.get(id).unwrap().0;
                        todo!()
                    }
                }
                if l.is_some()
                {
                    Ok(l.unwrap())
                }
                else
                {
                    Err(VmErr::Err("Variable not found".to_string()))
                }
            }
            Expr::Lit(literal) => Ok(literal.clone()),
            Expr::BinOp(op, left, right) => op.eval(left.eval(env)?, right.eval(env)?),
            Expr::Par(e) => e.eval(env),
            Expr::IfThenElse(c, t, e) => match c.eval(env)?.get_bool()? {
                true => (*t).eval(env),
                false => match e {
                    Some(e) => e.eval(env),
                    None => Ok(Literal::Unit),
                },
            },
            //Expr::While(_case, _block) => unimplemented!(),
            //Expr::Not(e) => Ok(Literal::Bool(!e.eval(env)?.get_bool()?)),
            Expr::Call(_id, _params) => 
            { 
                todo!()
            },

            Expr::Block(_) => 
            { 
                todo!()
            },

            Expr::UnOp(_, _) => 
            { 
                todo!()
            },
        }
    }
}

impl Block {
    pub fn eval(&self, env: &mut VarEnv) -> Result<Literal, VmErr> {
        // let mut env = env.clone();
        let abc : HashMap<String, (Option<Literal>, Option<FnDeclaration>)> = HashMap::new();
        env.push_front(abc);
        let mut return_val = Literal::Unit;
        for be in &self.statements {
            println!("be {:?}", be);
            match be {
                Statement::Fn(dec) => 
                {
                    let id = dec.id.to_string();
                    let _eval = dec.body.eval(env)?;
                    let cur_env = env.get_mut(0).unwrap();
                    if cur_env.contains_key(&id)
                    {
                        return Err(VmErr::Err("Duplicate function".to_string()));
                    }
                    else 
                    {
                        cur_env.insert(id, (None, Some(dec.clone())));
                    }
                },
                #[allow(unused_assignments)]
                Statement::Let(_mut, id, _, e) => {
                    // the right hand side, in the "old" env
                    let l :Option<Literal>;
                    match e
                    {
                        Some(e) => l = Some(e.eval(env)?),
                        None => l = None
                    }
                    // the left hand side, for now just accept an ident
                    let cur_env = env.get_mut(0).unwrap(); //Errors here, idk why
                    //cur_env.insert(id.get_id()?, (Some(l.unwrap()), None));
                },

                Statement::Assign(id, e) => {
                    let l = e.eval(env)?;
                   // the right hand side, in the "old" env
                    for (i, cur_env) in env.clone().iter_mut().enumerate() 
                    {
                        if cur_env.contains_key(&id.get_id()?) 
                        {
                            env[i].insert(id.get_id()?, (Some(l), None));
                            break;
                        }
                    } //Thank you, Huber.
                },

                Statement::Expr(e) => 
                {
                    return_val = e.eval(env)?;
                },

                Statement::While(c, block) => {
                    while c.eval(env)?.get_bool()? {
                        block.eval(env)?;
                    }
                },
            }
        }
        match self.semi {
            true => Ok(Literal::Unit),
            false => Ok(return_val),
        }
    }
}

impl Eval<Val> for Expr {
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        match self
        {
            Expr::BinOp(_, _, _) => { todo!()},
            Expr::Block(_) => { todo!()},
            Expr::Call(_, _) => { todo!()},
            Expr::Ident(_) => { todo!()},
            Expr::IfThenElse(_, _, _) => { todo!()},
            Expr::Lit(_) => { todo!()},
            Expr::Par(_) => { todo!()},
            Expr::UnOp(_, _) => { todo!()},
         }
    }
}

impl Eval<Val> for Block 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        todo!("not implemented {:?}", self)
    }
}

impl Eval<Val> for FnDeclaration 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        env.f.add_functions_unique(vec![self.clone()])?; 
        Ok((Val::Lit(Literal::Unit),None))
    }
}

impl Eval<Val> for Prog 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        todo!("not implemented {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::Val;
    use crate::ast::Literal;
    use crate::ast::{Block, Prog};
    use crate::common::parse_test;

    #[test]
    fn test_block_let() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a: i32 = 1;
        let b: i32 = 2;

        a + b
    }",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 3);
    }

    #[test]
    fn test_block_let_shadow() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a: i32 = 1;
        let b: i32 = 2;
        let a: i32 = 3;
        let b: i32 = 4;

        a + b
    }",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 7);
    }

    #[test]
    fn test_block_assign() {
        let v = parse_test::<Block, Val>(
            "
    {
        let mut a: i32 = 1;
        a = a + 2;
        a
    }",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 3);
    }

    #[test]
    fn test_expr_if_then_else() {
        let v = parse_test::<Block, Val>(
            "
    {
        let mut a: i32 = 1;
        a = if a > 0 { a + 1 } else { a - 2 };
        a
    }",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_expr_if_then_else2() {
        let v = parse_test::<Block, Val>(
            "
    {
        let mut a: i32 = 1;
        a = if a < 0 { a + 1 } else { a - 2 };
        a
    }",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), -1);
    }

    #[test]
    fn test_ref_deref() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        let b = &a;
        *b
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }

    #[test]
    fn test_ref_deref_indirect() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        let b = &a;
        let c = b;
        *c
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }

    #[test]
    fn test_deref_assign() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        let b = &a;
        *b = 7;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 7);
    }

    #[test]
    fn test_while() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 2;
        let b = 0;
        while a > 0 {
            a = a - 1;
            b = b + 1;
        }
        b
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_while_ref() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 2;
        let b = 0;
        let c = &b;
        while a > 0 {
            a = a - 1;
            *c = (*c) + 1;
        }
        *c
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_while_ref2() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 2;
        let b = 0;
        let c = &b;
        let d = &a;
        
        while (*d) > 0 {
            *d = (*d) - 1;
            *c = (*c) + 1;
        }
        *c
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_bool() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = true && false;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_bool().unwrap(), false);
    }

    #[test]
    fn test_bool_bang() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = true && !false;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_bool().unwrap(), true);
    }

    #[test]
    fn test_bool_bang2() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = (!true) && false;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_bool().unwrap(), false);
    }

    #[test]
    fn test_local_block() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        { 
            let b = &a;
            *b = 2;
        };
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_local_block_assign() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 6;
        let b = { 
            let b = &a;
            *b = (*b) + 1;
            *b
        };
        b
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 7);
    }

    #[test]
    fn test_prog() {
        let v = parse_test::<Prog, Val>(
            "
    fn main() {
        let a = 1;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }

    #[test]
    fn test_local_fn() {
        let v = parse_test::<Prog, Val>(
            "
    fn main() {
        fn f(i: i32, j: i32) -> i32 {
            i + j
        }
        let a = f(1, 2);
        println!(\"a = {} and another a = {}\", a, a);
    }
    ",
        );

        assert_eq!(v.unwrap(), Val::Lit(Literal::Unit));
    }

    #[test]
    fn test_check_if_then_else_shadowing() {
        let v = parse_test::<Block, Val>(
            "
        {
            let a: i32 = 1 + 2; // a == 3
            let a: i32 = 2 + a; // a == 5
            if true {
                a = a - 1;      // outer a == 4
                let a: i32 = 0; // inner a == 0
                a = a + 1       // inner a == 1
            } else {
                a = a - 1
            };
            a   // a == 4
        }
        ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 4);
    }
    #[test]
    fn test_ref() {
        let v = parse_test::<Block, Val>(
            "
        {
            let a = &1;
            *a
        }
        ",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }
}
