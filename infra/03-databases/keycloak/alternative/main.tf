# When there is already a managed postgres instance. Use it instead of creating a new one
resource "scaleway_rdb_database" "banana-db" {
  name        = "keycloak"
  instance_id = data.scaleway_rdb_instance.banana-db.id
}

resource "scaleway_rdb_privilege" "banana_user_privileges" {
  instance_id   = data.scaleway_rdb_instance.banana-db.id
  user_name     = data.scaleway_rdb_instance.banana-db.user_name
  database_name = scaleway_rdb_database.banana-db.name
  permission    = "all"
}
