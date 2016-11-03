# Calculator

## Motivation

This repository is a simple toy project to demonstrate how to write a basic
algebraic interpreter in rust.  This was mostly done as a fun exercise to learn
both rust, and how a Pratt parser works.

## State

It actually works.  This can evaluate basic algebraic expressions with the
following tokens: numbers, +, -, *, (, ).  For example,

```
    assert_eq!(evaluate("(2 * 3) - (14 + 3*2) * ((4))"), Ok(-74));
```

## Structure of the project

- src/lib.rs: main access point, only import other files
- src/tokenizer.rs: tokenizer (convert input string into token iterator)
- src/parser.rs: the most interesting file, this is the parser
- src/interpreter.rs: a really simple calculator.

Note that most of these files have a test submodule, so you can check how it is
supposed to work.