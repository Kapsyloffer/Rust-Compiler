use syn::token::Else;

use crate::ast::*;
use crate::common::Eval;
use crate::env::{Env, Ref};
use crate::error::Error;
#[allow(unused_imports)]
use std::convert::{From, Into};
use std::fmt::Debug;

type TypeErr = String;

// type check
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Lit(Type),
    Ref(Ref),
    Mut(Box<Ty>),
}

// Helpers for Ty
impl From<&Literal> for Ty {
    fn from(t: &Literal) -> Self {
        Ty::Lit(match *t {
            Literal::Bool(_) => Type::Bool,
            Literal::Int(_) => Type::I32,
            Literal::String(_) => Type::String,
            Literal::Unit => Type::Unit,
        })
    }
}

// Helper for Op
impl Op {
    // Evaluate operator to literal
    pub fn unify(&self, l: Ty, r: Ty) -> Result<(Ty, Option<Ref>), Error> 
    {
        match self 
        {
            Op::Add => unify(l, r, Ty::Lit(Type::I32)),
            Op::Sub => unify(l, r, Ty::Lit(Type::I32)),
            Op::Mul => unify(l, r, Ty::Lit(Type::I32)),
            Op::Div => unify(l, r, Ty::Lit(Type::I32)),
            Op::And => unify(l, r, Ty::Lit(Type::Bool)),
            Op::Or => unify(l, r, Ty::Lit(Type::Bool)),
            Op::Eq => unify(l, r, Ty::Lit(Type::Bool)),
            Op::Lt => unify(l, r, Ty::Lit(Type::Bool)),
            Op::Gt => unify(l, r, Ty::Lit(Type::Bool)),
            //Op::Not => todo!(),
        }
    }
}


// General unification
fn unify(got: Ty, expected: Ty, result: Ty) ->Result<(Ty, Option<Ref>), Error> {
    match got == expected 
    {
        true => Ok((result.into(), None)),
        false => Err(format!("expected type {:?}, got type {:?}", expected, got)),
    }
}


impl Eval<Ty> for Expr 
{
    fn eval(&self, env: &mut Env<Ty>) -> Result<(Ty, Option<Ref>), Error> 
    {
        match self
        {
            Expr::BinOp(op, l, r) => 
            {
                let l_type = l.eval(env)?;
                let r_type = r.eval(env)?;
                let optype = op.unify(l_type.0, r_type.0)?;
                Ok(optype)
            },
            Expr::Block(b) => 
            {
                b.eval(env)
            },
            Expr::Call(id, args) => 
            {
                let f = env.f.0.get(id).unwrap().clone();
                let mut i = 0;
                for param in f.0.parameters.0.clone() 
                {
                    let a = args.0.get(i).clone();
                    let arg;
                    if a != None
                    {
                        arg = a.unwrap();
                    }
                    else
                    {
                        break;
                    }
                    let arg = arg.eval(env)?.0;
                    if Ty::Lit(param.ty.clone()) != arg.clone() {
                        //Throw error since arg and param types don't match
                        return unify(Ty::Lit(param.ty), arg.clone(), arg)
                    }
                    i = i+1;
                }

                if f.0.ty.is_some()
                {
                    Ok((Ty::Lit(f.0.ty.unwrap()), None))
                }
                else
                {
                    Ok((Ty::Lit(Type::Unit), None))
                }
            },
            Expr::Ident(id) => match env.v.get(&id) 
            {
                Some(t) => Ok((t, None)),
                None => Err("variable not found".to_string()),
            },
            Expr::IfThenElse(cond, t, _else) => 
            {
                let cond_t = cond.eval(env)?;
                let do_t = t.eval(env)?;
                unify(cond_t.0, Ty::Lit(Type::Bool), Ty::Lit(Type::Bool))?;
                
                if _else.is_none()
                {
                    Ok((Ty::Lit(Type::Unit), None))
                }
                else 
                {
                    let e_type = _else.as_ref().unwrap().eval(env)?; //This, this is the way, this is the way I wanna live
                    unify(do_t.0.clone(), e_type.0.clone(), do_t.0.clone())?;
                    Ok((Ty::Lit(Type::Unit), None))
                }
            },
            Expr::Lit(Literal::Bool(_)) => 
            {
                Ok((Ty::Lit(Type::Bool), None))
            },
            Expr::Lit(Literal::Int(_)) => 
            {
                Ok((Ty::Lit(Type::I32), None))
            },
            Expr::Lit(Literal::String(_)) => 
            {
                Ok((Ty::Lit(Type::String), None))
            },
            Expr::Lit(Literal::Unit) => 
            {
                Ok((Ty::Lit(Type::Unit), None))
            },
            /*Expr::Not(e) => 
            {
                todo!()
            },*/
            Expr::Par(e) =>
            {
                e.eval(env)
            },
            Expr::UnOp(u, e) => 
            {
                todo!()
            },
        }
    }
}

