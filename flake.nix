{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShell.${system} = pkgs.mkShell {
        packages = with pkgs; [
	  gcc
	  gdb
	  gnumake
	  clang-tools
	  clang_20
	  cargo
	  rustc
	  rustfmt
	  clippy
        ];
        shellHook = ''
          exec zsh
        '';
      };
    };
}
