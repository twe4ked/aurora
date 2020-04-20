aurora_precmd() {
    local __jobs=$jobtexts
    PROMPT="$(__CMD__ run --config=__CONFIG__ --jobs="${__jobs:-__empty__}" --shell=zsh)"
}

[[ -z "${precmd_functions+1}" ]] && precmd_functions=()

if [[ ${precmd_functions[(ie)aurora_precmd]} -gt ${#precmd_functions} ]]; then
    precmd_functions+=(aurora_precmd)
fi
