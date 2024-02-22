{
    pkgs,
    ...
}:

pkgs.mkShell {
    NIX_CONFIG = "extra-experimental-features = nix-command flakes repl-flake";
    buildInputs = with pkgs; [
        protobuf
    ];
}
