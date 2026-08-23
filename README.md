# TraceHell

TraceHell shows where Rust's `?` operator returns an error.

It creates an instrumented copy of a Rust workspace and reports the exact
question-mark expression that passes an error back to its caller. The original
source stays untouched.

## Why TraceHell?

Rust's `?` operator keeps error-handling code concise, but a chain of propagated
errors can make the first failing expression harder to spot. TraceHell rewrites
those expressions in a separate copy of the workspace so each propagated error
prints useful context.

Example output:

```text
? src/parse.rs:42: file.read_to_string(&mut contents) -> Os { code: 2, kind: NotFound, ... }
```

## Quick start

TraceHell requires Rust with Cargo and one supported terminal: Kitty,
Alacritty, or GNOME Terminal.

```console
# Install from a local checkout
cargo install --path .

# From the Rust project you want to inspect
cargo tracehell create my-trace

# Run a command inside the instrumented copy
cargo tracehell run my-trace cargo run
```

On first use, TraceHell asks which terminal to launch and stores the choice in
`~/TraceHall_settings`. Instrumented copies live under `~/.TraceHell`.

## Commands

| Command | Purpose |
| --- | --- |
| `create <name>` | Copies the current Cargo workspace and instruments its Rust source files. |
| `run <name> <command>` | Opens the named copy in the configured terminal and runs the supplied command. |
| `delete <name>` | Removes the named copy from `~/.TraceHell`. |
| `help` | Prints usage, available commands, examples, and the installed version. The aliases `-help`, `--help`, and `-h` are also accepted. |

Every command accepts a bare name, a single dash, or a double dash. For example:

```console
cargo tracehell create parser-debug
cargo tracehell --run parser-debug cargo test
cargo tracehell -delete parser-debug
cargo tracehell help
```

## How it works

1. **Clone the workspace.** Project files are copied into a named directory
   under `~/.TraceHell`. Build output and Git metadata are skipped.
2. **Rewrite try expressions.** Rust files are parsed with `syn`. Each `?`
   expression becomes a match that prints context before returning an error.
3. **Launch the trace.** The selected terminal opens in the copied workspace
   and runs the supplied command, leaving the shell open afterward.

## What gets reported

When an instrumented expression returns `Err`, TraceHell writes the following
context to standard error:

- The path produced by Rust's `file!()` macro
- The line number of the generated trace point
- A colored `?` marker
- The expression as source text
- The error's debug representation

Trace names must be a single path component. Before launching a command, the
current command filter rejects text containing `rm`, `dd`, `wipefs`, or `sudo`.

## Development

```console
cargo build
cargo test
cargo fmt --check
```

TraceHell is written in Rust 2024 and distributed under the
[Apache License 2.0](LICENSE).
