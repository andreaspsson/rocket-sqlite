#!/bin/sh -e
[ -e .build-conf ] && {
  echo "Loading conf from .build.conf"
  source .build-conf
}

# Generate version, used for tagging the image
version=$(git describe --match=NeVeRmAtCh --always --abbrev=40 --dirty)

# Build and publish docker image
echo "Building $version"
./publish.sh -v $version -p debug

# Apply terraform config
echo "Apply changes to cloud env"
./terraform/terraform.sh -v $version -c apply