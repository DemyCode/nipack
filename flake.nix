{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      nixpkgs-mozilla,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain =
          (pkgs.rustChannelOf {
            rustToolchain = ./rust-toolchain.toml;
            sha256 = "sha256-Qxt8XAuaUR2OMdKbN4u8dBJOhSHxS+uS06Wl9+flVEk=";
            #        ^ After you run `nix build`, replace this with the actual
            #          hash from the error message
          }).rust;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          shellHook = ''
            rustup show
            fish
          '';
          nativeBuildInputs = [
            pkgs.rustup
            pkgs.fish
          ];
        };
      }
    );
}
