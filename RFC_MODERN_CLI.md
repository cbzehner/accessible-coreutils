# RFC: A Modern Command Line

### The problems with the command line interface

Clunky
Unintuitive / Shortened function names are
Inconsistent options
No one understands about GNU

### Goals

Accessible to everyone
Consistent interface
Experience and ability she correlate

### Non-goals

Replace coreutils
Speed
Cross-platform compatibly

### Level Up Utilities

1. ls -> exa -> list
2. find -> fd -> find
3. grep -> ripgrep -> search
4. uniq -> unique (+ sort by default)
5. man -> tldr -> how (or help?)
6. cp -> copy -> copy/rename
7. touch -> create/update
8. mkdir -> create —directory/folder
9. rm -> delete
10. ln -> link
11. file -> type
12. cat -> bat -> content/view
13. less -> view/read

### A new generation: Friendly command lines

1. list
2. find
3. search
4. view

Rust wrappers for incremental upgrades
Opt-in via the installer
And a single bundle

### The importance of bash compatibility

Swimming with the Fishes

### Don’t break the build

Rust wrappers for the existing coreutils (where supported)

Acceptance criteria: Nothing breaks the ability to build Linux from Scratch
