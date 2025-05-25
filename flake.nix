{
    description = "My very first rust environment flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    outputs = {self, nixpkgs} :
    let
        # creating a function that given a pkgs input
        # builds the attribute set for input the mkShell()
        dev_shell = pkgs: {
            packages = with pkgs; [
                rustc
                cargo
                nil
                nixd
                git
            ];
            shellHook = ''
                echo "Welcome to the Advent2024 development environment!"
                /Applications/Zed.app/Contents/MacOS/zed . &
            '';
        };
    in
    {
        devShells.aarch64-darwin =
            let
                pkgs = nixpkgs.legacyPackages.aarch64-darwin;
            in
            {
                default = pkgs.mkShell dev_shell pkgs;
            };
    };
}
