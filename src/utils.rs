// for asynchronus runtime
use async_std::prelude::*;

// for serialization / deserializatio
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::error::Error;
use std::marker::Unpin;

// Defineing Error type
// dyn Error is the behavior of air types that can be thrown and caught by rust error handling
// Send and Sync trait makes an object sharable between threads
// 'static lifetime'
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;

// Result Type
pub type ChatResult<T> = Result<T, ChatError>;

// function to send json

pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    // packet argument is serialized into json
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    // writing the json to the leaving
    leaving.write_all(json.as_bytes()).await?;
    Ok(())
}

// function to recieve a packet

pub fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
    I: async_std::io::BufRead + Unpin,
    T: DeserializeOwned,
{
  // incoming is a buffered reader returning a stream of json
    incoming.lines().map(|line| -> ChatResult<T> {
        let li = line?;
        let msg = serde_json::from_str::<T>(&li)?;
        Ok(msg)
    })
}
