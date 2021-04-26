# Rocket serverless sqlite

A highly experimental project that tries to run a [rocket](https://rocket.rs/) web server backed by [sqlite](https://www.sqlite.org/) in a serverless infrastructure.

The current version uses Google Cloud Run and Google Cloud Storage for deployment.
The server runs in a container in Google Cloud Run and downloads the sqlite database from Google Cloud Storage.
When the container is terminated it uploades the modified database file to the same Google Cloud Storage backup to persistently store any changes.

## Experiments and TODOS

* Measure time to load db large/medium/small file sizes
* Deploy, looks like there can be multiple instances of the function when deploying a new version

## Testing

Naej, no test we have so far, if this ever takes a step out of the experimentation box, tests will be added

## Configuration and deployment

* __publish.sh__ - Builds a docker image with the application + script to upload/download database ready for usage on Google Cloud Run
* __deploy.sh__ - Publishes a docker image using the _publish_ script, then creates/updates a cloud run function and other required infrastructure using [terraform](https://www.terraform.io/)

The deployment scripts use the following environment variables for configuration:

* __PROJECT_ID__ - GCP Project id
* __BUCKET_NAME__ - Bucket used to store the database file. The bucket will be created in the project PROJECT_ID upon first deploy.
