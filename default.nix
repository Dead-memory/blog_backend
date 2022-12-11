{
  pkgs ? import <nixpkgs> {}
}:
pkgs.mkShell {
  name="blog_backend";
  buildInputs = with pkgs; [
    cargo rustup
    diesel-cli
  ];
}
