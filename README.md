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

- `{cwd}`, `{cwd style=short underline_repo=true}`, `{cwd style=long}`, `{cwd style=default}`
- `{env name=HOME}`

    A Zsh example of using a precommand to populate an environment variable with
    a custom string. This can be used to put anything in your prompt.

    ```zsh
    my_date_precmd() {
        export MY_DATE="$(date)"
    }

    autoload -U add-zsh-hook
    add-zsh-hook precmd my_date_precmd

    eval "$(aurora_prompt init zsh "{env name=MY_DATE}")"
    ```

- `{git_branch}`
- `{git_commit}`
- `{git_stash}`
- `{git_status}`
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

### Reset

- `{reset}`

## Conditionals

Conditionals can be used to show and hide components and colors.

```
# Last command status returns "true" if the last command returned 0
{if last_command_status}{cyan}${else}{red}${end}{reset}

# Environment variables
{user}{if $SSH_CONNECTION}@{hostname}{end}
```

## Design Goals

- Speed
- Minimal
- Simple configuration
- Default fonts

## Adding a new component

Begin by adding a new variant to the `Component` enum in `token` and update the
`TryFrom<&str> for Component` impl. From there follow the compiler errors!

## Inspiration

- [starship/starship](https://github.com/starship/starship)
