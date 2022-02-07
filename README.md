# Reveal.yaml

[![dependency status](https://deps.rs/repo/github/KmolYuan/reveal-yaml-rs/status.svg)](https://deps.rs/crate/reveal-yaml/)
[![Documentation](https://docs.rs/reveal-yaml/badge.svg)](https://docs.rs/reveal-yaml)

Rust implementation of [Reveal.js](https://github.com/hakimel/reveal.js) YAML server, a command line interface (CLI) tool.

<details><summary>The old Python version?</summary>
This project is transferred from Python language, so you may <a href="https://pypi.org/project/reveal-yaml/">found it on PyPI</a>. Reveal.yaml is now operates in a way that is easier to maintain and release, and it is Rust. Some old functions might be deprecated, and some functions are improved.
</details>

This manager downloads the latest Reveal.js archive to provide serving and packing function, and had same licensed as Reveal.js.

Static demo on gh-pages: <https://kmolyuan.github.io/reveal-yaml-rs>

YAML backend: <https://github.com/KmolYuan/yaml-peg-rs> (hosted by myself)

### Why should I use this?

| Source                         | Output                         |
|:-------------------------------|:-------------------------------|
| YAML + media (Images / Videos) | HTML Slide (static) / Showcase |

Have you ever using Reveal.js with Markdown, but it is still difficult to maintain HTML slideshows? This work provides a clean YAML file for your slides, an auto-generated outline, a simple layout function, a powerful support with original Reveal.js function, and a live presentation when editing.

Difference to the before work, the Markdown to HTML translation works by this parser instead of using markdown.js, so **there is no more HTML escaping since they will be handled enough**. Except for using Markdown recursively in your code block, this needs to use the `<code>` tags by yourself.

If you are not decide yet, see the documentation for more information.

## Installation

Download CLI executable from GitHub release: <https://github.com/KmolYuan/reveal-yaml-rs/releases/>

Wherever the binary placed, it should be visible for the environment variable `PATH`.

If you are a Rust user, install it with cargo is possible. Unfortunately, the binary is build with the cargo toolchain, and you will get a large size than the CI/CD result.

```
cargo install reveal-yaml
```

It recommends download GitHub distribution directly for saving your time. For example, a CI/CD script can write as:

```bash
wget -O reveal-yaml.zip https://github.com/KmolYuan/reveal-yaml-rs/releases/latest/download/reveal-yaml-linux-amd64.zip
unzip reveal-yaml.zip
chmod +x rym
./rym pack
```

The executable can be checked with `rym --help`.

## Command Line Interface

The command `rym` stands for "Reveal-Yaml Manager".

```bash
# Download the latest Reveal.js archive
rym update
# Create a project to current directory
# Only "reveal.yaml" will be create, the rest is up to you!
rym new .
# Serve the slides
rym serve
# Reformat the project file
rym fmt
# Pack the project to HTML archive
rym pack
```

### Edit Mode (Hot Reload / Auto-reload)

There are `-e` / `--edit` flags on the `serve` command. This option let the server keep watching the project file `reveal.yaml`, then reload the page from the web browser.

If this option is not enabled, the server will only resolve once at startup, and always use the cache.

```bash
rym serve --edit
```
