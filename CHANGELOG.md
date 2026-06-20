# Changelog

## v0.2 — 2026-06-20

- Implement full markdown-to-html compiler in Homun language (.hom)
- Lexer, parser, inline formatter, table parser, codegen pipeline
- Support headings, bold, italic, strikethrough, inline code, code blocks
- Support links, images, lists (ordered/unordered/nested), blockquotes, HR
- Table parsing with column alignment, runs last after inline resolution
- HTML passthrough — raw HTML preserved verbatim
- WASM build for browser playground
- CLI: file input or stdin

## v0.1 — 2026-06-19

- Project infrastructure: CI, Docker, hooks, license
- Homunc build system (build.rs auto-downloads homunc)
- WASM playground with split-pane UI
- Golden test cases for all supported features
