# FNV
Research into reverse engineering fl studio's .fnv (curve/envelope/state) files and creation of an editor for them.

for more info, read RESEARCH.md

# Project plans:
- [-] RIIR?.. - in progress!
- [-] Complete reverse engineering of the format + an unofficial spec
- [-] Command-line app 
  * [ ] ability to convert JSON to .fnv and vice versa;
  * [ ] support for different streams (stdout, from file, pipe etc.)
- [ ] A full, Qt-based WYSIWYG visual editor
  * [ ] support for every function FL Studio's curve system has
  * [ ] additional features such as advanced grids, warping curves, inputting more precise data, formula input, conversion between curve types
- [ ] Possible support for Serum's .shp and Vital's .vitallfo (might be the easiest, since it's just a minified JSON string)
- [ ] Possible support for synchronising FL Studio with the editor (not sure, since it most likely would include hacking into FL Studio's process memory)
- [ ] Kaitai struct implemenation


