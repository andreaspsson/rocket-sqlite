#!/bin/sh
id=$(date +%s)
echo "$id: Launching using $BUILD_VERSION"
echo "$id: Validating environment"
if [ -z "$PROJECT_ID" ]; then
  echo "$id: Required evironment variable PROJECT_ID not found"
  exit 1
fi

echo "$id: Loading secrets"
export SERVICE_ACCOUNT_JSON=$(gcloud secrets versions access latest --secret=rocket-sqlite-test-sa --project=$PROJECT_ID)

# SQLite file management
echo "$id: Download db file"
db_file_name=test-db.db
export SQLITE_DB_FILE=/tmp/$db_file_name
# Download file
gsutil cp gs://$BUCKET_NAME/$db_file_name $SQLITE_DB_FILE

#File uploader
upload() {
  gsutil cp $SQLITE_DB_FILE gs://$BUCKET_NAME/$db_file_name
}

echo "$id: Setting up SIGTERM listener"
_term() { 
  echo "$id: Caught SIGTERM signal!" 
  kill -TERM "$child" 2>/dev/null
}
trap _term SIGTERM

echo "$id: Running as $$"

# Launch web service
echo "$id: Launching web server"
/usr/local/bin/rocket-sqlite &
child=$! 
wait "$child"

# Upload db file
echo "$id: Service is down upload the database file"
upload
echo "$id: DB upload complete!"
