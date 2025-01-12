# v0.4.0

- Update Rust grammar and queries pins. [7b590f4](https://github.com/hadronized/kak-tree-sitter/commit/7b590f4)
- Remove default theme and make cascaded tree-sitter faces. [1406e7d](https://github.com/hadronized/kak-tree-sitter/commit/1406e7d)
- Add more generic faces [d118438](https://github.com/hadronized/kak-tree-sitter/commit/d118438)
- Introduce user-only configuration. [fc7c5c6](https://github.com/hadronized/kak-tree-sitter/commit/fc7c5c6)
- Update default-config.md to use sources. [386c81b](https://github.com/hadronized/kak-tree-sitter/commit/386c81b)
- Introduce ktsctl sources. [e083aad](https://github.com/hadronized/kak-tree-sitter/commit/e083aad)
- Work on a better CLI for ktsctl. [8d8d6de](https://github.com/hadronized/kak-tree-sitter/commit/8d8d6de)
- Update man pages for the new ktsctl. [5eeb2f4](https://github.com/hadronized/kak-tree-sitter/commit/5eeb2f4)
- Enhance ktsctl’s look. [3edd4a5](https://github.com/hadronized/kak-tree-sitter/commit/3edd4a5)
- Rename ktsctl’s --has into --lang. [4367082](https://github.com/hadronized/kak-tree-sitter/commit/4367082)
- Support reading local paths when used as sources for both grammars & queries. [305455b](https://github.com/hadronized/kak-tree-sitter/commit/305455b)
- Add support for cached (git) sources. [03e037c](https://github.com/hadronized/kak-tree-sitter/commit/03e037c)
- Force git sources to have a pin. [4c6d168](https://github.com/hadronized/kak-tree-sitter/commit/4c6d168)
- Support detecting out-of-sync pins and more logs in ktsctl. [4903faf](https://github.com/hadronized/kak-tree-sitter/commit/4903faf)
- Add support for syncing resources. [2c7e87c](https://github.com/hadronized/kak-tree-sitter/commit/2c7e87c)
- Add SCSS config. [2a7dcd1](https://github.com/hadronized/kak-tree-sitter/commit/2a7dcd1)
- Add Vue config. [1c87643](https://github.com/hadronized/kak-tree-sitter/commit/1c87643)
- Add Unison config. [c08bfc3](https://github.com/hadronized/kak-tree-sitter/commit/c08bfc3)
- Add Nix config. [8ce2cec](https://github.com/hadronized/kak-tree-sitter/commit/8ce2cec)
- Add C# config. [665d755](https://github.com/hadronized/kak-tree-sitter/commit/665d755)
- Add Elixir config. [b9f85f5](https://github.com/hadronized/kak-tree-sitter/commit/b9f85f5)
- --depth 1 on git operations to speed things up. [6e375be](https://github.com/hadronized/kak-tree-sitter/commit/6e375be)
- Introduce --all. [c0bd4c8](https://github.com/hadronized/kak-tree-sitter/commit/c0bd4c8)

---

SHA 256 sums

```
5eabce3a45932207046f356f9763b402a3ee02467244daa5c4d2c8b712e99a3d  ktsctl.Linux-x86_64
70b8fcc5a6db25dfe05451a18a09875724cff2d6c7fabdfd7a078cfa68ea7692  ktsctl.macOS-x86_64
```

# v0.3.3

- Support for kak-tree-sitter-config-v0.4.0

# v0.3.2

- Add --has to ktsctl. [baaf735](https://github.com/hadronized/kak-tree-sitter/commit/baaf735)

# v0.3.1

- Enhance CLI of ktsctl. [1b8fbbd](https://github.com/hadronized/kak-tree-sitter/commit/1b8fbbd)

# v0.3.0

- `<lost changelog, sorry :(>`

# v0.2.0

- Proper error handling.
- Remove `--query -q`. It is now inferred from the other arguments.
- The meaning of the various `path` configuration option has changed a bit. We do not magically insert more
  indirections from there. For instance, if the query config has a given directory set for `path`, that directory
  content will be copied to `$XDG_DATA_DIR/kak-tree-sitter/queries/<lang>`.


# v0.1.0

- Initial release.
