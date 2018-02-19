# coreutils
Recreate the functionality of core unix utilities in Rust

# Design Considerations
1. Intuitive names
Make the names easy to guess, even for those unfamiliar with a command-line environment. This means avoiding shortened names, it's not worth saving a couple characters at the cost of confusion, real experts can create an alias.

2. Standard options
Standardize the various options and flags (binary options) used across utilities. This follows from the first principle of making use intutitve.

3. Provide helpful errors
When something goes wrong, do not merely fail, but suggest a possible way to use the program in a more correct way.

4. Incremental enhancement
When possible, provide guidance for new users to proceed from basic use cases (ex: `history | grep ls`) to more advanced uses.

# Standard Options

| Long name | Short name | Description | Type |
| --- | --- | --- |
| delimiter | d | Override the default delimiter | Option |
| dir | N/A | Act on a directory | Flag |
| file | N/A | Act on a file | Flag |
| help | h | Provide information about how to use this program | Flag |
| hidden | N/A | Access hidden files and folders | Flag |
| interactive | i | Enable interactive mode | Flag |
| verbose | N/A | Display detailed information about the program run | Flag |
| version | v | Output current program version | Flag |

# Standardize Delimiters
The default delimiter should always be spaces. Other delimiters can be specified using the `--delimiter` option.

# Progress
Currently 0%.

[] less -> view
[] ls -> list
[] mv -> move
[] mv -> rename
[] rm -> delete
[] rm -> remove
[] sort -> sort
[] cut -> ???
[] mkdir -> create --dir/--file
[] cd -> go/change/...?
[] make -> make
[] cat -> print
[] cmp -> compare
[] cp -> copy
[] yes -> yes
[] compress -> compress
[] ed -> edit
[] env -> environment
[] file -> file
[] ln -> ?
[] link -> ?
[] find -> search
[] man -> docs
[] pwd -> where
[] ps -> process
[] sleep -> sleep
[] time -> profile (include info about memory usage as well)
[] uniq -> unique
[] wc -> count
[] who -> who
