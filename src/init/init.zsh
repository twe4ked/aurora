function precmd() {
  PROMPT="\$(CMDCONFIG --jobs=\\"$jobtexts\\")"
}

export AURORA_SHELL="zsh"
