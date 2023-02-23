{ lib
, stdenv
, cargo
, clippy
, darwin
, mkShell
, pkg-config
, libiconv
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
  ] ++ lib.optionals stdenv.isDarwin [
    libiconv
    darwin.apple_sdk.frameworks.Security
  ];
}
