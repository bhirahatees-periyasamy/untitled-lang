A small arithmetic expression interpreter written in Rust. I built this to get a feel for how interpreters work under the hood — lexing, parsing into an AST, and evaluating it recursively.

It handles the usual stuff: `+`, `-`, `*`, `/`, and parentheses for grouping. Operator precedence works as you'd expect (`*` and `/` bind tighter than `+` and `-`).

```
123 * (2 + 3)  →  615
```

## How it works

The pipeline is pretty straightforward:

1. **Lexer** — walks the input string character by character and spits out tokens (`Number`, `Plus`, `Star`, `LParen`, etc.)
2. **Parser** — takes those tokens and builds an AST using a recursive descent approach. `expression → term → factor` handles precedence without any extra machinery.
3. **Evaluator** — walks the AST and computes the result. Divide by zero returns an error rather than panicking.
4. **Interpreter** — ties the three steps together into a single `execute(input)` call.

## Running it

You'll need Rust installed. Then:

```sh
cargo run
```

The expression is hardcoded in `main.rs` for now — just swap it out to try different inputs.

To run the tests:

```sh
cargo test
```

There are unit tests for each layer (lexer, parser, evaluator, token) that cover the happy path and a few edge cases like division by zero and unexpected characters.

## What's supported

- Integer arithmetic: `+`, `-`, `*`, `/`
- Parenthesized subexpressions: `(2 + 3) * 4`
- Multi-digit numbers: `1234 + 5678`
- Identifiers: `x`, `my_var`, `total123`
- Whitespace, newlines, and tabs are ignored

Numbers are `i64` internally, so negative results work fine but there's no float support yet. Identifier evaluation is a stub for now — identifiers parse correctly but always resolve to `0`.
