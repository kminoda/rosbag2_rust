mod rosbag2_writer;
mod serde;

use rosbag2_writer::Rosbag2Writer;
use rusqlite::Result;
use serde::serialize_cdr;

fn main() -> Result<()> {
    let writer = Rosbag2Writer::new("my_database.db3")?;
    writer.add_connection("/flag", "std_msgs/msg/Bool", "cdr", "")?;
    let message_data = serialize_cdr(true, "std_msgs/msg/Bool")?;
    let timestamp = 1704070800000000000;
    writer.write_message("/flag", timestamp, message_data)?;
    Ok(())
}
