{rustPlatform}:
rustPlatform.buildRustPackage {
  name = "nixy";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
