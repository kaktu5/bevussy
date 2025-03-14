{pkgs}: let
  inherit (pkgs) toml-sort;
  inherit (pkgs.lib) getExe;
in {
  projectRootFile = "flake.nix";
  programs = {
    alejandra.enable = true;
    deadnix.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
  };
  settings.formatter.toml-sort = {
    command = "${getExe toml-sort}";
    options = ["--all" "--no-sort-tables" "--in-place"];
    includes = ["*.toml"];
    excludes = [".cargo/config.toml"];
  };
  settings.formatter.toml-sort-2 = {
    command = "${getExe toml-sort}";
    options = ["--no-sort-tables" "--in-place"];
    includes = [".cargo/config.toml"];
  };
}
