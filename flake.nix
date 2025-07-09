#
# https://nixos.wiki/wiki/Rust
#
{
  description = "My very first rust environment flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      overrides = builtins.fromTOML (builtins.readFile (self + "/rust-toolchain.toml"));
      # Function that takes a Nixpkgs package set and returns an attribute set
      # suitable for passing to pkgs.mkShell. The attribute set includes:
      # - packages: List of packages to include in the shell environment
      # - shellHook: Commands to run when entering the shell environment
      mkShell_attrSet = pkgs: system: rec {
        packages = with pkgs; [
          rustup
          nil
          nixd
          git
          starship
          nix-bash-completions
        ];
        RUSTC_VERSION = overrides.toolchain.channel;
        shellHook = ''
          export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          # Dynamically determine the Rust system string (architecture-os) for the current system
          export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/${RUSTC_VERSION}-${system}/bin/

          # enable starship for branch and active SDKs
          [ ! -f ~/.config/starship.toml ] && starship preset no-nerd-font -o ~/.config/starship.toml
          eval "$(starship init bash)"
          # acticate bash completions
          source ${pkgs.bash-completion}/etc/profile.d/bash_completion.sh

          echo "Welcome to the Advent2024 development environment!"
          eval "$(starship init bash)"
          eval "$(hstr --show-bash-configuration)"
          # zed .
        '';

      };
      # This function takes a platform (like "aarch64-darwin") as input and:
      # 1. Gets the nixpkgs package set for that platform
      # 2. Creates a development shell using mkShell with the packages
      #    and shellHook defined in dev_shell
      # 3. Returns it as the default devShell for that platform
      build_DevShell =
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          # equivalent to
          # pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell (mkShell_attrSet pkgs system);
        };
      platforms = [
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    {
      # Generate development shells for specified platforms.
      # This uses nixpkgs.lib.genAttrs to iterate over the list of platform strings
      # and call the build_DevShell function for each platform.
      # The result is an attribute set where keys are platform strings (e.g., "aarch64-darwin")
      # and values are the development shells defined by build_DevShell for that platform.
      #
      # Specifically, for this example, it results in an attribute set like:
      # {
      #   aarch64-darwin = { default = <aarch64-darwin dev shell>; };
      #   x86_64-darwin = { default = <x86_64-darwin dev shell>; };
      # }
      devShells = nixpkgs.lib.genAttrs platforms build_DevShell;
      packages = nixpkgs.lib.genAttrs platforms (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          # Define your Rust application package here
          # We'll call the Nix package 'advent2024-solutions' as it contains multiple solutions
          advent2024-solutions = pkgs.rustPlatform.buildRustPackage {
            pname = "advent2024-solutions";
            version = "0.1";
            # The source code for your Rust project.
            # 'self' refers to the root of your flake.
            # This assumes your Cargo.toml is directly in the flake root.
            src = self;

            # This is CRUCIAL for reproducible Rust builds.
            # It tells Nix to use your project's Cargo.lock file.
            cargoLock = {
              lockFile = self + "/Cargo.lock";
            };

            # You can add build flags here, e.g., for release builds
            # cargoBuildFlags = "--release";

            # This tells cargo install to install ALL binaries defined in src/bin/*
            # by building the project from the current source path (.).
            cargoInstallFlags = "--path .";
          };
        }
      );
    };
}
