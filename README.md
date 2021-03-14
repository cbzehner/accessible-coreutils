# accessible-coreutils

Friendly command-line utilities for practical programmers.

## Design Considerations

1. Intuitive names

Make the names easy to guess, even for those unfamiliar with a command-line environment. This means avoiding shortened names, it's not worth saving a couple characters at the cost of confusion, real experts can create an alias.

2. Standard options

Standardize the various options and flags (binary options) used across utilities. This follows from the first principle of making use intutitve.

3. Provide helpful errors

When something goes wrong, do not merely fail, but suggest a possible way to use the program in a more correct way.

4. Incremental enhancement

When possible, provide guidance for new users to proceed from basic use cases (ex: `history | grep ls`) to more advanced uses.

## Approach

Building a friendly-commandline is not an all or nothing undertaking. There are existing utilities that improve upon the current standard utilities in a variety of ways, `rg` instead of `grep` and `bat` rather than `cat`. In many cases, we might choose to re-use these tools rather than building something new.

## Progress

Currently 1%. This is just an idea about a problem and what it takes to fix it.

The `aliases.sh` provides a starting point for testing out some of these names and this entire alternative experience with little initial commitment.

There is also ongoing work on a `porcelain` utility which provides the ability to create new executables that remap onto the existing GNU `coreutils` and shell built-in functions. However, further work there will require building out a robust config system which can be used to both provide mappings and support the CLAP crate's existing functionality.

## Detailed Design

Create internationalized configuration files which can be used by `porcelain` to build out a full set of wrapping executables on the existing command-line utilities.

### Standard Options

| Long name   | Short name | Description                                        | Type   |
| ----------- | :--------: | -------------------------------------------------- | ------ |
| color       |    N/A     | Colorize output                                    | Flag   |
| count       |     c      | Output count instead                               | Flag   |
| delimiter   |     d      | Override the default delimiter                     | Option |
| depth       |    N/A     | Limit depth of actions                             | Option |
| dir(ectory) |    N/A     | Act on a directory                                 | Flag   |
| file(s)     |    N/A     | Act on a file                                      | Flag   |
| help        |     h      | Provide information about how to use this program  | Flag   |
| hidden      |    N/A     | Access hidden files and folders                    | Flag   |
| ignore      |     i      | Ignore                                             | Flag   |
| interactive |     i      | Enable interactive mode                            | Flag   |
| json        |     j      | Output formatted JSON                              | Flag   |
| quiet       |     q      | Silence output                                     | Flag   |
| verbose     |    N/A     | Display detailed information about the program run | Flag   |
| version     |     v      | Output current program version                     | Flag   |

### Standardize Delimiters

The default delimiter should always be spaces. Other delimiters can be specified using the `--delimiter` option.
