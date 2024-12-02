#!/bin/bash
set -e

if [[ $# -eq 0 ]] ; then
  echo "Please pass passphrase as argument"
  exit 1
fi

mkdir -p inputs_encrypted

for file in inputs/*; do
  encrypted=inputs_encrypted/${file#inputs/}.gpg
  if [ -f $encrypted ]; then
    echo "$file ... already done"
  else
    gpg --passphrase $1 -c --no-symkey-cache --cipher-algo AES256 --batch -o $encrypted $file
    echo "$file  ->  $encrypted"
  fi
done
