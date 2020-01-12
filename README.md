# Prompt

A shell prompt. Currently only works in Zsh.

## Examples

```sh
eval "$(prompt init)"
```

Produces:

```
~/Dev/github/twe4ked/prompt master $
```

```sh
eval "$(prompt init "\
{green}{cwd style=short}\
{yellow} ± {git_branch}:{git_commit}{reset}\
{dark_grey} {git_stash}{reset}\
{dark_grey} {jobs}{reset}\
{cyan} $ {reset}")"
```

Produces:

```
~/D/g/t/prompt ± master:bacd2a3 1+ $
```

## Components

- `{cwd}`, `{cwd style=short}`, `{cwd style=long}`, `{cwd style=default}`
- `{git_branch}`
- `{git_commit}`
- `{git_stash}`
- `{jobs}`

## Colors

Colors are also used to define groups, if all components within a color group
return nothing, the entire group will be squashed. Groups are defined as
everything between a color and a `{reset}`.

- `{black}`, `{dark_grey}`
- `{blue}`, `{dark_blue}`
- `{green}`, `{dark_green}`
- `{red}`, `{dark_red}`
- `{cyan}`, `{dark_cyan}`
- `{magenta}`, `{dark_magenta}`
- `{yellow}`, `{dark_yellow}`
- `{white}`
- `{reset}`
