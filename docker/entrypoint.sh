#!/bin/bash

echo "Validating environment"
if [ -z "$PROJECT_ID" ]; then
  echo "Required evironment variable PROJECT_ID not found"
  exit 1
fi

echo "Loading secrets"
export SERVICE_ACCOUNT_JSON=$(gcloud secrets versions access latest --secret=rocket-sqlite-test-sa --project=$PROJECT_ID)

# Launch web service
echo "Launching"
/usr/local/bin/rocket-sqlite