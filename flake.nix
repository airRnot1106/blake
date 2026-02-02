{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/70801e06d9730c4f1704fbd3bbf5b8e11c03a2a7";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      fenix,
      git-hooks,
      nixpkgs,
      systems,
      treefmt-nix,
      ...
    }:
    let
      eachSystem =
        f:
        nixpkgs.lib.genAttrs (import systems) (
          system:
          f {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
            overlays = [
              fenix.overlays.default
            ];
          }
        );
      toolchainFor =
        system:
        fenix.packages.${system}.stable.withComponents [
          "cargo"
          "clippy"
          "rust-analyzer"
          "rust-src"
          "rustc"
          "rustfmt"
        ];
    in
    {
      devShells = eachSystem (
        { system, pkgs, ... }:
        {
          default = pkgs.mkShell {
            inherit (self.devHooks.${system}) shellHook;
            packages = with pkgs; [
              (toolchainFor system)
              openssl
              pkg-config
            ];
          };
        }
      );
      formatter = eachSystem (
        { pkgs, system, ... }:
        treefmt-nix.lib.mkWrapper pkgs {
          projectRootFile = "flake.nix";
          programs = {
            nixfmt.enable = true;
            rustfmt = {
              enable = true;
              package = toolchainFor system;
            };
          };
        }
      );
      checks = eachSystem (
        { system, ... }:
        {
          # For nix flake check - only treefmt (no network access in sandbox)
          pre-commit-check = git-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              treefmt = {
                enable = true;
                package = self.formatter.${system};
              };
            };
          };
        }
      );
      # For local development - includes clippy (has network access)
      devHooks = eachSystem (
        { system, ... }:
        git-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            treefmt = {
              enable = true;
              package = self.formatter.${system};
            };
            clippy = {
              enable = true;
              packageOverrides = {
                cargo = toolchainFor system;
                clippy = toolchainFor system;
              };
            };
          };
        }
      );
    };
}
