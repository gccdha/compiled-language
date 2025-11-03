let
  pkgs = import(fetchTarball "channel:nixpkgs-unstable"){};
in pkgs.mkShell {
  buildInputs = [ 
      pkgs.cargo      #for rust
      pkgs.rustc      #for rust
      pkgs.apostrophe #md editor
      pkgs.binutils   #for assembler
    ];
}
