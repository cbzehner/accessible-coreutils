# list

Display the contents of a directory.

## Usage

| Long name | Short name | Output                                            | Type   |
| --------- | :--------: | ------------------------------------------------- | ------ |
| all       |     a      | Include hidden files                              | Flag   |
| color     |    N/A     | Colorize output                                   | Flag   |
| count     |     c      | Output count instead                              | Flag   |
| depth     |    N/A     | Limit depth of actions                            | Option |
| help      |     h      | Provide information about how to use this program | Flag   |
| hidden    |    N/A     | Access hidden files and folders                   | Flag   |
| json      |     j      | Output formatted JSON                             | Flag   |
| version   |     v      | Output current program version                    | Flag   |

## Potential flags

- [ ] sort + sort keyword (ex: `list --sort size`, `list --sort modification_time`)

## Alternatives

- [`ls`](https://en.wikipedia.org/wiki/Ls)
- [`exa`](https://the.exa.website/)
- On most systems `find . -maxdepth 1`
