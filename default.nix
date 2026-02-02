{
  lib,
  craneLib,
  openssl,
  pkg-config,
}:
let
  cargoToml = fromTOML (builtins.readFile ./Cargo.toml);

  pname = cargoToml.package.name;
  version = cargoToml.package.version;
in
craneLib.buildPackage {
  inherit pname version;

  src = craneLib.cleanCargoSource ./.;

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config ];

  meta = {
    description = "Tig-like terminal UI for exploring git blame history";
    homepage = "https://github.com/airRnot1106/blake";
    license = lib.licenses.mit;
    mainProgram = "blake";
  };
}
