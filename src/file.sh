#!/usr/bin/env bash 

gt() {
	~/.cargo/bin/gt "$@"
}

read userinput

if [ $userinput == "gt" ];then
	gt
fi