#!/usr/bin/env bash 

gt() {
	dir=$(~/.cargo/bin/gt $@)
	echo $dir	
	cd $dir
}

export -f gt
eval "$(gt init bash)"