# loom

A small expression interpreter written in Rust. I built this to get a feel for how interpreters work under the hood — lexing, parsing into an AST, and evaluating it. It started as a pure arithmetic calculator and has since grown variable bindings and statement-based execution.

It handles the usual arithmetic: `+`, `-`, `*`, `/`, parentheses for grouping, and named variables you can declare with `let`. Operator precedence works as you'd expect (`*` and `/` bind tighter than `+` and `-`).

```
let x = 7
x + 3        →  10
```

## How it works

The pipeline is pretty straightforward:

1. **Lexer** — walks the input string character by character and emits tokens (`Number`, `Identifier`, `Keyword`, `Plus`, `Star`, `LParen`, `Equal`, etc.). Whitespace, newlines, and tabs are skipped.
2. **Parser** — takes those tokens and builds an AST using recursive descent, producing a list of statements. Each statement is either a `let` variable declaration or a bare expression. The `expression → term → factor` grammar handles precedence without any extra machinery. A `consume` helper enforces expected tokens (like the closing `)` or the `=` in a declaration) and surfaces a clean error if they're missing.
3. **AST** — expressions have three node types: `Literal` for number values, `Identifier` for named variables, and `Binary` for operations. Operators are represented by a `BinaryOperator` enum (`Add`, `Subtract`, `Multiply`, `Divide`) rather than raw tokens, so the evaluator never has to think about syntax. Statements are either `VariablesDeclaration`, `Expression`, or `Empty`.
4. **Evaluator** — walks the statements in order, carrying an environment (a `HashMap` of variable bindings). A `let` declaration evaluates its initializer and stores the result; an expression statement is evaluated and its value kept. Evaluating a program returns the value of the last expression, or nothing if the program ends on a declaration. Divide by zero returns an error, and referencing an undefined variable returns `Undefined variable: x` — there's no implicit zero fallback.
5. **Interpreter** — thin wrapper that chains lexer → parser → evaluator into a single `execute(input)` call.

## Running it

You'll need Rust installed. Then:

```sh
cargo run
```

The expression is hardcoded in `main.rs` for now — swap it out to try different inputs.

To run the tests:

```sh
cargo test
```

There are unit tests for each layer (lexer, parser, evaluator, token) covering the happy path and edge cases like divide by zero, missing parentheses, undefined variables, multi-statement programs, and operator precedence.

## What's supported

- Integer arithmetic: `+`, `-`, `*`, `/`
- Parenthesized subexpressions: `(2 + 3) * 4`
- Multi-digit numbers: `1234 + 5678`
- Variable declarations: `let x = 1 + 2`, then reference `x` later
- Multiple statements in a single program, evaluated in order
- Whitespace, newlines, and tabs are ignored

Numbers are `i64` internally, so negative results work fine but there's no float support yet.
