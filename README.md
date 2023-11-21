# Meador Language Compiler

Meador is a simple, custom scripting language designed for educational purposes.

## Language Description

Meador is a statically-typed language with a syntax similar to JavaScript and Rust.
It supports variables, functions, and basic control flow structures.

## Supported Features

- Variables: You can declare variables using the `let` keyword.
- Functions: Functions can be declared and called.
- Control Flow: `if`, `else`, `while` loop are supported.
- Operators: Basic arithmetic and comparison operators are supported.
- Error Handling: The compiler provides detailed error messages, including the position and context of the error.

## Grammar

The grammar of the Meador language is defined as follows:
```
program = { statement+ ~ EOI }

int = { ("+" | "-")? ~ ASCII_DIGIT+ }
decimal = @{ int ~ "." ~ ASCII_DIGIT* }
ident = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }

statement = { variable_declaration | if_stmt | while_loop | code_block | function_call_stmt }
variable_declaration = { "let" ~ ident ~ "=" ~ expr ~ ";" }
while_loop = { "while" ~ expr ~ statement }
code_block = { "{" ~ statement* ~ "}"}
if_stmt = { "if" ~ expr ~ statement ~ ("else" ~ statement)? }

expr = { value ~ (bi_operator ~ value)* }
value = { parenthesis | decimal | int | boolean | function_call | ident }
boolean = { "true" | "false" }
parenthesis = { "(" ~ expr ~ ")" }
function_call = { ident ~ "(" ~ expr* ~ ")" }
function_call_stmt = { function_call ~ ";" }
bi_operator = {
    "&&" | "||" |
    "+" | "-" | "*" | "/" | "^" |
    "==" | "<=" | ">=" | "!=" | "<" | ">"
}
```
