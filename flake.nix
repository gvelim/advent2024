{
    description = "My very first rust environment flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    };

    outputs = {self, nixpkgs} : {
        devShells.aarch64-darwin =
            let
                pkgs = nixpkgs.legacyPackages.aarch64-darwin;
            in
            {
                default = pkgs.mkShell {
                    packages = [
                        pkgs.rustc
                        pkgs.cargo
                        pkgs.nil
                        pkgs.nixd
                        pkgs.git
                    ];
                shellHook = ''
                    echo "Welcome to the Advent2024 development environment!"
                    /Applications/Zed.app/Contents/MacOS/zed . &
                '';
                };
            };
    };
}
