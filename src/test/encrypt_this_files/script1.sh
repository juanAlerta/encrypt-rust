#!/bin/bash
declare -a hashes

for file in * 
do 
	if [ -f "$file" ]; then
		hash=$(sha256sum "$file")
		hashes+=("$hash")
	fi
done