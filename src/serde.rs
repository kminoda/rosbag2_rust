use rusqlite::Result;

pub fn serialize_cdr(data: bool, msg_type: &str) -> Result<Vec<u8>> {
    match msg_type {
        "std_msgs/msg/Bool" => Ok(vec![data as u8]),
        _ => Err(rusqlite::Error::InvalidQuery)
    }
}