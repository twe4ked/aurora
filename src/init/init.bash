# Will be run before *every* command (even ones in pipes!)
aurora_preexec() {
    # Save previous command's last argument, otherwise it will be set to
    # "aurora_preexec"
    local PREV_LAST_ARG=$1

    : "$PREV_LAST_ARG"
}

# Will be run before the prompt is drawn
aurora_precmd() {
    PS1="$(__CMD__ run --config=__CONFIG__ --jobs="$(jobs -p | wc -l)" --shell=bash)"
}

# We want to avoid destroying an existing DEBUG hook. If we detect one, create
# a new function that runs both the existing function AND our function, then
# re-trap DEBUG to use this new function. This prevents a trap clobber.
dbg_trap="$(trap -p DEBUG | cut -d' ' -f3 | tr -d \')"
if [[ -z "$dbg_trap" ]]; then
    trap 'aurora_preexec "$_"' DEBUG
elif [[ "$dbg_trap" != 'aurora_preexec "$_"' && "$dbg_trap" != 'aurora_preexec_all "$_"' ]]; then
    function aurora_preexec_all(){
      local PREV_LAST_ARG=$1 ; $dbg_trap; aurora_preexec; : "$PREV_LAST_ARG";
    }
    trap 'aurora_preexec_all "$_"' DEBUG
fi

# Finally, prepare the precmd function and set up the start time. Avoid adding
# multiple instances of the aurora function and keep other user functions if
# any.
if [[ -z "$PROMPT_COMMAND" ]]; then
    PROMPT_COMMAND="aurora_precmd"
elif [[ "$PROMPT_COMMAND" != *"aurora_precmd" ]]; then
    # Remove any trailing semicolon before appending
    PROMPT_COMMAND="${PROMPT_COMMAND%;};aurora_precmd;"
fi
