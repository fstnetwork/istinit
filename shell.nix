let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  inherit (pkgs) stdenv;
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [

    git
    drone-cli

    rustup
    cargo-make

    clang
    llvmPackages.libclang
    pkgconfig

    openssl

    kubectl
    kind
    istioctl
  ];

  RUST_BACKTRACE = 1;

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";

  KUBECONFIG = "./target/kubeconfig";
}
