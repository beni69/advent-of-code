with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "aoc-hs";
  buildInputs = [
    ghc
    haskell-language-server
  ];
}
