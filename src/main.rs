use std::error::Error;

use redis::{Client, AsyncCommands, streams::StreamRangeReply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establishing connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_tokio_connection().await?;
    
    println!("->> Connection to redis established!\n");
    
    // Setting a value 
    connection.set("my_key", "Hello, World!").await?;

    // Getting a value
    let result: String = connection.get("my_key").await?;

    println!("->> my_key: {}\n", result);

    // Add to redis as a queue
    // xadd to redis stream (to mirror the redis protocol).
    // If the stream name doesn't exist, it'll be created on the fly.
    // Adding a wildcard to the id, will let redis generate the id.
    // You suffix the entries as key-value pairs in the syntax of: array of tuples.
    connection.xadd("my-stream", "*", &[("name", "name-1"), ("title", "title-01")]).await?;

    // Number of messages for the given stream key
    let len: i32 = connection.xlen("my-stream").await?;
    println!("->> my-stream len: {}\n", len);

    // Read queue from latest to oldest
    let result: Option<StreamRangeReply> = connection.xrevrange_count(
        "my-stream", 
        "+", 
        "-", 
        10).await?;

    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("->> xrevrange stream entity: {} ", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!("  ->> {}: {}", name, redis::from_redis_value::<String>(value)?);
            }
            println!();
        }
    }

    // Cleanup
    connection.del("my_key").await?;
    connection.del("my-stream").await?;

    println!("->> The End.");

    Ok(())
}
