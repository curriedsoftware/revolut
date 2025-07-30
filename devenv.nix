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

  packages = with pkgs; [alejandra bat cargo-audit cargo-deny just jq openssl pkg-config];

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

  scripts = {
    setup-business-api.exec = ''
      if [ -d .setup-business-api ]; then
        echo "Remove .setup-business-api directory before continuing"
        exit 1
      else
        mkdir .setup-business-api
      fi
      pushd .setup-business-api &> /dev/null
      openssl genrsa -out privatecert.pem 2048
      openssl req -new -x509 -key privatecert.pem -out publiccert.cer -days 1825 -subj "/C=US/ST=State/L=City/O=Organization/OU=Unit/CN=example.com"
      echo "Certificate contents:"
      bat --style=header-filename,grid publiccert.cer
      echo "Upload certificate to the API certificates section:"
      echo "  - Follow: https://developer.revolut.com/docs/guides/manage-accounts/get-started/make-your-first-api-request#upload-your-certificate"
      echo "  - Enter:"
      echo "    - OAuth redirect URI: https://example.com"
      echo -n "Enter provided client ID: "
      read client_id
      echo "Generating client assertion..."
      cat <<EOF > header.json
      {
        "alg": "RS256",
        "typ": "JWT"
      }
      EOF
      cat <<EOF > payload.json
      {
        "iss": "example.com",
        "sub": "$client_id",
        "aud": "https://revolut.com",
        "exp": 1761663836
      }
      EOF
      cat header.json | tr -d '\n' | tr -d '\r' | openssl enc -base64 -A | tr +/ -_ | tr -d '=' > client_assertion.txt
      echo -n "." >> client_assertion.txt
      cat payload.json | tr -d '\n' | tr -d '\r' | openssl enc -base64 -A | tr +/ -_ | tr -d '=' >> client_assertion.txt
      cat client_assertion.txt | tr -d '\n' | tr -d '\r' | openssl dgst -sha256 -sign privatecert.pem | openssl enc -base64 -A | tr +/ -_ | tr -d '=' > sign.txt
      echo -n "." >> client_assertion.txt
      cat sign.txt >> client_assertion.txt

      echo "Client assertion:"
      bat --style=header-filename,grid client_assertion.txt

      popd &> /dev/null
    '';
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
