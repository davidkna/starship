# Custom WASM Example

This example demonstrates how to create a custom Starship module using WebAssembly components.

## Building

This example uses custom WIT interfaces, so it requires `cargo-component`:

```bash
# Install cargo-component if not already installed
cargo install cargo-component

# Build the component
cargo component build --release
```

The output will be at: `target/wasm32-wasip1/release/custom_wasm_example.wasm`

## Note on wasm32-wasip2 Target

As of Rust 1.82, there is now a native `wasm32-wasip2` target that can produce components without `cargo-component`. However, since this example uses **custom WIT interfaces** (our `starship:wasm-interface`), we need to use `cargo-component`.

If you were only using standard WASI interfaces, you could use the native target:
```bash
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2 --release
```

But for custom WIT like ours, `cargo-component` is required.

## Usage

1. Build the component (see above)

2. Configure in your `starship.toml`:

```toml
format = "$custom_wasm$character"

[custom_wasm]
wasm_path = "~/path/to/custom_wasm_example.wasm"
format = "[$symbol$output]($style) "
symbol = "🦀 "
style = "bold cyan"
```

3. Test with:

```bash
# Build starship
cargo build --release

# Test with the component
./target/release/starship prompt
```

## Implementation

The example implements the `starship:wasm-interface/formatter` interface defined in `wit/starship-wasm.wit`:

- `is-enabled()` - Returns whether the module should be shown (always true in this example)
- `map(variable)` - Maps variables like "output" to their values
- `map-meta(variable)` - Maps meta variables like "symbol"
- `map-style(variable)` - Maps style variables (returns None to use config defaults)

The component has access to:
- Environment variables (via WASI)
- stderr for diagnostics (stdout is not inherited, since stray prints would corrupt the prompt)
- A read-only view of the current directory, mounted at `.`

Execution is bounded by starship's `command_timeout` (unless `ignore_timeout = true`)
and a 64 MB memory limit.

This example reads the `USER` environment variable and displays it in the prompt.
