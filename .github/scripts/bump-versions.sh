#!/usr/bin/env bash

# $FRAGMENT: see possible options https://github.com/christian-draeger/increment-semantic-version/tree/1.2.3?tab=readme-ov-file#version-fragment
if [ "$FRAGMENT" = "" ]; then
  FRAGMENT=$1
fi

CRATES="protocol oauth core discovery audio metadata playback connect"

if [ "$FRAGMENT" = "patch" ]; then
  LAST_TAG=$(git describe --tags --abbrev=0)
  AWK_CRATES=$(echo "$CRATES" | tr ' ' '|')
  DIFF_CRATES=$(git diff $LAST_TAG... --stat --name-only \
    | awk '/(rs|proto)$/{print}' \
    | awk "/($AWK_CRATES)/{print}" \
    | cut -d '/' -f 1 \
    | uniq \
    | tr \\n '\ ' \
    | xargs )
  echo "upgrading the following crates: [$DIFF_CRATES]"
else
  DIFF_CRATES=$CRATES
  echo "upgrading all crates for consistency"
fi

# append bin so that the version of the binary is also bumped
DIFF_CRATES="$DIFF_CRATES bin"

# required by script as it's usually a github action
export GITHUB_OUTPUT="version.txt"
# https://github.com/christian-draeger/increment-semantic-version/tree/1.2.3
INCREMENT_SEMVER=$(curl https://raw.githubusercontent.com/christian-draeger/increment-semantic-version/refs/tags/1.2.3/entrypoint.sh)

for CRATE in $DIFF_CRATES ; do
  if [ "$CRATE" = "bin" ]; then
    TOML="./Cargo.toml"
  else
    TOML="./$CRATE/Cargo.toml"
  fi

  FROM="$(cat $TOML | awk "/version/{print; exit}" | cut -d\" -f 2)"

  # execute script inline, extract result and remove output file
  echo "$INCREMENT_SEMVER" | bash /dev/stdin $FROM $FRAGMENT
  TO=$(cat $GITHUB_OUTPUT | cut -d= -f 2)
  rm $GITHUB_OUTPUT

  echo "upgrading [librespot-$CRATE] from [$FROM] to [$TO]"

  # replace version in associated crate toml
  sed -i "0,/$FROM/{s/$FROM/$TO/}" $TOML

  if [ "$CRATE" = "bin" ]; then
    continue
  fi

  # update workspace dependency in root toml
  sed -i "/librespot-$CRATE/{s/$FROM/$TO/}" ./Cargo.toml

  # update related dependencies in crate
  for crate in $CRATES ; do
    cat $TOML | grep librespot-$crate > /dev/null
    if [ $? = 0 ]; then
      sed -i "/librespot-$CRATE/{s/$FROM/$TO/}" $TOML
    fi
  done
done

exit 0
