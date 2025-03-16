{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    treefmt-nix,
  }: let
    forEachSystem = nixpkgs.lib.genAttrs ["aarch64-linux" "x86_64-linux"];
    mkPkgs = system: (import nixpkgs {
      inherit system;
      overlays = [(import rust-overlay)];
    });
    mkToolchain = pkgs: (pkgs.rust-bin.selectLatestNightlyWith (toolchain:
      toolchain.default.override {
        extensions = ["rustc-codegen-cranelift-preview"];
      }));
  in {
    packages = forEachSystem (system: let
      pkgs = mkPkgs system;
    in {
      default = self.packages.${system}.bevussy;
      bevussy =
        (pkgs.makeRustPlatform {
          cargo = mkToolchain pkgs;
          rustc = mkToolchain pkgs;
        })
        .buildRustPackage rec {
          name = "bevussy";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs =
            [(mkToolchain pkgs)]
            ++ (with pkgs; [cargo-watch mold pkg-config]);
          buildInputs = with pkgs; [alsa-lib-with-plugins libxkbcommon udev vulkan-loader wayland];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
    });
    devShells.default = forEachSystem (system: let
      pkgs = mkPkgs system;
    in
      pkgs.mkShell {
        inherit (self.packages.${system}.bevussy) LD_LIBRARY_PATH buildInputs nativeBuildInputs;
      });
    formatter = forEachSystem (
      system: let
        pkgs = mkPkgs system;
      in
        (treefmt-nix.lib.evalModule pkgs (import ./treefmt.nix {inherit pkgs;}))
        .config
        .build
        .wrapper
    );
  };
}
