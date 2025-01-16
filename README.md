[Specification](https://github.com/modelcontextprotocol/specification/tree/main)

```bash
export GITHUB_PERSONAL_ACCESS_TOKEN=XXXXXX
cargo run
```

## prepare

```bash
export GITHUB_PERSONAL_ACCESS_TOKEN=XXXXXX
cargo build --release
./target/release/mcp-github-server
echo '{"jsonrpc": "2.0", "method": "listTools", "id": 1}' | ./target/release/mcp-github-server
```
