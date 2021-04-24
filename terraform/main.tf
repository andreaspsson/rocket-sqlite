// Common variables, required
variable "project_id" {}
variable "build_version" {}
variable "bucket_name" {}

provider "google" {
  // https://registry.terraform.io/providers/hashicorp/google/latest
  version = "3.28.0"

  project = var.project_id
  region  = "europe-west1"
  zone    = "europe-west1-d"
}

provider "google-beta" {
  version = "3.28.0"

  project = var.project_id
  region  = "europe-west1"
  zone    = "europe-west1-d"
}
