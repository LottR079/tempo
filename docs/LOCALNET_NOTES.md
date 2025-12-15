# Tempo Localnet – Quick Notes

This file complements the main documentation for running a local Tempo network.

## Basic startup

A minimal flow to start a local network:

```bash
git clone https://github.com/tempoxyz/tempo.git
cd tempo
```
# install dependencies (via just)
just

# build all components
just build-all

# start the local network
just localnet
