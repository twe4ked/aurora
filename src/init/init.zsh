setopt prompt_subst

aurora_precmd() {
    local __status=$?
    local __jobs=$jobtexts
    PROMPT="$(__CMD__ run --config=__CONFIG__ --jobs="${__jobs:-__empty__}" --shell=zsh --status="$__status")"
}

autoload -U add-zsh-hook

add-zsh-hook precmd aurora_precmd
