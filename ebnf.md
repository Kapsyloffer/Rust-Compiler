# EBNF Grammar

---


## Datatypes

digits = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0";

type = "i32" | "bool" | "()";

integer = [ "-" ], {digits};

bool = "true" | "false" ;

---

## Expressions

literal = integer | bool;

op = "-" | "+" | "*" | "/" | "&&" | "||" | "<" | ">" | "==" | "!" ;

binop = expr, op, expr;

par = "(", expr, ")";

if_then_else = "if", expr, block, ["else", block];

expr = literal | binop | par | if_then_else;

---

## Statements

let = "let", expr, ":", type, "=", expr;

while = "while", expr, block;

assign = expr, "=", expr;

statement = (let | assign | while | expr);

## Blocks

block = "{", statement, "}", ";";
