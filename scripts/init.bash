function cd() {
	builtin cd $@
	pls
}

function mv() {
	command mv $@
	pls
}

function touch() {
	command touch $@
	pls
}

function rm() {
	command rm $@
	pls
}