impl Eval<Ty> for Block 
{
    fn eval(&self, env: &mut Env<Ty>) -> Result<(Ty, Option<Ref>), Error> 
    {
        env.v.push_scope();

        let mut return_ty = (Ty::Lit(Type::Unit), None);
        for stmt in &self.statements 
        {
            // update the return type for each iteration
            return_ty = stmt.eval(env)?;
        }
        env.v.pop_scope();
        if self.semi
        {
            Ok((Ty::Lit(Type::Unit), None))
        }
        else
        {
            Ok(return_ty)
        }
    }
}

impl Eval<Ty> for FnDeclaration 
{
    fn eval(&self, env: &mut Env<Ty>) -> Result<(Ty, Option<Ref>), Error> 
    {
        if self.ty.is_none()
        {
            Ok((Ty::Lit(Type::Unit), None))
        } 
        else 
        {
            Ok((Ty::Lit(self.ty.clone().unwrap()), None))
        }
    }
}

impl Eval<Ty> for Prog 
{
    fn eval(&self, env: &mut Env<Ty>) -> Result<(Ty, Option<Ref>), Error> 
    {
        env.f.add_functions_unique(self.0.clone());
        for _f in self.0.clone()
        {
            _f.eval(env)?;
        }
        match env.f.0.get("main")
        {
            Some(_f) => Err("Ok")?,
            None => Err("Main not found")?,
        }
    }
}

