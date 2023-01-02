{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, utils }: {
    devShells = utils.lib.eachDefaultSystemMap (system:
    with import nixpkgs { inherit system; overlays = [(import rust-overlay)]; };
    {
      default = mkShell {
        buildInputs = [
          darwin.apple_sdk.IOKit
          rustup
          (rust-bin.fromRustupToolchainFile ./rust-toolchain)
          gtk4
          gdk-pixbuf
          graphene
          pkg-config
wabt
        ];
        shellHook = ''
          export PATH="$PATH:$HOME/.cargo/bin"
        '';
      };
    });
  };
}
