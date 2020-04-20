function precmd() {
  PROMPT="\$(__CMD__ run --config=__CONFIG__ --jobs=\\"$jobtexts\\" --shell=zsh)"
}
