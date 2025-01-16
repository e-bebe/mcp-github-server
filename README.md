[Specification](https://github.com/modelcontextprotocol/specification/tree/main)

## debug

```bash
export GITHUB_PERSONAL_ACCESS_TOKEN=XXXXXX
echo '{"jsonrpc": "2.0", "method": "listTools", "id": 1}' | cargo run
```

## prepare

```bash
cargo build --release
```

## run

```bash
export GITHUB_PERSONAL_ACCESS_TOKEN=XXXXXX
./target/release/mcp-github-server
echo '{"jsonrpc": "2.0", "method": "listTools", "id": 1}' | ./target/release/mcp-github-server
```