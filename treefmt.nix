{pkgs}: {
  projectRootFile = "flake.nix";
  programs = {
    alejandra.enable = true;
    deadnix.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
  };
  settings.formatter.toml-sort = {
    command = with pkgs; "${lib.getExe toml-sort}";
    options = ["--all" "--no-sort-tables" "--in-place"];
    includes = ["*.toml"];
  };
}
