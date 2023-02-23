{ lib
, stdenv
, cargo
, clippy
, mkShell
, pkg-config
, openssl
, protobuf
, rustc
, rustfmt
, spire
}:

mkShell {
  nativeBuildInputs = [
    cargo
    clippy
    pkg-config
    protobuf
    rustc
    rustfmt

    # For testing
    spire
  ];

  buildInputs = [
    openssl
  ];
}
