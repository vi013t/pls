function cd() {
	builtin cd "$@" && clear && pls
}

function mv() {
	command mv "$@" && clear && pls
}

function touch() {
	command touch "$@" && clear && pls
}

function rm() {
	command rm "$@" && clear && pls
}

function mkdir() {
	command mkdir "$@" && clear && pls
}
