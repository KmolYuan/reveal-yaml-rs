# Reveal.yaml

Rust implementation of Reveal.js YAML server.

This project is transferred from Python language and now operates in a way that is easier to maintain and release, and it is Rust.
Some old functions might be deprecated, and some functions are improved.

This manager downloads the latest Reveal.js archive to provide serving and packing function, and had same licensed as Reveal.js.

Static demo on gh-pages: <https://kmolyuan.github.io/reveal-yaml-rs/>

Download CLI executable from GitHub release: <https://github.com/KmolYuan/reveal-yaml-rs/releases/>

### Why should use this?

Using Reveal.js with Markdown, but it is still difficult to maintain HTML slideshows. This work provides a clean YAML file for your slides, an auto-generated outline, and a live demo when editing.

Difference to the before work, the Markdown to HTML translation is done by this parser instead of using markdown.js, so **there is no more HTML escaping since they will be handled enough**. (except using Markdown in your code block recursively, this needs to use `<code>` tag by yourself)

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
This work supports YAML 1.2 without anchor function.

### Sized Attribute

The images and resizeable items are support "sized" attribute, which contains three options: `src`, `width` and `height`. The `src` option is required, otherwise the feature will be disabled or invalid.

The `width` and `height` options are the same as the attributes on the `<img>` tag, they are optional.
```yaml
img:
  src: img/icon.png
  width: 50%  # same as width="50%"
  height: 70  # same as height="70"
```

### Reveal Plugins

The plugins excluding `markdown` are enabled by default.

## Command Line Interface

The command `rym` stands for "Reveal-Yaml Manager".
```bash
# Download the latest Reveal.js archive
rym update
# Create a project to current directory
rym new .
# Serve the slides
rym serve
# Reformat the project file
rym fmt
# Pack the project to HTML archive
rym pack
```

## Features

Here are the implemented features, or the functions are designed in progress. Generally, the parser will not check extra key values. In addition, almost all values support the `null` placeholder to present the default value (blank, `~`, `null`).

Some functions are planed to be demonstrated in the help page. Open the help page by adding `/help/` after URL, like `http://localhost:8080/help/`.

### Metadata

Metadata contains HTML settings and global slide settings.
The definition contains in the first YAML doc, split by horizontal line `---`.

+ [x] icon: Icon path, "img/icon.png" by default.
+ [x] lang: Set the "lang" attribute for the page, "en" by default.
+ [x] title: The webpage title, defaults to the first page.
+ [x] description: Webpage description.
+ [x] author: Webpage author.
+ [x] background: Global [background setting](https://revealjs.com/backgrounds/#image-backgrounds).
  + src: Background source.
  + size: Background size.
  + position: Background position.
  + repeat: Background repeat. (repeat / no-repeat)
  + opacity: Background opacity from zero to one. (**float**)
+ [x] outline: Auto generated table of the contents (TOC). Default to true. (**boolean**)
+ [x] theme: Reveal.js theme, "serif" by default.
+ [x] code-theme: Highlight theme, "zenburn" by default.
+ [x] style: Extra CSS style path.
+ [x] footer: Global footer option. You can add your logo here. (**sized**)
  + label: Footer text.
  + link: Footer link, works on image and text.
+ [x] option: Other Reveal.js [config](https://revealjs.com/config/) options.
  + Use any case string to indicate the option, this function will translate into lower camelcase, for example, YAML `slide number: c/t` will be JavaScript `slideNumber: "c/t"`.

### Slides

+ [x] title: Markdown level 1 title without `#` notation.
+ [x] $title: Invisible title, same as `title` but will be excluded in TOC, this page will uncounted.
  + If `title` is enabled, `$title` will not work.
+ [x] Content: (they are placed in the following order)
  + [x] doc: Multiline Markdown text, accept HTML.
  + [x] include: Include a Markdown file from path, append after `doc`.
    + [x] math: Latex math without `$$` / `\[\]` brackets.
  + [x] img: A list of image source.
    + **Array**, can be map if there is only one image.
    + Each block are **sized**.
    + label: Image `<caption>`.
  + [ ] stack:
    + [x] hstack: Columns view of contents. (**Array**)
    + [x] vstack: Row view of contents.
    + [ ] border: Make a border between each element.
    + This function allows nesting.
+ [x] note: Speak view note.
+ [x] bg-color: [Background color](https://revealjs.com/backgrounds/#color-backgrounds).
+ [x] background: Background setting, as same as global.
  + Local background option can be boolean `false` to disable global background.
+ [x] trans: [Transition](https://revealjs.com/transitions/) option.
+ [x] bg-trans: [Background transition](https://revealjs.com/transitions/#background-transitions) option.
+ [x] fragment: [Fragment](https://revealjs.com/fragments/) option.
  + **Array**, the index are the `data-fragment-index`.
  + Each block are **content**, but exclude stacks.
  + Stacks can have local fragment option, but still ordered.
+ [x] sub: Vertical slides, for horizontal slides only.
