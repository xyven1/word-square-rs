{
  inputs = {
    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    fenix,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [
          fenix.packages.${system}.complete.toolchain
          (pkgs.writeShellScriptBin "lldb-dap" ''
            ${pkgs.lib.getExe' pkgs.lldb_19 "lldb-dap"} --pre-init-command  "command script import ${pkgs.fetchFromGitHub {
              owner = "cmrschwarz";
              repo = "rust-prettifier-for-lldb";
              rev = "v0.4";
              hash = "sha256-eje+Bs7kS87x9zCwH+7Tl1S/Bdv8dGkA0BoijOOdmeI=";
            }}/rust_prettifier_for_lldb.py" $@
          '')
        ];
      };
    });
}
