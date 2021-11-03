# Reveal.yaml

[![dependency status](https://deps.rs/repo/github/KmolYuan/reveal-yaml-rs/status.svg)](https://deps.rs/crate/reveal-yaml/)

Rust implementation of [Reveal.js](https://github.com/hakimel/reveal.js) YAML server, a command line interface (CLI) tool.

<details><summary>Old Python version?</summary>
This project is transferred from Python language, so you may <a href="https://pypi.org/project/reveal-yaml/">found it on PyPI</a>. Reveal.yaml is now operates in a way that is easier to maintain and release, and it is Rust. Some old functions might be deprecated, and some functions are improved.
</details>

This manager downloads the latest Reveal.js archive to provide serving and packing function, and had same licensed as Reveal.js.

Static demo on gh-pages: <https://kmolyuan.github.io/reveal-yaml-rs>

YAML backend: <https://github.com/KmolYuan/yaml-peg-rs> (hosted by myself)

### Why should I use this?

Using Reveal.js with Markdown, but it is still difficult to maintain HTML slideshows. This work provides a clean YAML file for your slides, an auto-generated outline, a simple layout function, a powerful support with original Reveal.js function, and a live presentation when editing.

Difference to the before work, the Markdown to HTML translation works by this parser instead of using markdown.js, so **there is no more HTML escaping since they will be handled enough**. Except for using Markdown recursively in your code block, this needs to use the `<code>` tags by yourself.

If you are not decide yet, see the [tutorial](#tutorial) for more information.

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

## Tutorial

**Slides in HTML**: In Reveal.js, the HTML structure shown as following. The typesetting is done by original HTML and CSS.

```html
<section> <!-- Horizontal slide 1 -->
  <section>...</section> <!-- Vertical slide 1 -->
  <section>...</section> <!-- Vertical slide 2 -->
  ...
</section>
<section> <!-- Horizontal slide 2 -->
  ...
</section>
```

**Slides in YAML**: The horizontal slides are as listed in the second block, which is an array. A slide can work with at least one attribute structure.

```yaml
# metadata block
description: ...
author: ...
---
# slides block
- title: ...  # Works!
- doc: ...  # Works!
- img: ...  # Works!
```

The vertical slides work under the `sub` node of first slide, the attributes are same as horizontal slides.

```yaml
- title: Horizontal slide 1
  sub:
    - title: Vertical slide 1
    - title: Vertical slide 2
- title: Horizontal slide 2
```

This work supports YAML 1.2, and the anchor function supports for specific fields, such as content blocks.

### Layout

There are two layout types, called "horizontal stack" (`hstack`) and "vertical stack" (`vstack`). The vertical layout is default, as same as HTML.

The stack blocks list in the `hstack` / `vstack` tag, and the stack tags can be nested.

```yaml
hstack:
  - doc: Left
  - vstack:
    - doc: Right Top
    - doc: Right Bottom
```

The stack tag can add a dollar sign `$` to add a borderline between the elements.

```yaml
$hstack:
  - doc: Splitted left
  - doc: Splitted right
```

### Sized Attribute

The images and resizeable items are support "sized" attribute, which contains three options: `src`, `width` and `height`. The `src` option is required, otherwise the feature will be disabled or invalid.

The `width` and `height` options are the same as the attributes on the `<img>` tag, they are optional.

```yaml
footer:  # Footer is a metadata option
  src: help/icon.png
  width: 50pt
  label: Reveal.yaml
  link: https://github.com/KmolYuan/reveal-yaml/
img:
  - src: img/icon.png
    width: 50%  # same as width="50%"
    height: 70  # same as height="70"
video:
  width: 320
  height: 240
  src: https://www.w3schools.com/html/movie.mp4
  autoplay: true
```

### Reveal Plugins

The plugins excluding `markdown` are enabled by default.

## Functions

Here are the implemented features, or the functions are designed in progress.

Generally, the parser will not check extra key values. In addition, almost all values support the `null` placeholder to present the default value (blank, `~`, `null`).

Some functions are planed to be demonstrated in the help page. Open the help page by adding `/help/` after URL, like `http://localhost:8080/help/`.

### Metadata

Metadata contains HTML settings and global slide settings. They are totally YAML Maps. The definition contains in the first YAML doc, split by horizontal line `---`.

+ `icon`: Icon path, "img/icon.png" by default.
+ `lang`: Set the "lang" attribute for the page, "en" by default.
+ `title`: The webpage title, defaults to the first page.
+ `description`: Webpage description.
+ `author`: Webpage author.
+ `background`: Global [background setting](https://revealjs.com/backgrounds/#image-backgrounds).
  + `src`: Background source.
  + `size`: Background size.
  + `position`: Background position.
  + `repeat`: Background repeat. (repeat / no-repeat)
  + `opacity`: Background opacity from zero to one. (**float**)
+ `outline`: Auto generated table of the contents (TOC), boolean `true` by default.
+ `theme`: Reveal.js theme, "serif" by default.
+ `code-theme`: Highlight theme, "zenburn" by default.
+ `style`: Extra CSS script in `<style>` tag.
+ `footer`: Global footer option. You can add your logo here.
  + The block is **sized**.
  + `label`: Footer text.
  + `link`: Footer link, works on image and text.
+ `option`: Other Reveal.js [config](https://revealjs.com/config/) options.
  + Use any case string to indicate the option, this function will translate into lower camelcase, for example, YAML `slide number: c/t` will be JavaScript `slideNumber: "c/t"`.
  + This place is actually what `Reveal.initialize` input. So plugin options should be placed here.
  + Use `!!markdown` type on the string type, let us help you convert from Markdown to HTML simply!
+ `plugin`: Third-party Reveal plugins, a map with keys and arrays.
  + The key is the plugin object names, such as `RevealNotes`. The array is a list of plugin file paths, such as `plugin/notes/notes.js`.
  + The external plugin folder can be placed nearing `reveal.yaml`.
  + There is a repo demonstrate how to use [reveal.js-menu](https://github.com/denehyg/reveal.js-menu) plugin: [Reveal.yaml-menu](https://github.com/KmolYuan/reveal.yaml-menu).
  + [Here](https://github.com/hakimel/reveal.js/wiki/Plugins,-Tools-and-Hardware#plugins) is the plugin list recommend by official.

### Slides

Slides are a list of multiple slide blocks, they are totally YAML Maps.

```yaml
- title: Title 1
  doc: Document 1
- title: Title 2
  doc: Document 2
  sub:
    - title: Title 2-1
      doc: Document 2-1
```

+ **Title** Variants:
  + `title`: Markdown level 1 title without `#` notation.
  + `$title`: Visible title but will be excluded in TOC.
  + `-title`: Invisible title, doesn't show but will be included in TOC.
+ **Content** (they are placed in the following order)
  + `fit`: [Fit texts](https://revealjs.com/layout/#fit-text).
    + **Array** of level 2 texts.
    + Longer text will be smaller.
    + Special symbol `---` represents horizontal line `<hr/>`.
  + `doc`: Multiline Markdown text, accept HTML.
  + `include`: Include a Markdown file from path, append after `doc`. If you want to include an HTML file without conversion, just add `!!html` type notation.
  + `math`: Latex math without `$$` / `\[\]` brackets.
  + `img`: Embed images.
    + **Array**, can be **Map** if there is only one image.
    + The blocks are **sized**.
    + `label`: Image `<caption>`.
    + `pop`: Pop-up modal image, boolean `false` by default.
  + `video`: Embed videos.
    + **Array**, can be **Map** if there is only one video.
    + The blocks are **sized**.
    + `controls`: Allow controls, boolean `true` by default.
    + `autoplay`: Allow autoplay, boolean `false` by default.
    + `type`: Video type, default to "video/mp4".
  + `iframe`: Embed `<iframe>` structures, such as YouTube videos.
    + **Array**, can be **Map** if there is only one frame.
    + The blocks are **sized**.
    + Please be aware that `<iframe>` maybe slow down your web browser and cause security issues!
  + `lay-img`: [Layout stack](https://revealjs.com/layout/#stack) for images.
    + **Array**, can be **Map** if there is only one image.
    + The blocks are **sized**.
    + `fragment`: The animation option. Independent from `fragment` option.
  + `fragment`: [Fragment](https://revealjs.com/fragments/) option.
    + **Array**, the index are the `data-fragment-index`.
    + Block are **content**, but exclude stacks.
    + Stacks can have local fragment option, but still ordered.
  + **Stack**
    + **Array** format.
    + This function allows nesting.
    + `hstack`: Horizontal (columns) view of contents.
    + `vstack`: Vertical (rows) view of contents.
    + `$hstack` / `$vstack`: Make a border between each element.
+ `note`: Note in Speaker's view, Markdown syntax.
+ `bg-color`: [Background color](https://revealjs.com/backgrounds/#color-backgrounds).
+ `background`: Background setting, as same as global.
  + Local background option can be boolean `false` to disable global background.
+ `trans`: [Transition](https://revealjs.com/transitions/) option.
+ `bg-trans`: [Background transition](https://revealjs.com/transitions/#background-transitions) option.
+ `sub`: Vertical slides, for horizontal slides only.
