# Latex Resume Generator

`Laregen` (Latex Resume Generator) is a latex cv generator that uses `Tera` for templating. It allows the user to generate [se-resume](https://github.com/mdauthentic/se-resume) (but really any latex cv if you provide your own template) using a `json` or `yaml` (also `yml`) file containing contents to be generated as the `cv`.


## How to use

```bash
Usage: laregen [OPTIONS] --input <input>

Options:
-i, --input <input>        resume data file in either json or yaml format
-t, --template <template>  template file written in tera`s dialect [default: resume.tex]
-o, --output <output>      location on disk where you want the output to be written [default: ./output/generated.tex]
-h, --help                 Print help
-V, --version              Print version


# using the default schema and template
cargo run -- --input ./resources/resume.json
# Passing
cargo run -- -i ./resources/resume.json -t test.tex -o ./output/test-generated.tex
```

## Todo

The current implementation only generate cv suited for one template (se-resume).

- Add support for more CV templates generation
- Add support for [jsonresume](https://jsonresume.org/)