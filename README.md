# Schmides
For when sliding is unavoidable

# Usage
Render slides to SVG by
```
typst compile <input_file> --format svg <output_dir>/{p}.svg 
```
here the `{p}` will be replaced by the page number. As pages do not have an inherent order otherwise, this is something the viewer currently expects.

An example Typst project is provided under `./slides`.

It can be rendered with properly formatted output by
```
cd slides && mkdir build
typst compile example.typ --format svg ./build/{p}.svg
```

To run the viewer:

```
cargo run --release
```

The selected directory must contain the slides and only the slides, named as shown in the example `<page no.>.svg`.

The Typst source file for the slides can be provided containing the notes.

The `./slides/example.typ` example contains some notes for the provided demo slides.


## Eventual goals
  - [x] Viewing `#speaker-note[...]` in the speaker note view
  - [x] Parsing speaker notes is currently a hack, maybe use `typst-lib`?
  - [x] Making it less shaky at least on MacOS
  - [ ] The code is a bit of a mess, could use some moving around
  - [ ] Add an editor (feature creep?)
