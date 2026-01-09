{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    gobject-introspection
  ];

  buildInputs = with pkgs; [
    gtk4
    gtk4-layer-shell
    # libadwaita
    # glib.dev
    # pango.dev
    # gdk-pixbuf.dev
    # cairo.dev
    # harfbuzz.dev
    # fontconfig.dev
    # freetype.dev
    # fribidi
    # libxml2.dev
    # graphene
  ];

  shellHook = ''
    # This manually assembles the PKG_CONFIG_PATH for all core dependencies
    echo "--- GTK Development Environment Ready ---"
    echo "Check: pkg-config --exists glib-2.0 && echo 'GLib found!'"
  '';
}
