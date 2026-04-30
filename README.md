# Schmdies
For when sliding is unavoidable

# Usage
Render slides to SVG by
```
typst compile <input_file> --format svg <output_dir>/{p}.svg 
```
here the `{p}` will be replaced by the page number. As pages do not have an inherent order otherwise, this is something the viewer currently expects.

An example Typst project is provided under `./lecture_7`.

It can be rendered with properly formatted output by
```
typst compile example.typ --format svg ./build/{p}.svg
```

To run the viewer:

```
cargo run --release
```

The selected directory must contain the slides and only the slides, named as show in the example `<page no.>.svg`.

## Eventual goals
  - Viewing `#speaker-note[...]` in the speaker note view
