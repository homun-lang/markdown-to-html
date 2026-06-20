# markdown-to-html

A Markdown-to-HTML compiler written in [Homun language](https://github.com/HomunMage/Homun-Lang). Converts standard Markdown syntax to HTML while passing through any existing HTML elements untouched.

## Design

**HTML passthrough** — Any raw HTML tags in the input are preserved as-is. The compiler only transforms Markdown syntax; it never parses, modifies, or escapes existing HTML DOM elements.

## Supported Markdown Features

| Feature | Syntax |
|---------|--------|
| Headings | `# H1` through `###### H6` |
| Bold | `**bold**` |
| Italic | `*italic*` |
| Bold + Italic | `***both***` |
| Strikethrough | `~~deleted~~` |
| Inline code | `` `code` `` |
| Code blocks | ` ``` ` fenced blocks with language tag |
| Links | `[text](url)` |
| Images | `![alt](src)` |
| Unordered lists | `- item` or `* item` |
| Ordered lists | `1. item` |
| Nested lists | Indented sub-items |
| Blockquotes | `> quote` |
| Horizontal rules | `---` |
| Tables | `| col | col |` with `|---|---|` separator |
| Line breaks | Trailing double space or `\` |
| Paragraphs | Blank line separated blocks |

### Parse order

All other Markdown features are parsed before tables. By the time table `|` delimiters are evaluated, inline code, HTML elements, bold, italic, links, etc. are already resolved into opaque nodes. A `|` inside any previously parsed construct is not a table delimiter — it belongs to that construct.

## Build

Requires [homunc](https://github.com/HomunMage/Homun-Lang) (auto-downloaded by `build.rs`).

```bash
cargo build --release
```

## Usage

```bash
# File input
markdown-to-html input.md > output.html

# Stdin
echo "# Hello" | markdown-to-html
```

## Architecture

```
Markdown text (with possible inline HTML)
  → Lexer (tokenize md syntax, pass HTML through)
  → Parser (build block tree: headings, lists, tables, code blocks, paragraphs)
  → Codegen (emit HTML from block tree, raw HTML nodes copied verbatim)
  → HTML output
```

Core logic is written in `.hom` files, compiled to Rust at build time by `homunc`.

## License

MIT
