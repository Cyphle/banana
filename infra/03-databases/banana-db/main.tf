resource "scaleway_rdb_instance" "banana-db" {
  name          = "banana-db"
  engine        = "PostgreSQL-16"
  node_type     = "DB-DEV-S"
  is_ha_cluster = false
  user_name     = var.db_user
  password      = var.db_password

  private_network {
    pn_id      = data.scaleway_vpc_private_network.private_net.id
    enable_ipam = true
  }
}

resource "scaleway_rdb_database" "banana-db" {
  name        = "bananadb"
  instance_id = scaleway_rdb_instance.banana-db.id
}

resource "scaleway_rdb_privilege" "banana_user_privileges" {
  instance_id   = scaleway_rdb_instance.banana-db.id
  user_name     = var.db_user
  database_name = scaleway_rdb_database.banana-db.name
  permission    = "all"
}
