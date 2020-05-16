# Aurora Prompt

A shell prompt for Bash and Zsh.

## Examples

```sh
eval "$(prompt init zsh)"
```

Produces:

```
~/Dev/github/twe4ked/prompt master $
```

```sh
eval "$(prompt init bash "{cwd=short} $ ")"
```

Produces:

```
~/D/g/t/prompt $
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
- `{hostname}`
- `{jobs}`
- `{user}`

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

## Conditionals

Conditionals can be used to show and hide components and colors.

```
{if last_command_status}{cyan}${else}{red}${end}{reset}
```

## Design Goals

- Speed
- Minimal
- Simple configuration
- Default fonts

## Inspiration

- [starship/starship](https://github.com/starship/starship)
