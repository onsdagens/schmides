# LTU Typst Beamer Template
This repo is a work-in-progress Touying-based slides template.

We are aiming for this to eventually be as close to the Luleå University of Technology PowerPoint template reference as possible, this is not yet the case.

The theme itself lives in `ltu_touying.typ`.

We've included some lecture slides in `example.typ` as a usage example

`halcyon.tmTheme` contains an example syntax highlighting theme (in this case [Halcyon](https://github.com/bchiang7/Halcyon))

## Rendering

Install `typst-cli` 

```bash
cargo install typst-cli --locked
```

Compile the slides by

```bash
typst compile example.typ
```

This yields `example.pdf` containing the slides. 
