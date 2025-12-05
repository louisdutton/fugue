{
  lib,
  rustPlatform,
  callPackage,
  runCommand,
  installShellFiles,
  git,
  gitRev ? null,
  grammarOverlays ? [],
  includeGrammarIf ? _: true,
}: let
  fs = lib.fileset;

  src = fs.difference (fs.gitTracked ./.) (fs.unions [
    ./.envrc
    ./rustfmt.toml
    ./docs
    ./flake.lock
    (fs.fileFilter (file: lib.strings.hasInfix ".git" file.name) ./.)
    (fs.fileFilter (file: file.hasExt "svg") ./.)
    (fs.fileFilter (file: file.hasExt "md") ./.)
    (fs.fileFilter (file: file.hasExt "nix") ./.)
  ]);
in
  rustPlatform.buildRustPackage (self: {
    cargoLock = {
      lockFile = ./Cargo.lock;
      # This is not allowed in nixpkgs but is very convenient here: it allows us to
      # avoid specifying `outputHashes` here for any git dependencies we might take
      # on temporarily.
      allowBuiltinFetchGit = true;
    };

    nativeBuildInputs = [
      installShellFiles
      git
    ];

    buildType = "release";

    name = with builtins; (fromTOML (readFile ./fugue-term/Cargo.toml)).package.name;
    src = fs.toSource {
      root = ./.;
      fileset = src;
    };

    # So Fugue knows what rev it is.
    FUGUE_NIX_BUILD_REV = gitRev;

    doCheck = false;
    strictDeps = true;

    # Get all the application stuff in the output directory.
    postInstall = ''
      mkdir -p $out/lib
      installShellCompletion ${./contrib/completion}/fugue.{bash,fish,zsh}
    '';

    meta.mainProgram = "fugue";
  })
