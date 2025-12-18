# Tempo Localnet Cheatsheet

This cheatsheet complements the main README and docs by collecting a few frequently used localnet commands.

## 1. Bootstrap the repository

    git clone https://github.com/tempoxyz/tempo.git
    cd tempo
    just

## 2. Build and run a localnet

Build the node:

    just build-all

Start a localnet:

    just localnet

Stop the localnet (from another terminal):

    pkill -f tempo || true

## 3. Basic health checks

Check that the RPC endpoint is up:

    curl -s http://localhost:8545 \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}'

If you receive a valid JSON-RPC response, your localnet is running.

## 4. Useful next commands

- Run the test suite: `cargo nextest run`
- Format the code: `cargo fmt`
- Run the lints: `cargo clippy`
