{
  description = 
    "Ferramenta de lnha de comando para criação de novos serviços Pescarte";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = {
        linux = "x86_64-linux";
        darwin = "aarch64-darwin";
      };

      pkgs = system: import nixpkgs {
        inherit system;
        config.allowUnfree = true;
        overlays = [(import rust-overlay)];
      };

      inputs = sys: with pkgs sys; [
        rust-bin.stable.latest.default clippy
      ];
    in {
      devShells = {
        "${systems.linux}".default = with pkgs systems.linux; mkShell {
          name = "pescarte-cli";
          buildInputs = inputs systems.linux;
        };

        "${systems.darwin}".default = with pkgs systems.darwin; mkShell {
          name = "pescarte-cli";
          buildInputs = inputs systems.darwin;
        };
      };
    };
}
