#!/usr/bin/env bash

# $fragment: see possible options https://github.com/christian-draeger/increment-semantic-version/tree/1.2.3?tab=readme-ov-file#version-fragment
if [ "$fragment" = "" ]; then
  fragment=$1
fi

allowed_crates="protocol oauth core discovery audio metadata playback connect"

if [ "$fragment" = "patch" ]; then
  last_tag=$(git describe --tags --abbrev=0)
  awk_crates=$(echo "$allowed_crates" | tr ' ' '|')
  diff_crates=$(git diff $last_tag... --stat --name-only \
    | awk '/\.(rs|proto)$/{print}' \
    | awk "/($awk_crates)\//{print}" \
    | cut -d '/' -f 1 \
    | uniq \
    | tr \\n '\ ' \
    | xargs )
  echo "upgrading the following crates: [$diff_crates]"
else
  diff_crates=$allowed_crates
  echo "upgrading all crates for consistency"
fi

# append bin so that the version of the binary is also bumped
diff_crates="$diff_crates bin"

# required by script as it's usually a github action
export GITHUB_OUTPUT="version.txt"
# https://github.com/christian-draeger/increment-semantic-version/tree/1.2.3
increment_semver=$(curl https://raw.githubusercontent.com/christian-draeger/increment-semantic-version/refs/tags/1.2.3/entrypoint.sh)

for crate in $diff_crates ; do
  if [ "$crate" = "bin" ]; then
    toml="./Cargo.toml"
  else
    toml="./$crate/Cargo.toml"
  fi

  from="$(cat $toml | awk "/version/{print; exit}" | cut -d\" -f 2)"

  # execute script inline, extract result and remove output file
  echo "$increment_semver" | bash /dev/stdin $from $fragment
  to=$(cat $GITHUB_OUTPUT | cut -d= -f 2)
  rm $GITHUB_OUTPUT

  echo "upgrading [librespot-$crate] from [$from] to [$to]"

  # replace version in associated crate toml
  sed -i "0,/$from/{s/$from/$to/}" $toml

  if [ "$crate" = "bin" ]; then
    continue
  fi

  # update workspace dependency in root toml
  sed -i "/librespot-$crate/{s/$from/$to/}" ./Cargo.toml

  # update related dependencies in crate
  for crate in $allowed_crates ; do
    cat $toml | grep librespot-$crate > /dev/null
    if [ $? = 0 ]; then
      sed -i "/librespot-$crate/{s/$from/$to/}" $toml
    fi
  done
done

exit 0
