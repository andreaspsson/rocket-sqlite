#!/bin/sh -e

# Load and validate environment and arguments
[ "$PROJECT_ID" ] || {
  echo "PROJECT_ID not set" 1>&2;
  exit 1
}
[ "$BUCKET_NAME" ] || {
  echo "BUCKET_NAME not set" 1>&2;
  exit 1
}

usage() {
  echo "Usage: $0 -v <version> -c <plan | apply>" 1>&2;
  exit 1;
}

while getopts ":v:c:" o; do
    case "${o}" in
        v)
            build_version=${OPTARG}
            ;;
        c)
            command=${OPTARG}
            [ "$command" == "plan" ] || [ "$command" == "apply" ] || usage
            ;;
        *)
            usage
            ;;
    esac
done
shift $((OPTIND-1))

[ "$command" ] && [ "$build_version" ]  || usage

# Navigate to terraform folder
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd $DIR

# Select workspace for the selected project
terraform workspace select $PROJECT_ID || terraform workspace new $PROJECT_ID

# Run terraform
echo yes | terraform $command -input=false -var "project_id=$PROJECT_ID" -var "build_version=$build_version" -var "bucket_name=$BUCKET_NAME"
