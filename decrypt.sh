#!/bin/bash
set -e

if [[ $# -eq 0 ]] ; then
  echo "Please pass passphrase as argument"
  exit 1
fi

mkdir -p inputs

for encrypted in inputs_encrypted/*; do
  file=inputs/${encrypted#inputs_encrypted/}
  file=${file%.gpg}
  if [ -f $file ]; then
    echo "$encrypted ... already done"
  else
    # gpg --passphrase $1 -c --no-symkey-cache --cipher-algo AES256 --batch -o $encrypted $file
    gpg --quiet --passphrase $1 -d --no-symkey-cache --cipher-algo AES256 --batch -o $file $encrypted
    echo "$encrypted  ->  $file"
  fi
done
