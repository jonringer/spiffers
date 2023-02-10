{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "spiffers";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  meta = with lib; {
    description = "Rust implementation of SPIFFE";
    homepage = "https://github.com/jonringer/spiffers";
    license = licenses.agpl3Plus;
    maintainers = with maintainers; [ jonringer ];
  };
}
