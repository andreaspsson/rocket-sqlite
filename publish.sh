#!/bin/sh -e

DOCKER_HOST_NAME=eu.gcr.io

# Validate environment
[ "$PROJECT_ID" ] || {
  echo "PROJECT_ID not set"
  exit 1
}

# Parse commands
usage() {
  echo "Usage: $0 -v <version> [-p <release | debug>]" 1>&2;
  exit 1;
}

while getopts ":v:p:" o; do
    case "${o}" in
        v)
            build_version=${OPTARG}
            ;;
        p)
            profile=${OPTARG}
            [ "$profile" == "release" ] || [ "$profile" == "debug" ] || usage
            ;;
        *)
            usage
            ;;
    esac
done
shift $((OPTIND-1))
[ "$build_version" ] || usage
[ "$profile" ] || {
  profile=debug
}


# Build binary
# Make sure to have linux toolchain installed
# rustup target add x86_64-unknown-linux-musl
# And musl cross tooling
# brew install filosottile/musl-cross/musl-cross
# And the compiler
# ln -s /usr/local/opt/musl-cross/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
build_args=
[ $profile == "relese" ] && {
  build_args="$build_args --release"
}
cargo build $build_args --target=x86_64-unknown-linux-musl

# Build docker image
rm -rf target/cloud-run || true
mkdir target/cloud-run
cp target/x86_64-unknown-linux-musl/$profile/rocket-sqlite target/cloud-run
cp deployment/resources/* target/cloud-run
docker build target/cloud-run --tag "$DOCKER_HOST_NAME/$PROJECT_ID/rocket-sqlite:$build_version"

# Push docker image
docker push "$DOCKER_HOST_NAME/$PROJECT_ID/rocket-sqlite:$build_version"

