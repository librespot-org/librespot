#!/bin/bash

SKIP_MERGE='false'
DRY_RUN='false'

WORKINGDIR="$( cd "$(dirname "$0")" ; pwd -P )"
cd $WORKINGDIR

crates=( "protocol" "core" "discovery" "oauth" "audio" "metadata" "playback" "connect" "librespot" )

OS=`uname`
function replace_in_file() {
    if [ "$OS" == 'darwin' ]; then
        # for MacOS
        sed -i '' -e "$1" "$2"
    else
        # for Linux and Windows
        sed -i'' -e "$1" "$2"
    fi
}

function switchBranch {
  if [ "$SKIP_MERGE" = 'false' ] ; then
    # You are expected to have committed/stashed your changes before running this.
    echo "Switching to master branch and merging development."
    git checkout master
    git pull
    if [ "$DRY_RUN" = 'true' ] ; then
      git merge --no-commit --no-ff dev
    else
      git merge dev
    fi
  fi
}

function updateVersion {
  for CRATE in "${crates[@]}"
  do
    if [ "$CRATE" = "librespot" ]
    then
      CRATE_DIR=''
    else
      CRATE_DIR=$CRATE
    fi
    crate_path="$WORKINGDIR/$CRATE_DIR/Cargo.toml"
    crate_path=${crate_path//\/\///}
    $(replace_in_file "s/^version.*/version = \"$1\"/g" "$crate_path")
    echo "Path is $crate_path"
    if [ "$CRATE" = "librespot" ]
    then
      echo "Updating lockfile"
      if [ "$DRY_RUN" = 'true' ] ; then
        cargo update --dry-run
        git add . && git commit --dry-run -a -m "Update Cargo.lock"
      else
        cargo update
        git add . && git commit -a -m "Update Cargo.lock"
      fi
    fi
  done
}

function commitAndTag {
  if [ "$DRY_RUN" = 'true' ] ; then
    # Skip tagging on dry run.
    git commit --dry-run -a -m "Update version numbers to $1"
  else
    git commit -a -m "Update version numbers to $1"
    git tag "v$1" -a -m "Update to version $1"
  fi
}

function get_crate_name {
  awk -v FS="name = " 'NF>1{print $2; exit}' Cargo.toml
}

function remoteWait() {
  IFS=:
  secs=${1}
  crate_name=${2}
  while [ $secs -gt 0 ]
  do
    sleep 1 &
    printf "\rSleeping to allow %s to propagate on crates.io servers. Continuing in %2d second(s)." ${crate_name} ${secs}
    secs=$(( $secs - 1 ))
    wait
  done
  echo
}

function publishCrates {
  for CRATE in "${crates[@]}"
  do
    if [ "$CRATE" = "librespot" ]
    then
      CRATE=''
    fi

    crate_path="$WORKINGDIR/$CRATE"
    crate_path=${crate_path//\/\///}
    cd $crate_path
    # Also need to update Cargo.lock in root directory
    crate_name=`echo $( awk -v FS="name = " 'NF>1{print $2; exit}' Cargo.toml )`
    echo "Publishing $crate_name to crates.io"
    if [ "$CRATE" == "protocol" ]
    then
      # Protocol crate needs --no-verify option due to build.rs modification.
      if [ "$DRY_RUN" = 'true' ] ; then
        cargo publish --no-verify --dry-run
      else
        cargo publish --no-verify
      fi
    else
      if [ "$DRY_RUN" = 'true' ] ; then
        cargo publish --dry-run
      else
        cargo publish
      fi
    fi
    echo "Successfully published $crate_name to crates.io"
    remoteWait 30 $crate_name
  done
}

function updateRepo {
  cd $WORKINGDIR
  if [ "$DRY_RUN" = 'true' ] ; then
    echo "Pushing to master branch of repo. [DRY RUN]"
    git push --dry-run origin master
    echo "Pushing v$1 tag to master branch of repo. [DRY RUN]"
    git push --dry-run origin v$1

    # Cancels any merges in progress
    git merge --abort

    git checkout dev
    git merge --no-commit --no-ff master

    # Cancels above merge
    git merge --abort

    git push --dry-run
  else
    echo "Pushing to master branch of repo."
    git push origin master
    echo "Pushing v$1 tag to master branch of repo."
    git push origin v$1
    # Update the dev repo with latest version commit
    git checkout dev
    git merge master
    git push
  fi
}

function rebaseDev {
  git checkout dev
  git merge master
  git push
}

function run {
  switchBranch
  updateVersion $1
  commitAndTag $1
  publishCrates
  updateRepo $1
  rebaseDev
  echo "Successfully published v$1 to crates.io and uploaded changes to repo."
}

#Set Script Name variable
SCRIPT=`basename ${BASH_SOURCE[0]}`

print_usage () {
  local l_MSG=$1
  if [ ! -z "${l_MSG}" ]; then
    echo "Usage Error: $l_MSG"
  fi
  echo "Usage: $SCRIPT <args> <version>"
  echo "  where <version> specifies the version number in semver format, eg. 1.0.1"
  echo "Recognized optional command line arguments"
  echo "--dry-run -- Test the script before making live changes"
  echo "--skip-merge -- Skip merging dev into master before publishing"
  exit 1
}

### check number of command line arguments
NUMARGS=$#
if [ $NUMARGS -eq 0 ]; then
  print_usage 'No command line arguments specified'
fi

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      print_usage
      exit 0
      ;;
      --dry-run)
        DRY_RUN='true'
        shift
        ;;
    --skip-merge)
      SKIP_MERGE='true'
      shift
      ;;
    *)
      break
      ;;
  esac
done

# First argument is new version number.
run $1
