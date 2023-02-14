# SOS
## Literal

### Variable assignment

```math
\frac{}{ (v, s) \rightarrow v}
```

### Variable lookup

```math
\frac{}{ (x ,s) \rightarrow s(x)}
```

## Block

Several lines of code to be executed withing måsvingar {}

```math
\frac{(B, s) \rightarrow (v, s^\prime) }{ ({S}, s) \rightarrow (v, s^{\prime})}
```

# Binary operations:

## Add (+):

Add B to A (A + B)

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 + v_2, s) \rightarrow v_1^\prime + v_2^\prime} 
```

## Sub (-):

SUBTRACT B From A (A-B)

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 - v_2, s) \rightarrow v_1^\prime - v_2^\prime}
```


## Mul (x):

Multiply A WITH B (A*B)

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 * v_2, s) \rightarrow v_1^\prime * v_2^\prime}
```

## Div (/):

Divide A with B (A/B)

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 / v_2, s) \rightarrow v_1^\prime / v_2^\prime}
```

## Gt (>):

IF A > B, so A = 10, B = 1 would be true. But not A = 5, B = 6.

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 > v_2, s) \rightarrow v_1^\prime > v_2^\prime}
```

## Lt (<):

IF A < B, so A = 5, B = 6 would be true. But not A = 10, B = 1.

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 < v_2, s) \rightarrow v_1^\prime < v_2^\prime}
```

## Eq (==):

If A EQUALS B

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 == v_2, s) \rightarrow v_1^\prime == v_2^\prime}
```

## And (&&):

if both A or B is the same value (true or false), then it is true.

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 \&\& v_2, s) \rightarrow v_1^\prime \&\& v_2^\prime}
```

# Unary operations:

## Or (||):

If A or B is true, then it is true.

```math
\frac{(v_1, s) \rightarrow v_1^\prime \space (v_2, s) \rightarrow v_2^\prime}{ (v_1 || v_2, s) \rightarrow v_1^\prime || v_2^\prime}
```

## Bang (!):

Negate bool. So True => False, False => True

```math
\frac{(v, s) \rightarrow v^\prime}{ (!v, s) \rightarrow !v^\prime}
```

## DeRef (*):

Read value of reference.

```math
\frac{(Ref(v), s) \rightarrow v^\prime}{ (*(Ref(v)), s) \rightarrow v^\prime}
```

## Mut

Allow varaible to be changed (mutable)

```math
\frac{(v, s) \rightarrow v^\prime}{ (v), s) \rightarrow \text{mut } v^\prime}
```

## Ref (&):

Borrow data

```math
\frac{(v, s) \rightarrow Ref(v)^\prime}{ (\&v, s) \rightarrow Ref(v)^\prime}
```


# Statements:

## Assign

Set value of a variable, like a = b, or a = 5 etc

```math
\frac{(v, s) \rightarrow v^\prime \space (E, s) \rightarrow s^\prime}{ (v \space = \space {E}, s) \rightarrow s^\prime}
```

## Function (Fn)

A callable block of code.

```math
\frac{(v, s) \rightarrow v^\prime \space (B, s) \rightarrow s^\prime}{ (\text{fn } v \space {B}, s) \rightarrow s^\prime}
```

## Let

Assign variable.

```math
\frac{(v,s) \rightarrow v^\prime \space (E,s) \rightarrow s^\prime}{ (\text{let } v \space = \space {E},s) \rightarrow s^\prime}
```

## While

While a condition is true loop through this block

```math
\frac{(v, s) \rightarrow v^\prime \space (B, s) \rightarrow s^\prime}{ (\text{while } v \space {B}, s) \rightarrow s^\prime}
```

# Expressions

## If statement

If [condition], if the condition is True, do Then. Else do nada

```math
\frac{(v, s) \rightarrow v^\prime \space (B_1, s) \rightarrow s^\prime}{ (\text{If } v \space {B_1}, s) \rightarrow s^\prime}
```

## If-Else statement

If [condition], if the condition is True, do This, else do that.

```math
\frac{(v,s) \rightarrow v^\prime \space (B_1,s) \rightarrow s^\prime \space (B_2,s) \rightarrow s^{\prime\prime}}{ (\text{If } v \space {B_1} \space else \space {B_2},s) \rightarrow s^{\prime\prime}}
```
## Parentheses

Used for maths, encapsulates a statement. Like (A\*B)^C and A\*B^C

```math
\frac{(E, s) \rightarrow v^\prime}{ ((E), s) \rightarrow v^\prime}
```