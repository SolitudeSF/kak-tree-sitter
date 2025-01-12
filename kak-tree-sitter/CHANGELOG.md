# v0.6.0

- First shot at text-objects. [1350fa1](https://github.com/hadronized/kak-tree-sitter/commit/1350fa1)
- Adding support for selections with text-objects. [46075c8](https://github.com/hadronized/kak-tree-sitter/commit/46075c8)
- First full and working implementation of text-object. [42935c9](https://github.com/hadronized/kak-tree-sitter/commit/42935c9)
- Remove default theme and make cascaded tree-sitter faces. [1406e7d](https://github.com/hadronized/kak-tree-sitter/commit/1406e7d)
- Introduce user-only configuration. [fc7c5c6](https://github.com/hadronized/kak-tree-sitter/commit/fc7c5c6)
- Move the user manual into the repository, extracted from the wiki. [a7ac734](https://github.com/hadronized/kak-tree-sitter/commit/a7ac734)
- Add --with-highlighting. [152f5f6](https://github.com/hadronized/kak-tree-sitter/commit/152f5f6)
- Fix injecting Kakoune RC when we do not have any verbose flag. [ec1ee3c](https://github.com/hadronized/kak-tree-sitter/commit/ec1ee3c)
- Support reading local paths when used as sources for both grammars & queries. [305455b](https://github.com/hadronized/kak-tree-sitter/commit/305455b)
- Introduce ktsctl sources. [e083aad](https://github.com/hadronized/kak-tree-sitter/commit/e083aad)

---

SHA 256 sums

```
f0bd6991a492218870bdcddaa7ee1eb8fdd5b69bbc2f99a1b16e2e5bf247d1dd  kak-tree-sitter.Linux-x86_64
a8c0dd2899c5492f53b6e2d9c53ac0cf300f52298be5ada0f1d5c9b52444cf91  kak-tree-sitter.macOS-x86_64
```

# v0.5.4

- Fix getting (already existing) sessions. [3c8e86d](https://github.com/hadronized/kak-tree-sitter/commit/3c8e86d)

# v0.5.3

- Fix freezing / crashing Kakoune instances by allowing to recover, as well as
  optimizing testing whether sessions exist. [5cbba41](https://github.com/hadronized/kak-tree-sitter/commit/5cbba41) [c525dff](https://github.com/hadronized/kak-tree-sitter/commit/c525dff) [61b8b50](https://github.com/hadronized/kak-tree-sitter/commit/61b8b50)
- Dependencies bumps.
- Fix a typo in `kak-tree-sitter-req-stop`. [660c162](https://github.com/hadronized/kak-tree-sitter/commit/660c162)

# v0.5.2

- Fix regression introduced in 7db85a4b4e394750e633ceea8e390e052c61dac0. [974dc39](https://github.com/hadronized/kak-tree-sitter/commit/974dc39)

# v0.5.1

- Some internal fix with how newlines are handled. This fix is not complete though, so expect
  weird behavior sometimes. Will be fixed in an upcoming patch. [7db85a4](https://github.com/hadronized/kak-tree-sitter/commit/7db85a4)
- Fix unicode graphemes in the byte -> (col, line) mapper. [46d6f1c](https://github.com/hadronized/kak-tree-sitter/commit/46d6f1c)

# v0.5.0

- Add `remove_default_highlighter` option. [d78abc0](https://github.com/hadronized/kak-tree-sitter/commit/d78abc0)
- Send disconnect response (deinit) when the server quits. [5d58316](https://github.com/hadronized/kak-tree-sitter/commit/5d58316)
- Remove `colored` in `kak-tree-sitter`. [d5b0094](https://github.com/hadronized/kak-tree-sitter/commit/d5b0094)
- Fix blocking Kakoune after stopping KTS. [a3d68ea](https://github.com/hadronized/kak-tree-sitter/commit/a3d68ea)
- (Re)-add response queue to prevent blocking Kakoune. [404c8b8](https://github.com/hadronized/kak-tree-sitter/commit/404c8b8)
- Clear buf fifo on errors. [1343f00](https://github.com/hadronized/kak-tree-sitter/commit/1343f00)
- dependencies bumps. [Various](https://github.com/hadronized/kak-tree-sitter/commit/Various)

# v0.4.6

- Add kak-tree-sitter-req-reload to static.kak. [36f4710](https://github.com/hadronized/kak-tree-sitter/commit/36f4710)
- Add support for reloading config, grammars and queries. #108 [efe0669](https://github.com/hadronized/kak-tree-sitter/commit/efe0669)
- Add build.rs to add git commit hash to versions in clap. [4381835](https://github.com/hadronized/kak-tree-sitter/commit/4381835)
- Fix FIFO corrupted data. [b06338f](https://github.com/hadronized/kak-tree-sitter/commit/b06338f)

# v0.4.5

- Rework FIFOs to make them all nonblocking and introduce per-session state machine. [291f376](https://github.com/hadronized/kak-tree-sitter/commit/291f376)
- Fix fork leak. #10 [34f2a9a](https://github.com/hadronized/kak-tree-sitter/commit/34f2a9a)

# v0.4.4

- Fix for EOF on the command FIFO. [ce72760](https://github.com/hadronized/kak-tree-sitter/commit/ce72760)
- Update static.kak for HTML (@attribute and @tag). [ec5d77d](https://github.com/hadronized/kak-tree-sitter/commit/ec5d77d)

# v0.4.3

- Enhance CLI of kak-tree-sitter. #100 [ceb0811](https://github.com/hadronized/kak-tree-sitter/commit/ceb0811)
- Add some requests in the .kak. [3147012](https://github.com/hadronized/kak-tree-sitter/commit/3147012)
- Revamp RC and --kakoune to fix opening files from the CLI. #101 [a017807](https://github.com/hadronized/kak-tree-sitter/commit/a017807)

# v0.4.2

- Fix IO command reads. [17d7866](https://github.com/hadronized/kak-tree-sitter/commit/17d7866)

# v0.4.1

- Introduce kts_lang. [a446955](https://github.com/hadronized/kak-tree-sitter/commit/a446955)
- Fix commands by appending ;. [322823d](https://github.com/hadronized/kak-tree-sitter/commit/322823d)

# v0.4.0

- Add support for YAML. [a045858](https://github.com/hadronized/kak-tree-sitter/commit/a045858)
- Java support. [2a58160](https://github.com/hadronized/kak-tree-sitter/commit/2a58160)
- Scheme support. [30e10c9](https://github.com/hadronized/kak-tree-sitter/commit/30e10c9)
- Fix highlighting for injections. [ac1d239](https://github.com/hadronized/kak-tree-sitter/commit/ac1d239)
- More highlighting groups. [3f47a8f](https://github.com/hadronized/kak-tree-sitter/commit/3f47a8f)
- Hand-craft markdown queries. [d2d9761](https://github.com/hadronized/kak-tree-sitter/commit/d2d9761)
- Fix html config and vendor C++ queries. [bf5fbfc](https://github.com/hadronized/kak-tree-sitter/commit/bf5fbfc)
- Fix C++ highlighting. [848bddb](https://github.com/hadronized/kak-tree-sitter/commit/848bddb)
- Support for git-commit. [18dafa0](https://github.com/hadronized/kak-tree-sitter/commit/18dafa0)
- Add support for tree-sitter-diff. [4e700ad](https://github.com/hadronized/kak-tree-sitter/commit/4e700ad)
- Remove cabal config. [cce2278](https://github.com/hadronized/kak-tree-sitter/commit/cce2278)
- Fix bash support. [eedda58](https://github.com/hadronized/kak-tree-sitter/commit/eedda58)
- Add LaTeX support. [9d3b046](https://github.com/hadronized/kak-tree-sitter/commit/9d3b046)
- Simplify highlighting not to override the highlighter everytime. [e288aca](https://github.com/hadronized/kak-tree-sitter/commit/e288aca)
- Add CONTRIBUTING.md. [8a78dec](https://github.com/hadronized/kak-tree-sitter/commit/8a78dec)
- Add python support. [72eb686](https://github.com/hadronized/kak-tree-sitter/commit/72eb686)
- Support BibTex highlighting. [50fdec1](https://github.com/hadronized/kak-tree-sitter/commit/50fdec1)
- Support Haskell highlighting. [9f384ea](https://github.com/hadronized/kak-tree-sitter/commit/9f384ea)
- Support JSON highlighting. [345c61e](https://github.com/hadronized/kak-tree-sitter/commit/345c61e)
- Support XML highlighting. [df43f7d](https://github.com/hadronized/kak-tree-sitter/commit/df43f7d)
- Support CSS highlighting. [30d91ae](https://github.com/hadronized/kak-tree-sitter/commit/30d91ae)
- Support Makefile highlighting. [9391357](https://github.com/hadronized/kak-tree-sitter/commit/9391357)
- Fix highlighter range computations (better unicode support). [fd4d770](https://github.com/hadronized/kak-tree-sitter/commit/fd4d770)
- Allow to read commands from a command FIFO. [ea0fe6f](https://github.com/hadronized/kak-tree-sitter/commit/ea0fe6f)
- Enable highlighting via the FIFO command interface. [58e0aea](https://github.com/hadronized/kak-tree-sitter/commit/58e0aea)
- Rewrite with mio and remove tokio. [8961a3a](https://github.com/hadronized/kak-tree-sitter/commit/8961a3a)
- Introduce per-session FIFO. [c57a2f1](https://github.com/hadronized/kak-tree-sitter/commit/c57a2f1)
- Fix PID detection. #81 [db20bdf](https://github.com/hadronized/kak-tree-sitter/commit/db20bdf)
- Go grammar. [db92ec5](https://github.com/hadronized/kak-tree-sitter/commit/db92ec5)
- Add support for sessionless (standalone) server. #90 [616863c](https://github.com/hadronized/kak-tree-sitter/commit/616863c)
- Adding support for ctrl-c signals. [8eb30e7](https://github.com/hadronized/kak-tree-sitter/commit/8eb30e7)
- Allow to specify the log level. [9b25d55](https://github.com/hadronized/kak-tree-sitter/commit/9b25d55)

# v0.3.0

- Proper error handling.
- Some Kakoune commands are now hidden.
- Because the FIFO / UNIX socket bug was fixed, `kak-tree-sitter-highlight-enable` now always performs an initial
  highlight of the buffer with `kak-tree-sitter-highlight-buffer`.
- Rework faces to include specific ones. For instance, `ts_function` with `ts_function_builtin`, `ts_function_macro`,
  etc.
- Add more details about the runtime state, especially when loading grammars, and make loading resources
  (grammars / queries) error collectable instead of fallible.

commit 3713c420dd9c9ac3845e90ceb71de6a87e36ba88 (tag: ktsctl-v0.2.0, tag: kak-tree-sitter-v0.3.0, tag: kak-tree-sitter-con

# v0.2.0

- Initial version of the project. `v0.1` was being used on crates.io by a completely different project, and they gave
  use the name.
