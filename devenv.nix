{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  languages = {
    c.enable = true;
    rust.enable = true;
  };

  packages = with pkgs; [alejandra cargo-audit cargo-deny just jq openssl pkg-config];

  enterTest = ''
    cargo test
    cargo clippy
    cargo deny check
    cargo audit -f ${./Cargo.nix.lock} --json | ${lib.getExe pkgs.jq} -e '. as $expression | $expression, ($expression | .vulnerabilities.found | not)'
  '';

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  outputs = {
    revolut = pkgs.rustPlatform.buildRustPackage {
      name = "revolut";
      cargoLock.lockFile = ./Cargo.nix.lock;
      postPatch = ''
        ln -s ${./Cargo.nix.lock} Cargo.lock
      '';
      src = ./.;
      env = {
        GIT_REVISION = "devenv";
      };
    };
  };
}
