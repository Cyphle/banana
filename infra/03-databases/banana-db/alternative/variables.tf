variable "scaleway_access_key" {
  type = string
  sensitive = true
}

variable "scaleway_secret_key" {
  type = string
  sensitive = true
}

variable "organization_id" {
  type = string
  sensitive = true
}

variable "project_id" {
  type = string
  sensitive = true
}

variable "region" {
  type = string
  default = "fr-par"
}

variable "zone" {
  type = string
  default = "fr-par-1"
}

variable "db_user" {
  type = string
  sensitive = true
}

variable "rdb_instance_name" {
  description = "Name of the existing Scaleway RDB instance to reuse"
  type        = string
  default     = "banana-keycloak-db"
}
