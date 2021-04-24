#!/bin/sh -e

if [ -z "$PROJECT_ID" ]; then
  echo "PROJECT_ID not set"
  exit 1
fi

secret_name=rocket-sqlite-test-sa
service_account_name=rocket-sa

temp_dir=$(mktemp -d)
file=$temp_dir/account.json

# Generate service account key
echo "Generating key into $file"
gcloud iam service-accounts keys create $file  \
  --iam-account=${service_account_name}@${PROJECT_ID}.iam.gserviceaccount.com

# Upload key to secret manager
gcloud secrets versions add $secret_name --data-file=$file --project=$PROJECT_ID

# Delete the created key
rm -rf $temp_dir

# Delete all old keys
old_keys=$(gcloud iam service-accounts keys list \
    --managed-by=user \
    --iam-account $service_account_name@${PROJECT_ID}.iam.gserviceaccount.com \
    | tail -n +2 | sed  -e '$ d') # Skip first line (header) + Skip last line (key just created)
    
echo "$old_keys"
echo "$old_keys" | while IFS= read -r key ; do
  key_id=$(echo $key | cut -d' ' -f1)
  echo "Delete $key_id"
  echo y | gcloud iam service-accounts keys delete $key_id \
    --iam-account ${service_account_name}@${PROJECT_ID}.iam.gserviceaccount.com
done
