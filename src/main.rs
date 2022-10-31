///
/// # Test d'une connection redis asynchrone
///
/// il faut ajouter les features suivantes :
///
/// * "tokio-comp"
/// * "aio"
/// * "connection-manager"
///


use redis::{AsyncCommands, Client, aio::{ConnectionManager}, RedisResult};

#[tokio::main]
async fn main() {
    let mut con = start_redis_connection().await.unwrap();
    let _ : () = con.set(10, "Joseph".to_string()).await.unwrap();

    let res: String = con.get(7).await.unwrap();
    println!("{res:?}");
    let res_b: String = con.get(8).await.unwrap();
    println!("{res_b:?}");
    let res_c: String = con.get(9).await.unwrap();
    println!("{res_c:?}");
    let res_d: String = con.get(10).await.unwrap();
    println!("{res_d:?}");
}

///
/// Retourne une Connection dans un ConnectionManager\
/// A ConnectionManager is a proxy that wraps a multiplexed connection\
/// and automatically reconnects to the server when necessary.\
/// On pourrait retourner une simple connection via _get_async_connection()_
///
pub async fn start_redis_connection() -> RedisResult<ConnectionManager> {
    let client = Client::open("redis://127.0.0.1/")?;
    //let con = client.get_async_connection().await;
    let con = ConnectionManager::new(client).await;
    con
}
