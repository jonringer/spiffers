{ lib
, stdenv
, cargo
, mkShell
, protobuf
}:

mkShell {
  nativeBuildInputs = [ cargo protobuf ];
  buildInputs = [ protobuf ];
}
