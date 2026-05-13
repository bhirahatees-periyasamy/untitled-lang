A small arithmetic expression interpreter written in Rust. I built this to get a feel for how interpreters work under the hood — lexing, parsing into an AST, and evaluating it recursively.

It handles the usual stuff: `+`, `-`, `*`, `/`, parentheses for grouping, and named identifiers. Operator precedence works as you'd expect (`*` and `/` bind tighter than `+` and `-`).

```
123 * (2 + 3)  →  615
```

## How it works

The pipeline is pretty straightforward:

1. **Lexer** — walks the input string character by character and emits tokens (`Number`, `Identifier`, `Plus`, `Star`, `LParen`, etc.). Whitespace, newlines, and tabs are skipped.
2. **Parser** — takes those tokens and builds an AST using recursive descent. The `expression → term → factor` grammar handles precedence without any extra machinery. The parser also has a `consume` helper that enforces expected tokens (like the closing `)`) and surfaces a clean error if they're missing.
3. **AST** — three node types: `Literal` for number values, `Identifier` for named variables, and `Binary` for operations. Operators are represented by a `BinaryOperator` enum (`Add`, `Subtract`, `Multiply`, `Divide`) rather than raw tokens, so the evaluator never has to think about syntax.
4. **Evaluator** — recursively walks the AST and computes the result. Divide by zero returns an error. Referencing an identifier that hasn't been defined also returns an error (`Undefined variable: x`) — there's no implicit zero fallback.
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

There are unit tests for each layer (lexer, parser, evaluator, token) covering the happy path and edge cases like divide by zero, missing parentheses, and unexpected characters.

## What's supported

- Integer arithmetic: `+`, `-`, `*`, `/`
- Parenthesized subexpressions: `(2 + 3) * 4`
- Multi-digit numbers: `1234 + 5678`
- Identifiers: `x`, `my_var`, `total123` — they lex and parse correctly; evaluating one returns an `Undefined variable` error until a variable scope is wired in
- Whitespace, newlines, and tabs are ignored

Numbers are `i64` internally, so negative results work fine but there's no float support yet.
