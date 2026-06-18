{ pkgs, lib, config, inputs, ... }:

let
  # Vendored libchdb.so lives here (gitignored; fetch with
  # scripts/fetch-libchdb.sh). build.rs adds an rpath to it, and we also export
  # LD_LIBRARY_PATH so plain `cargo run`/`cargo test` and rust-analyzer pick it
  # up without depending on the rpath.
  libchdbDir = "${config.devenv.root}/vendor/chdb";
in
{
  packages = with pkgs; [ git curl gnutar gzip ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  env.LD_LIBRARY_PATH = lib.mkForce libchdbDir;

  enterShell = ''
    if [ ! -f "${libchdbDir}/libchdb.so" ]; then
      echo "libchdb.so missing — fetching it (~530MB)..."
      ${config.devenv.root}/scripts/fetch-libchdb.sh
    fi
  '';
}
