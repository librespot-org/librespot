CRATES="protocol oauth core discovery audio metadata playback connect"

CURRENT_TAG=$(git describe --abbrev=0)
LAST_TAG=$(git describe --abbrev=0 $CURRENT_TAG^)

BIN_VERSION=$(cat ./Cargo.toml | awk "/version/{print; exit}" | cut -d\" -f 2)

SIMPLE_VERSION_REGEX="^v?([0-9]+)\.([0-9]+)\.([0-9]+)$"
if [[ $LAST_TAG =~ $SIMPLE_VERSION_REGEX ]]; then
  last_major="${BASH_REMATCH[1]}"
  last_minor="${BASH_REMATCH[2]}"
  last_patch="${BASH_REMATCH[3]}"
else
  echo "regex for tag didn't match"
  exit 1
fi

if [[ $BIN_VERSION =~ $SIMPLE_VERSION_REGEX ]]; then
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
AWK_CRATES=$(echo "$CRATES" | tr ' ' '|')
DIFF_CRATES=$(git diff $LAST_TAG... --stat --name-only \
  | awk '/(rs|proto)$/{print}' \
  | awk "/($AWK_CRATES)/{print}" \
  | cut -d '/' -f 1 \
  | uniq \
  | tr '\n' ' ' \
  | xargs \
  | tr ' ' '|' )

EXCLUDED_DIFF=$(echo $CRATES \
  | tr ' ' '\n' \
  | awk "!/($DIFF_CRATES)/{print}" \
  | tr '\n' ' ' \
  | xargs \
  | sed "s/ /\" }\, { \"crate\": \"/g")

echo "[ { \"crate\": \"$EXCLUDED_DIFF\" } ]"
