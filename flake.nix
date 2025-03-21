{
  description = "Espresso Decentralized Sequencer";

  nixConfig = {
    extra-substituters = [
      "https://espresso-systems-private.cachix.org"
      "https://nixpkgs-cross-overlay.cachix.org"
    ];
    extra-trusted-public-keys = [
      "espresso-systems-private.cachix.org-1:LHYk03zKQCeZ4dvg3NctyCq88e44oBZVug5LpYKjPRI="
      "nixpkgs-cross-overlay.cachix.org-1:TjKExGN4ys960TlsGqNOI/NBdoz2Jdr2ow1VybWV5JM="
    ];
  };

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.nixpkgs-cross-overlay = {
    url = "github:alekseysidorov/nixpkgs-cross-overlay";
    inputs.flake-utils.follows = "flake-utils";
  };

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.foundry.url = "github:shazow/foundry.nix/monthly"; # Use monthly branch for permanent releases
  inputs.solc-bin.url = "github:EspressoSystems/nix-solc-bin";

  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";


  outputs = { self, nixpkgs, rust-overlay, nixpkgs-cross-overlay, flake-utils, pre-commit-hooks, fenix, foundry, solc-bin, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # node=error: disable noisy anvil output
        RUST_LOG = "info,libp2p=off,isahc=error,surf=error,node=error";
        RUST_BACKTRACE = 1;
        RUSTFLAGS = " --cfg async_executor_impl=\"async-std\" --cfg async_channel_impl=\"async-std\"";

        overlays = [
          (import rust-overlay)
          foundry.overlay
          solc-bin.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        crossShell = { config }:
          let
            localSystem = system;
            crossSystem = { inherit config; useLLVM = true; isStatic = true; };
            pkgs = import "${nixpkgs-cross-overlay}/utils/nixpkgs.nix" {
              inherit overlays localSystem crossSystem;
            };
          in
          import ./cross-shell.nix {
            inherit pkgs;
            inherit RUST_LOG RUST_BACKTRACE RUSTFLAGS;
          };
      in
      with pkgs;
      {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              doc = {
                enable = true;
                description = "Generate figures";
                entry = "make doc";
                types_or = [ "plantuml" ];
                pass_filenames = false;
              };
              cargo-fmt = {
                enable = true;
                description = "Enforce rustfmt";
                entry = "cargo fmt --all";
                types_or = [ "rust" "toml" ];
                pass_filenames = false;
              };
              cargo-sort = {
                enable = true;
                description = "Ensure Cargo.toml are sorted";
                entry = "cargo sort -g -w";
                types_or = [ "toml" ];
                pass_filenames = false;
              };
              cargo-clippy = {
                enable = true;
                description = "Run clippy";
                entry = "cargo clippy --workspace --all-targets -- -D warnings";
                types_or = [ "rust" "toml" ];
                pass_filenames = false;
              };
              forge-fmt = {
                enable = true;
                description = "Enforce forge fmt";
                entry = "forge fmt";
                types_or = [ "solidity" ];
                pass_filenames = false;
              };
              # MA: broken as of 2023-10-05. Cargo fmt makes changes to the generated
              # bindings which then cause the consistency check of `forge bind` to fail
              # because the files are no longer identical.
              # contract-bindings = {
              #   enable = true;
              #   description = "Generate contract bindings";
              #   entry = "just bindings";
              #   types_or = [ "solidity" ];
              #   pass_filenames = false;
              # };
              prettier-fmt = {
                enable = true;
                description = "Enforce markdown formatting";
                entry = "prettier -w";
                types_or = [ "markdown" ];
                pass_filenames = true;
              };
            };
          };
        };
        devShells.default =
          let
            stableToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            solc = pkgs.solc-bin.latest;
          in
          mkShell
            {
              buildInputs = [
                # Rust dependencies
                pkg-config
                openssl
                curl
                protobuf # to compile libp2p-autonat
                stableToolchain

                # Rust tools
                cargo-audit
                cargo-edit
                cargo-watch
                cargo-sort
                just
                fenix.packages.${system}.rust-analyzer

                # Tools
                entr

                # Figures
                graphviz
                plantuml
                coreutils

                # Ethereum contracts, solidity, ...
                foundry-bin
                solc
                nodePackages.prettier
                (python3.withPackages (ps: with ps; [ black ]))

              ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.SystemConfiguration ];
              shellHook = ''
                # Prevent cargo aliases from using programs in `~/.cargo` to avoid conflicts
                # with rustup installations.
                export CARGO_HOME=$HOME/.cargo-nix
              '' + self.checks.${system}.pre-commit-check.shellHook;
              RUST_SRC_PATH = "${stableToolchain}/lib/rustlib/src/rust/library";
              FOUNDRY_SOLC = "${solc}/bin/solc";
              inherit RUST_LOG RUST_BACKTRACE RUSTFLAGS;
            };
        devShells.crossShell = crossShell { config = "x86_64-unknown-linux-musl"; };
        devShells.armCrossShell = crossShell { config = "aarch64-unknown-linux-musl"; };
        devShells.rustShell =
          let
            stableToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          in
          mkShell
            {
              buildInputs = [
                # Rust dependencies
                pkg-config
                openssl
                curl
                protobuf # to compile libp2p-autonat
                stableToolchain
              ];
              inherit RUST_LOG RUST_BACKTRACE RUSTFLAGS;
            };
      }
    );
}
