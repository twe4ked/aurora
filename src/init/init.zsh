function precmd() {
  PROMPT="\$(CMDCONFIG --jobs=\\"$jobtexts\\" --shell=zsh)"
}
