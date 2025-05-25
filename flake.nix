{
    description = "My very first rust environment flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    outputs = {self, nixpkgs} :
    let
    # Function that takes a Nixpkgs package set and returns an attribute set
    # suitable for passing to pkgs.mkShell. The attribute set includes:
    # - packages: List of packages to include in the shell environment
    # - shellHook: Commands to run when entering the shell environment
    dev_shell_attrSet = pkgs: {
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
    # This function takes a platform (like "aarch64-darwin") as input and:
    # 1. Gets the nixpkgs package set for that platform
    # 2. Creates a development shell using mkShell with the packages
    #    and shellHook defined in dev_shell
    # 3. Returns it as the default devShell for that platform
    build_DevShell = platform :
        let
            pkgs = nixpkgs.legacyPackages.${platform};
        in
        {
            default = pkgs.mkShell (dev_shell_attrSet pkgs);
        };
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
        devShells = nixpkgs.lib.genAttrs [
            "aarch64-darwin"
            "x86_64-darwin"
        ]  build_DevShell;
    };
}
