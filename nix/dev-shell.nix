{ lib
, stdenv
, cargo
, mkShell
}

mkShell {
  nativeBuildInputs = [ cargo ];
}
