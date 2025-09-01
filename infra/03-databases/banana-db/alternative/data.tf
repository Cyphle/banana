data "scaleway_rdb_instance" "banana-keycloak-db" {
  name = var.rdb_instance_name
}