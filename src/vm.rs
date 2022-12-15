use crate::ast::*;
use crate::common::Eval;
use crate::env::{Env, Ref};
use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Val 
{
    Lit(Literal),
    Ref(Ref),
    UnInit,
    Mut(Box<Val>),
}

// Helpers for Val
// Alternatively implement the TryFrom trait
impl Val {
    pub fn get_bool(&self) -> Result<bool, Error> 
    {
        match self 
        {
            Val::Lit(Literal::Bool(b)) => Ok(*b),
            _ => Err(format!("cannot get Bool from {:?}", self)),
        }
    }

    pub fn get_int(&self) -> Result<i32, Error> 
    {
        match self 
        {
            Val::Lit(Literal::Int(i)) => Ok(*i),
            _ => Err(format!("cannot get integer from {:?}", self)),
        }
    }

    pub fn get_string(&self) -> Result<Literal, Error> 
    {
        match self 
        {
            Val::Lit(s) => Ok(s.clone()),
            _ => Err(format!("cannot get string from {:?}", self)),
        }
    }

}

// Helper for Op
impl Op 
{
    // Evaluate operator to literal
    pub fn eval(&self, left: Val, right: Val) -> Result<Val, Error> 
    {
        use Literal::{Bool, Int};
        match self 
        {
            Op::Add => Ok(Val::Lit(Int(left.get_int()? + right.get_int()?))),
            Op::Sub => Ok(Val::Lit(Int(left.get_int()? - right.get_int()?))),
            Op::Mul => Ok(Val::Lit(Int(left.get_int()? * right.get_int()?))),
            Op::Div => Ok(Val::Lit(Int(left.get_int()? / right.get_int()?))),
            Op::And => Ok(Val::Lit(Bool(left.get_bool()? && right.get_bool()?))),
            Op::Or => Ok(Val::Lit(Bool(left.get_bool()? || right.get_bool()?))),
            Op::Eq => Ok(Val::Lit(Bool(left == right))), // overloading
            Op::Lt => Ok(Val::Lit(Bool(left.get_int()? < right.get_int()?))),
            Op::Gt => Ok(Val::Lit(Bool(left.get_int()? > right.get_int()?))),
        }
    }
}

impl Eval<Val> for Expr 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        match self 
        {
            Expr::Ident(id) => match env.v.get(&id)
            {
                Some(t) => Ok((t, env.v.get_ref(id))),
                None => Err("Variable not found".to_string()),
            },
            Expr::Lit(literal) => 
            {
                Ok((Val::Lit(literal.clone()), None))
            },
            Expr::BinOp(op, left, right) => 
            {
                Ok((op.eval(left.eval(env)?.0, right.eval(env)?.0)?, None))
            }
            Expr::Par(e) => 
            {
                e.eval(env)
            }
            Expr::IfThenElse(c, t, e) => 
            match c.eval(env)?.0.get_bool()? 
            {
                true => (*t).eval(env),
                false => match e 
                {
                    Some(e) => e.eval(env),
                    None => Ok((Val::Lit(Literal::Unit), None)),
                },
            },
            Expr::Block(b) => 
            {
                b.eval(env)
            },
            Expr::Call(id, params) => 
            {
                //Check if the function exists.
                if !env.f.0.contains_key(id)
                {
                    return Err("Missing function".to_string())
                }

                let env_temp = env.clone();
                let _fn = env_temp.f.0.get(id).unwrap();
                if _fn.0.id == "println!"
                {
                    let mut args : Vec<Literal> = Vec::new();
                    for arg in params.0.iter()
                    {
                        args.push(arg.eval(env)?.0.get_string()?);
                    }
                Ok((Val::Lit(_fn.1.unwrap()(args)), None))
                }
                else 
                {
                    env.v.push_scope();
                    let mut i = 0;
                    for arg in params.0.clone()
                    {
                        let arg_id = _fn.0.parameters.0[i].id.clone();
                        let arg_val = arg.eval(env)?.0;
                        env.v.alloc(&arg_id, arg_val.clone());
                        i = i+1;
                    }
                    let b : Block = _fn.0.body.clone();
                    let retval = b.eval(env);
                    env.v.pop_scope();
                    retval
                }
            },
            Expr::UnOp(u, e) => 
            {
                u.eval(*e.clone(), env)
            },
        }
    }
}

impl Eval<Val> for Block 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        env.v.push_scope();
        let mut return_val = Val::Lit(Literal::Unit);
        for be in &self.statements 
        {
            println!("be {:?}", be);
            match be 
            {
                Statement::Let(m, id, _, e) => 
                {
                    // the right hand side, in the "old" env
                    let l: Val;
                    match e
                    {
                        Some(e) => l = e.eval(env)?.0,
                        None => l = Val::UnInit
                    }
                    // the left hand side, for now just accept an ident
                    env.v.alloc(id, l);
                },
                Statement::Assign(id, e) => 
                {
                    // the right hand side, in the "old" env
                    let id_val = id.eval(env)?;
                    let ex = e.eval(env)?;
                    /* for currentenv in env.iter_mut() {
                        if currentenv.0.contains_key(&id.get_id()?){
                            currentenv.0.insert(id.get_id()?, Some(l));
                            break;
                        }
                    } */
                    let err = env.v.get(&id.to_string());
                    if err.is_none()
                    {
                        return Err("That variable has not been declared".to_string());
                    }
                    if id_val.1.is_some()
                    {
                        env.v.set_ref(id_val.1.unwrap(), ex.0);
                    }
                    else
                    {
                        return Err("Expected ref in assignment".to_string());
                    }
                },
                Statement::Expr(e) => 
                {
                    return_val = e.eval(env)?.0;
                },

                Statement::While(c, block) => 
                {
                    while c.eval(env)?.0.get_bool()? 
                    {
                        block.eval(env)?;
                    }
                },
                Statement::Fn(fndecl) => 
                {
                    fndecl.eval(env)?;
                },
            }
        }
        env.v.pop_scope();
        match self.semi 
        {
            true => Ok((Val::Lit(Literal::Unit), None)),
            false => Ok((return_val, None)),
        }
    }
}

impl Eval<Val> for FnDeclaration 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        Ok((Val::Lit(Literal::Unit), None))
    }
}

impl Eval<Val> for Prog 
{
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> 
    {
        for func in self.0.clone()
        {
            func.eval(env)?;
        }
        match env.f.0.get("main") 
        {
            Some(_f) => Err("Ok")?,
            None => Err("Warning, function 'main' not found")?,
        }

    }
}

impl UnOp
{
    fn eval(&self, expr: Expr, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> {
        use Literal::Bool;
        match self 
        {
            UnOp::Bang => 
            {
                Ok((Val::Lit(Bool(!expr.eval(env)?.0.get_bool()?)), None))
            },
            UnOp::DeRef => 
            {
                todo!()
            },
            UnOp::Mut => 
            {
                Ok((Val::Mut(Box::new(expr.eval(env)?.0)), None))
            },
            UnOp::Ref => 
            {
                todo!()
            },
        }
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
