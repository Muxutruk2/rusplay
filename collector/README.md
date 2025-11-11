# Rugplay Collector

To use the `collector` program, run `cargo install rusplay_collector` or build it with `cargo build --release`

## Usage

```
Helper program to automatically collect rewards in multiple rugplay clients

Usage: rusplay_collector <TOKEN_FILE>

Arguments:
  <TOKEN_FILE>
          TOML File containing the name, tokens and cookies of each Rugplay client

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

The program parses a toml file with this format:

```
[[tokens]]
name = "Client 1"
api_key = "rgpl_xxxxxxxxxxxxxxxx"
cookie = "__Secure-better-auth.session_token=xxxxxxxxxxxxxxxxxxxx"

[[tokens]]
name = "Client 2"
api_key = "rgpl_xxxxxxxxxxxxxxxx"
cookie = "__Secure-better-auth.session_token=xxxxxxxxxxxxxxxxxxxx"
```

Then, it tries to claim the reward for all of the clients concurrently, and waits for each client the necessary time to claim the reward. Runs infinitely.
