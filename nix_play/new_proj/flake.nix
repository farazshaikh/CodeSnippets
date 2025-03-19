rec {
  name = "faraz";
  occupation = "Engineering";
  family = {
    mother = "samina";
    wife = "Sana";
    daughter = "hana";
    son = "faiz";
  };

  recursive = {
    one = 1;
    two = 2;
    first_two_nums = {
      one = recursive.one;
      two = recursive.two;
    };

  };

  computed_number = let
    x = 1;
    y = 2;
  in x + y;

  wife_name = family.wife;

  hiding = let
    m = 2;
    x = 4;
  in m + x;
  access_hiding = hiding;

  attset_extraction = let
    attrset = {
      x = 1;
      y = 2;
    };
  in attrset.x;

  latestgen = let
    nesting = { hamzekhan = { qudsiya = { samina = { son = "faraz"; }; }; }; };
  in nesting.hamzekhan.qudsiya.samina.son;

  with_exampe = let
    qudsiya.siblings = {
      first = "masood";
      second = "samina";
      third = "Shakeel";
      forth = "reza";
    };
  in {
    last_two = with qudsiya.siblings; [ third forth ];
    inherit latestgen;
    inherit (family) mother wife daughter;
  };

  more_iherit = { inherit (with_exampe) latestgen; };

  hello_world = let
    h = "hello";
    w = "world";
  in "{h} {w}";
  echo_faraz_and_user = let f = "faraz"; in "echo ${f} $user";
  mutlline_string = ''
    thisis ia
        multiline string spanning over
        multiple pyhusical lines'';
  home_dir = /home/faraz;
  rel_path = ./.;
  nix_pgks = <nixpkgs/lib>;
  zfunction = x: x + 1;
  zfunction2 = x: y: x + y;
  zfattrse = args@{ x, y ? 0, ... }: x + y;
  zfevalue = zfattrse { x = 1; };
  zinlinedefcall = (x: x + 1) 1;
  zf = x: x + 1;
  zl = [ zf 1 ];
  zle = [ (zf 1) ];
  zzf = x: y: z: x * y * z;
  zzfv = (zzf 2 2 3);
  zzprimo = builtins.toString 1;
  zzprim2 = builtins.toString 23;
  sd = 23;
  zzimp = import ./imp.nix;
  zzlib = let pkgs = import <nixpkgs> { };
  in pkgs.lib.strings.toUpper "lookup paths considered harmful";
  zcs = builtins.currentSystem;
  zzzf = "${/tmp/data.txt}";
  zzzg = builtins.fetchurl "https://www.google.com";

  zzzzm = let pkgs = import <nixpkgs> { }; in "${pkgs.nix}";
}
