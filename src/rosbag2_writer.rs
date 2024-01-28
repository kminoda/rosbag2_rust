use rusqlite::{params, Connection, Result};

pub struct Rosbag2Writer {
    pub conn: Connection,
}

impl Rosbag2Writer {
    pub fn new (db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Rosbag2Writer::create_tables(&conn)?;
        Ok(Rosbag2Writer { conn })
    }

    fn create_tables(conn: &Connection) -> Result<()> {
        // Create the 'topics' table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS topics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                serialization_format TEXT NOT NULL,
                offered_qos_profiles TEXT NOT NULL
            )",
            params![],
        )?;

        // Create the 'messages' table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY,
                topic_id INTEGER NOT NULL,
                timestamp INTEGER NOT NULL,
                data BLOB NOT NULL
            )",
            params![],
        )?;

        // Create an index on the 'timestamp' column of the 'messages' table
        conn.execute(
            "CREATE INDEX IF NOT EXISTS timestamp_idx ON messages (timestamp ASC)",
            params![],
        )?;

        Ok(())
    }

    pub fn add_connection(&self, name: &str, msg_type: &str, serialization_format: &str, offered_qos_profiles: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO topics (name, type, serialization_format, offered_qos_profiles) VALUES (?1, ?2, ?3, ?4)",
            params![name, msg_type, serialization_format, offered_qos_profiles],
        )?;
    
        Ok(())
    }

    pub fn write_message(&self, topic_name: &str, timestamp: i64, data: Vec<u8>) -> Result<()> {
        let _topic_id = self.get_topic_id(topic_name)?;

        self.conn.execute(
            "INSERT INTO messages (topic_id, timestamp, data) VALUES (?1, ?2, ?3)",
            params![_topic_id, timestamp, data],
        )?;

        Ok(())
    }

    fn get_topic_id(&self, topic_name: &str) -> Result<i64> {
        self.conn.query_row(
            "SELECT id FROM topics WHERE name = ?1",
            params![topic_name],
            |row| row.get(0),
        ).map_err(|e| {
            eprintln!("Topic '{}' not found.", topic_name);
            e
        })
    }
}