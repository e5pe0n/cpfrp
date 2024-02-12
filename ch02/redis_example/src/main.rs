use redis::{Commands, RedisResult};

fn main() -> RedisResult<()> {
    let client = redis::Client::open("redis://redis/")?;
    let mut conn = client.get_connection()?;

    conn.set("aKey", "a string")?;
    conn.set("anotherKey", 4567)?;
    conn.set(45, 12345)?;

    println!(
        "{}, {}, {}, {:?}, {}.",
        conn.get::<_, String>("aKey")?,
        conn.get::<_, u64>("anotherKey")?,
        conn.get::<_, u16>(45)?,
        conn.get::<_, String>(40),
        conn.exists::<_, bool>(40)?,
    );
    // rust  | a string, 4567, 12345, Err(Response was of incompatible type - TypeError: "Response type not string compatible." (response was nil)), false.

    Ok(())
}
