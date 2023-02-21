{ lib
, stdenv
, cargo
, clippy
, mkShell
, protobuf
, rustfmt
, spire
}:

mkShell {
  nativeBuildInputs = [
    cargo
    clippy
    protobuf
    rustfmt

    # For testing
    spire
  ];
}
