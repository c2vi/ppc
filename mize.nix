{

module = { mkMizeRustModule, ... }:

mkMizeRustModule ({
  modName = "ppc";
  src = ./.;
  postInstall = ''
    cp -r $src/static $out/static
  '';
});

}
