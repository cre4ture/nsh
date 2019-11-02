nsh
====
[![Build Status](https://travis-ci.com/seiyanuta/nsh.svg?branch=master)](https://travis-ci.com/seiyanuta/nsh)
[![Latest version](https://img.shields.io/crates/v/nsh.svg)](https://crates.io/crates/nsh)

**Currently nsh is incomplete and not yet stable. Succeeded in crashing nsh? [Let me know](https://github.com/seiyanuta/nsh/issues)!**

A command-line shell that focuses on performance and productivity featuing:
- A not-yet-completed-but-aims-to-be **Bash compatible** interactive shell.
- **Tab completions** and **syntax highlighting** like **[fish](http://fishshell.com/)**.
- **[bash-completion](https://github.com/scop/bash-completion) support** (by simply invoking an external Bash for now).
- A builtin **interactive fuzzy completion filter** like **[fzf](https://github.com/junegunn/fzf)**.
- Builtin **zero configration** features and web-based config editor: `nsh-config`.
- **Written in Rust** :crab:

![screenshot](https://gist.githubusercontent.com/seiyanuta/5747db6c43978d9aa1941ce321cc1741/raw/405b7a1156292fd0456010b657f299b1daa367ff/nsh.png)

Installation
------------
```
$ cargo install nsh
```

To enable completions, install ([bash-completion](https://github.com/scop/bash-completion)). If you are using macOS,
install newer Bash as well:

```
$ brew install bash bash-completion@2
```

Configuration
-------------
Edit `~/.nshrc`.

## Prompt Format `($PROMPT)`

### Current context values
|          **Value**            |                **Description**               |
|:-----------------------------:|:--------------------------------------------:|
| \\{username}                  | User name                                    |
| \\{hostname}                  | Host name                                    |
| \\{current_dir}               | The current working directory                |


### Text styles and colors
|          **Value**            |                **Description**               |
|:-----------------------------:|:--------------------------------------------:|
| \\{reset}                     | Reset text styles and colors                 |
| \\{bold}                      | Bold                                         |
| \\{underline}                 | Underline                                    |
| \\{red}                       | Red                                          |
| \\{blue}                      | Blue                                         |
| \\{green}                     | Green                                        |
| \\{yellow}                    | Yellow                                       |
| \\{cyan}                      | cyan                                         |
| \\{magenta}                   | magenta                                      |

### Git
|          **Value**            |                **Description**               |
|:-----------------------------:|:--------------------------------------------:|
| \\{repo_status}               | Git repository status                        |

### Ternary expression
```
\\if{cond}{then}{else}
```

#### Conditionals
|          **Value**            |                **Description**               |
|:-----------------------------:|:--------------------------------------------:|
| \\in_repo                     | User name                                    |

### misc.
|          **Value**            |                **Description**               |
|:-----------------------------:|:--------------------------------------------:|
| \\n                           | User name                                    |

Key Shortcuts
-------------

|     **Key**     |                 **Action**                 |
|:---------------:|:------------------------------------------:|
| Up              | Select the previous history.               |
| Down            | Select the next history.                   |
| ^C              | Clear the input.                           |
| ^L              | Clear the screen.                          |
| ^W              | Delete the previous word.                  |
| ^K              | Delete from cursor to the end of input.    |
| M-f (Alt+Right) | Move the cursor to the next word.          |
| M-b (Alt+Left)  | Move the cursor to the previous word.      |
| ^A              | Move the cursor to the beginning of input. |
| ^E              | Move the cursor to the beginning of input. |
| TAB             | Enter the completion mode.                 |
| ^R              | Enter the history search mode.             |

----

Why create a new shell?
-----------------------
Bash is the best for executing shell scripts but its interactive mode is not satisfactory. I am
a zsh user for the last decade but I don't need *customizability* and got tired of making my zshrc
faster. Fish is really neat but I prefer old-fashioned, traditional, and ergonomic shell syntax.

Comparisons
-----------
| | **nsh**  | **[bash](https://www.gnu.org/software/bash)**  | **[zsh](http://www.zsh.org/)** | **[fish](http://fishshell.com/)** | **[PowerShell](https://github.com/PowerShell/PowerShell)** |
| :-: | :-: | :-: | :-: | :-: | :-: |
| **POSIX shell features**   | **Yes**              | **Yes**             | **Yes**                      | original syntax             | No             |
| **Bash compatibility**     | partially supported           | **100% compatible** | **provides `emulate(1)`**    | requires Bass               | No             |
| **Prompt UX**              | **(aims to be) awesome**           | minimum standard    | comfortable                  | **awesome**                 | comfortable    |
| **Configuration easiness** | **web-based config** | insufficient        | oh-my-zsh or very long zshrc | **web-based config**        | insufficient   |
| **Name**                   | not bad              | **noble**           | **cool**                     | **cute**                    | **super cool** |


----

Building
--------
### Prerequisites
- macOS or Linux
- Rust 1.31.0 or higher

```
$ cargo build --release
$ ./target/release/nsh
```

Testing
-------
```
$ cargo test
$ ./tools/run-blackbox-tests.py
```

Debugging
---------
The debug log file is located at `~/.nsh.log`. To enable debug log, run nsh with
`RUST_LOG` evironment variable (i.e., `$ RUST_LOG=nsh=trace nsh`).

Contributing
------------
nsh is in *alpha* stage: there are many missing features which Bash provides, there are kludges in
source code, and there must be bugs. To make nsh practical for daily use, I need your help!

### How can I contribute?
- **Report bugs** in [GitHub issues](https://github.com/seiyanuta/nsh/issues). Please attach
  a minimal reproducible example (e.g. shell script) *if possible*. It helps me to fix the bug easier.
- **Suggest enhancements** in [GitHub issues](https://github.com/seiyanuta/nsh/issues).
- **Submit a Pull Request** which implements a new feature, fixes a bug, refactors code, rephrases sentences in documentation, etc.

License
-------
CC0 or MIT. Choose whichever you prefer.
