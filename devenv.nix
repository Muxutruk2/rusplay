{ pkgs, ... }:

{
  packages = with pkgs; [ openssl ];

  env.RUST_LOG = "debug,reqwest=info,hyper=info,cookie_store=info";

  languages.rust.enable = true;
  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
