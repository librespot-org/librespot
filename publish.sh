#!/bin/bash

WORKINGDIR="$( cd "$(dirname "$0")" ; pwd -P )"
cd $WORKINGDIR

crates=( "protocol" "core" "audio" "metadata" "playback" "connect" "librespot" )

function switchBranch {
  # You are expected to have committed/stashed your changes before running this.
  echo "Switching to master branch and merging development."
  git checkout master
  git pull
  git merge dev
}

function updateVersion {
  for CRATE in "${crates[@]}"
  do
    if [ "$CRATE" = "librespot" ]
    then
      CRATE=''
    fi
    crate_path="$WORKINGDIR/$CRATE/Cargo.toml"
    crate_path=${crate_path//\/\///}
    sed -i '' "s/^version.*/version = \"$1\"/g" "$crate_path"
    echo "Path is $crate_path"
    if [ "$CRATE" = "librespot" ]
    then
      cargo update
      git add . && git commit -a -m "Update Cargo.lock"
    fi
  done
}

function commitAndTag {
  git commit -a -m "Update version numbers to $1"
  git tag "v$1" -a -m "Update to version $1"
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
      cargo publish --no-verify
    else
      cargo publish
    fi
    echo "Successfully published $crate_name to crates.io"
    remoteWait 30 $crate_name
  done
}

function updateRepo {
  cd $WORKINGDIR
  echo "Pushing to master branch of repo."
  git push origin master
  echo "Pushing v$1 tag to master branch of repo."
  git push origin v$1
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

# First argument is new version number.
run $1
