# Git Changelog

Simple tool for release changelog reports with multiple template support.

Features

- Supports multiple output templates
- [Handlebars](https://handlebarsjs.com/) templating language support
- Excludes bots noise from the output (Dependabot, etc.)

## Usage

You can get usage details by running the command with the `--help` argument:

```sh
./git-changelog --help
```

```text
USAGE:
    git-changelog [FLAGS] [OPTIONS] <range>

ARGS:
    <range>    Commit range, i.e. master..develop

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Verbose output
    -V, --version    Prints version information

OPTIONS:
    -c, --config <path>          Sets a custom config file [default: default.conf]
    -d, --dir <path>             Working directory [default: .]
    -n, --max-count <number>     Limit the number of commits to output
    -o, --output <output>        Output file, will use console output if not defined
        --skip <number>          Skip number commits before starting to show the commit output
    -t, --template <template>    Path to the custom output template
```

### Usage Examples

```sh
# generate release log with markdown format using changes between develop and master branches
./git-changelog -d ~/github/my-app master..develop > v1.md

# generate release log with HTML format using changes between develop and master branches
./git-changelog -d ~/github/my-app master..develop -t ./assets/templates/html.hbs > v1.html
```

## Templates

The tool supports [Handlebars templating language](https://handlebarsjs.com/).

Output templates:

- **Markdown (default)** (assets/templates/md.hbs)
- **HTML** (assets/templates/html.hbs)

Simple Markdown template example looks like the following:

```text
# Changelog

{{#each commits as |commit|}}
 - [{{commit.hash}}]({{../repo_url}}/commit/{{commit.hash}}) {{commit.subject}}
{{/each}}
```

## Building

```sh
# debug build
cargo build

# release build
cargo build --release
```

## License

MIT License, see [LICENCE](LICENSE) for more details.