impl Eval<Ty> for Statement
{
    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn eval(&self, env: &mut Env<Ty>) -> Result<(Ty, Option<Ref>), Error> 
    {
        Ok
        (
            match self
            {
                Statement::Assign(id, e) =>
                {
                    let id_type = id.eval(env)?.0;
                    let m = id.eval(env)?.1;
                    let ty: Option<Ty> = env.v.get(&id.to_string());
                    if m.is_none()
                    {
                        //
                    }
                    else
                    {
                        match env.v.de_ref(m.unwrap()) 
                        {
                            Ty::Mut(_) => {},
                            Ty::Lit(Type::Ref(_)) => return Err("Can't assign to Reference".to_string()),
                            _ => return Err("Can't assign to none mutable".to_string())
                        }
                    }
                    let e_type = e.eval(env)?; //Angels crying
                    match ty.unwrap() 
                    {
                        Ty::Lit(Type::Unit) => 
                        { 
                            match id 
                            {
                                Expr::Ident(key) => {env.v.alloc(&key, e_type.0);},
                                _ => unreachable!()
                            }
                        },
                        _ => 
                        {
                            let res1 = id.eval(env);
                            let res2 = e.eval(env);
                            if res1.is_err() || res2.is_err() || unify(res1.clone()?.0, res2?.0, res1.clone()?.0).is_err() 
                            {
                                return Err("Error in assignment".to_string())
                            }
                        },
                    }
                    (Ty::Lit(Type::Unit), None)
                },
                Statement::Expr(e) =>
                {
                    let mut _type = e.eval(env)?;
                    _type.0 = match _type.0
                    {
                        Ty::Mut(b) => *b,
                        _=> _type.0,
                    };
                    _type
                },
                Statement::Fn(decl) =>
                {
                    decl.eval(env)?
                },
                Statement::Let(m, id, t, e) =>
                {
                    let e_val : Ty;
                    if e.is_some()
                    {
                        e_val = e.as_ref().unwrap().eval(env)?.0;
                    }
                    else
                    {
                        e_val = Ty::Lit(Type::Unit);
                    }

                    match (e, t)
                    {
                        (Some(e), Some(t)) =>
                        {
                            if unify(e_val.clone(),Ty::Lit((*t).clone()), Ty::Lit((*t).clone())).is_err() 
                            {
                                return Err("Missmatching types in let-statement".to_string())
                            }

                            if m.0 
                            {
                                env.v.alloc(&id, Ty::Mut(Box::new(e_val.clone())));
                            }
                            else
                            {
                                env.v.alloc(&id, e_val);
                            }
                        }
                        (Some(e), None) =>
                        {
                            if m.0 
                            {
                                env.v.alloc(&id, Ty::Mut(Box::new(e_val.clone())));
                            }
                            else
                            {
                                env.v.alloc(&id, e_val.clone());
                            }
    
                        }
                        (None, Some(t)) =>
                        {
                            if m.0 
                            {
                                env.v.alloc(&id, Ty::Mut(Box::new(Ty::Lit((*t).clone()))));
                            }
                            else
                            {
                                env.v.alloc(&id, Ty::Lit((*t).clone()));
                            }
    
                        }
                        (None, None) =>
                        {
                            if m.0 
                            {
                                env.v.alloc(&id, Ty::Mut(Box::new(Ty::Lit(Type::Unit))));
                            }
                            else
                            {
                                env.v.alloc(&id, Ty::Lit(Type::Unit));
                            }
                        }
                    }
                    (Ty::Lit(Type::Unit), None)
                },
                Statement::While(e, b) =>
                {
                    let cond_t = e.eval(env)?;
                    let do_t = b.eval(env)?;

                    if unify(cond_t.0, Ty::Lit(Type::Bool), Ty::Lit(Type::Bool)).is_ok()
                    {
                        (Ty::Lit(Type::Unit), None)
                    }
                    else
                    {
                        return Err("Error message".to_string())
                    }
                },
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::Ty;
    use crate::ast::{Block, Prog, Type};
    use crate::common::parse_test;

    #[test]
    fn test_block_let() {
        let v = parse_test::<Block, Ty>(
            "
    {
        let a: i32 = 1;
        let b: i32 = 2;

        a + b
    }",
        );
        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_block_let_shadow() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a: i32 = 1;
            let b: i32 = 2;
            let a: i32 = 3;
            let b: i32 = 4;

            a + b
        }",
        );
        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_block_assign() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let mut a: i32 = 1;
            a = 1 + 2;
            a
        }",
        );
        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_expr_if_then_else() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let mut a: i32 = 1;
            a = if a > 0 { a + 1 } else { a - 2 };
            a
        }",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_expr_if_then_else_bool() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let mut a: bool = false;
            a = if a || false { a || false } else { a && true };
            a
        }",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::Bool));
    }

    #[test]
    fn test_ref_deref() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a: i32 = 1;
            let b: &i32 = &a;
            *b
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_ref_deref_err() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a: i32 = 1;
            let b: &bool = &a;
            *b
        }
        ",
        );

        assert_eq!(v.is_err(), true);
    }

    #[test]
    fn test_ref_deref_indirect() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 1;
            let b = &a;
            let c = b;
            *c
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_ref_deref_indirect2() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 1;
            let b = &a;
            let c = &b;
            **c
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_deref_assign_err() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 1;
            let b = &a;
            *b = false;
            a
        }
        ",
        );

        assert_eq!(v.is_err(), true);
    }

    #[test]
    fn test_deref_assign() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 1;
            let b = &a;
            *b = 7;
            a
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_while_err() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 2;
            let b = false;
            while a > 0 {
                a = a - 1;
                b = b + 1;
            }
            b
        }
        ",
        );

        assert_eq!(v.is_err(), true);
    }

    #[test]
    fn test_while() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 2;
            let b = 1;
            while a > 0 {
                a = a - 1;
                b = b + 1;
            }
            b
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }
    #[test]
    fn test_while_ref() {
        let v = parse_test::<Block, Ty>(
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

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_while_ref2() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 2;
            let b = 0;
            let c = &b;
            let d = &a;

            while (*d) > 0 {
                *d = (*d) - 1;
                // not sure if this is even allowed in Rust
                *&*c = (*c) + 1;
            }
            *c
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_bool() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = true && false;
            a
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::Bool));
    }

    #[test]
    fn test_bool_bang() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = true && !false;
            a
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::Bool));
    }

    #[test]
    fn test_bool_bang2() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = (!true) && false;
            a
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::Bool));
    }

    #[test]
    fn test_local_block() {
        let v = parse_test::<Block, Ty>(
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

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_local_block_assign() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = 6;
            let b = {
                let b : &i32 = &a;
                *b = (*b) + 1;
                *b
            };
            b
        }
        ",
        );

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_prog_fn_sig() {
        let v = parse_test::<Prog, Ty>(
            "
        fn a(i: i32, bo: bool) -> i32 {
            let q = 0;
            fn b(j: i32) -> i32 {
                a(j, c())
            }

            fn c() -> bool {
                false
            }

            b(1 + i);
            a(i, bo)
        }
        ",
        );
        println!("v {:?}", v);
        assert_eq!(v.is_err(), true);
    }

    #[test]
    fn test_prog_fn_defined_twice() {
        let v = parse_test::<Prog, Ty>(
            "
        fn a() {
        }

        fn b() {
            fn b() {

            }

        }
        ",
        );
        println!("v {:?}", v);
        assert_eq!(v.is_err(), true);
    }

    #[test]
    fn test_prog() {
        let v = parse_test::<Prog, Ty>(
            "
        fn main() {
            let a = 1;
            a;
        }
        ",
        );
        println!("v {:?}", v);
        assert_eq!(v.unwrap_err(), "Ok");
    }

    #[test]
    fn test_local_fn() {
        let v = parse_test::<Prog, Ty>(
            "
        fn main() {
            fn f(i: i32, j: i32) -> i32 {
                i + j
            }
            let a = f(1, 2);
            // println!(\"a = {} and another a = {}\", a, a);
        }
        ",
        );
        println!("v {:?}", v);
        assert_eq!(v.unwrap_err(), "Ok");
    }

    #[test]
    fn test_check_if_then_else_shadowing() {
        let v = parse_test::<Block, Ty>(
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

        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }

    #[test]
    fn test_ref() {
        let v = parse_test::<Block, Ty>(
            "
        {
            let a = &1;
            *a
        }
        ",
        );
        assert_eq!(v.unwrap(), Ty::Lit(Type::I32));
    }
}
