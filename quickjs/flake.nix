{
  description = "quicker";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;

  outputs = { self, nixpkgs }: {

    defaultPackage.x86_64-linux =
      # Notice the reference to nixpkgs here.
      let 
        pkgs = import nixpkgs { system = "x86_64-linux"; };
      in
        pkgs.stdenv.mkDerivation {
            name = "quicker-js";
            src = self;
            buildInputs = with pkgs; [
                pkgconfig
                llvm
                llvmPackages.libclang
                clang
                python3
                gdb
                wasmtime
            ];
            buildPhase = "";
            installPhase = "";
        };
  };
}
