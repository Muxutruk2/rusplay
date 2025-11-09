{ pkgs, ... }:

{
  packages = with pkgs; [
    openssl
    rust-analyzer
    rustPlatform.rustLibSrc
  ];

  env.RUST_LOG = "debug,reqwest=info,hyper=info,cookie_store=info";

  languages.rust = {
    enable = true;

    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };
  git-hooks.hooks = {
    rustfmt.enable = true;
  };
}
