{ lib
, stdenv
, cargo
, mkShell
, protobuf
, rustfmt
}:

mkShell {
  nativeBuildInputs = [ cargo protobuf rustfmt ];
  buildInputs = [ protobuf ];
}
