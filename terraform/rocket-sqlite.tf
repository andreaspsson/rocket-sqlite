
// Create a service account
resource "google_service_account" "rocket-sa" {
  account_id = "rocket-sa"
  display_name = "SA used to run rcket continer"
}

// Create bucket for db storage
resource "google_storage_bucket" "sqlite-db-bucket" {
  name          = var.bucket_name
  location      = "EU"
  force_destroy = true
}

// Add permissions to service account to allow it to access bucket
resource "google_storage_bucket_iam_binding" "binding" {
  bucket = google_storage_bucket.sqlite-db-bucket.name
  role = "roles/storage.admin"
  members = [
    "serviceAccount:${google_service_account.rocket-sa.email}"
  ]
}

resource "google_cloud_run_service" "rocket-sqlite-test" {
  name = "rocket-sqlite-test"
  location = "europe-west1"

  template {
    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = 1
      }
    }
    spec {
      containers {
        image = "eu.gcr.io/${var.project_id}/rocket-sqlite:${var.build_version}"
        env {
          name = "BUCKET_NAME"
          value = var.bucket_name
        }
        resources {
          limits = {
            cpu = "2000m"
            memory = "2048Mi"
          }
        }
      }
      container_concurrency = 0
      service_account_name = google_service_account.rocket-sa.email
      timeout_seconds = 900
    }
  }

  traffic {
    percent = 100
    latest_revision = true
  }
}

data "google_iam_policy" "noauth" {
  binding {
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }
}

resource "google_cloud_run_service_iam_policy" "noauth" {
  location    = google_cloud_run_service.rocket-sqlite-test.location
  project     = google_cloud_run_service.rocket-sqlite-test.project
  service     = google_cloud_run_service.rocket-sqlite-test.name

  policy_data = data.google_iam_policy.noauth.policy_data
}