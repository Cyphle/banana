resource "scaleway_redis_cluster" "banana_redis" {
  name         = "banana-redis"
  version      = "7.0.5"
  node_type    = "RED1-MICRO"
  cluster_size = 1
  user_name    = var.redis_user
  password     = var.redis_password

  private_network {
    id = data.scaleway_vpc_private_network.private_net.id
  }
}