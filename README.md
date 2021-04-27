# Reveal.yaml

Rust implementation of Reveal.js YAML server.

This project was transferred from Python and now operates in a way that is easier to maintain and release, and it is Rust.
Some old functions might be deprecated, and some functions are improved.

This manager downloads the latest Reveal.js archive to provide serving and packing function, and had same licensed as Reveal.js.

### Why should use this?

Use Reveal.js with Markdown, but it is still difficult to maintain HTML slideshows. This work provides a clean YAML file for your slides.

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
**Slides in YAML**: The horizontal slides are as list in the second block, which is an array. A slide can work with at least one attribute structure.
```yaml
# metadata block
...
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
This work supports YAML 1.2.

## Command Line Interface

The command `rym` stands for "Reveal-Yaml Manager".
```bash
# Download the latest Reveal.js archive
rym update
# Create a project to current directory
rym new .
# Serve the slides
rym serve
# Pack the project to HTML archive
rym pack
```

## Features

Here are the implemented features, or the functions are designed in progress.

Some functions are planed to be demonstrated in the help page.

### Metadata

Metadata contains HTML settings and global slide settings.

+ [x] title: The webpage title, defaults to the first page.
+ [x] description: Webpage description.
+ [x] author: Webpage author.
+ [ ] background: Global background setting.
+ [x] outline: Auto generated table of the contents (TOC). This value is boolean.
+ [x] theme: Reveal.js theme, "serif" by default.
+ [x] code-theme: Highlight theme, "zenburn" by default.
+ [x] icon: Icon path, “img/icon.png” by default.
+ [x] style: Extra CSS style path.
+ [ ] Other Reveal.js options.
+ [ ] footer: Global footer option.

### Slides

+ [x] title: Markdown h2 title.
+ [x] no-title: Same as `title` but excluding in TOC.
+ [x] doc: Multiline Markdown text.
+ [x] include: Include a Markdown file from path, append after `doc`.
+ [x] math: Latex math without “$$” brackets.
+ [x] img: A list of image source.
+ [x] note: Speak view note.
+ [ ] background: Background setting.
+ [ ] Fragment option.
+ [x] sub: Vertical slides, for horizontal slides only.
