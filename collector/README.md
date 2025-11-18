# Rugplay Collector

To use the `collector` program, run `cargo install rusplay_collector` or build it with `cargo build --release`

## Description

The collector reads from a config file, the token and cookies of the users. Then, for each one, checks wether it can claim the login reward. If it can, it collects it and waits 12h, otherwise, it waits the necessary time to get the reward.

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

## Example 

```
DEBUG collector{user=The Important One}: rusplay_collector: Can claim!
DEBUG collector{user=The Original}: rusplay_collector: Can claim!
DEBUG collector{user=The Important One}: rusplay_collector: Successfully claimed reward! Won $1800, New balance: $47200.00, Login streak: 3
DEBUG collector{user=The Important One}: rusplay_collector: Will sleep 11h 59m
DEBUG collector{user=The Original}: rusplay_collector: Successfully claimed reward! Won $1800, New balance: $57790.00, Login streak: 3
DEBUG collector{user=The Original}: rusplay_collector: Will sleep 11h 59m
```
