let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
  pkgs = import nixpkgs { config = { }; overlays = [ ]; };
in
pkgs.mkShell {
  packages = with pkgs; [
    dotnet-sdk_8
    clang
  ];
}
