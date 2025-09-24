#!/usr/bin/env bash

allowed_crates="protocol oauth core discovery audio metadata playback connect"

current_tag=$(git describe --abbrev=0)
last_tag=$(git describe --abbrev=0 $current_tag^)

bin_version=$(cat ./Cargo.toml | awk "/version/{print; exit}" | cut -d\" -f 2)

simple_version_regex="^v?([0-9]+)\.([0-9]+)\.([0-9]+)$"
if [[ $last_tag =~ $simple_version_regex ]]; then
  last_major="${BASH_REMATCH[1]}"
  last_minor="${BASH_REMATCH[2]}"
  last_patch="${BASH_REMATCH[3]}"
else
  echo "regex for tag didn't match"
  exit 1
fi

if [[ $bin_version =~ $simple_version_regex ]]; then
  if [ "$last_major" != "${BASH_REMATCH[1]}" ] || [ "$last_minor" != "${BASH_REMATCH[2]}" ]; then
    echo "[]"
    exit 0
  elif [ "$last_patch" == "${BASH_REMATCH[3]}" ]; then
    echo "version didn't change"
    exit 1
  fi
else
  echo "regex for bin version didn't match"
  exit 1
fi

# if we go through here, we build a patch version and only want to update the crates that have changed
awk_crates=$(echo "$allowed_crates" | tr ' ' '|')
diff_crates=$(git diff $last_tag... --stat --name-only \
  | awk '/\.(rs|proto)$/{print}' \
  | awk "/($awk_crates)\//{print}" \
  | cut -d '/' -f 1 \
  | uniq \
  | tr '\n' ' ' \
  | xargs \
  | tr ' ' '|' )

excluded_diff=$(echo $allowed_crates \
  | tr ' ' '\n' \
  | awk "!/($diff_crates)/{print}" \
  | tr '\n' ' ' \
  | xargs \
  | sed "s/ /\" }\, { \"crate\": \"/g")

echo "[ { \"crate\": \"$excluded_diff\" } ]"
