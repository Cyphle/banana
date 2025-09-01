data "scaleway_rdb_instance" "banana-db" {
  name = var.rdb_instance_name
}