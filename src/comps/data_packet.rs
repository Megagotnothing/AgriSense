use crate::comps::{components::*};
use crate::database::TableColumnNames as Col;
#[derive(Serialize, Deserialize, Debug)]
pub struct DataPacket{
    // #[serde(with = "ts_seconds")]
    pub date_time : DateTime<Utc>,
    pub frequency: u64,
    pub duration : u64,
    pub amount   : u64,
    pub sensor_id : u64,
}

impl DataPacket{

    pub fn push_packet(&self) -> Result<(), rusqlite::Error>{
        let date_time = Database::add_packet(self).unwrap();
        println!("@ {date_time} > New packet added!\n");
        Ok(())
    }
    pub fn pull_packets(sensor_id: u64, query: &Option<Query>) -> Option<Vec<Self>>{
        DataPacket::pull(sensor_id, query).ok()
    }
    
    fn pull(sensor_id: u64, query: &Option<Query>) -> Result<Vec<Self>, rusqlite::Error>{
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::DATA_PACKET, TableColumnNames::SENSOR_ID);
        let mut statement = conn.prepare(&format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        let packet_iter = statement.query_map(params![sensor_id], 
            |row|{
                Ok(
                    DataPacket{
                        date_time: row.get(0)?,
                        frequency: row.get(1)?,
                        duration: row.get(2)?,
                        amount: row.get(3)?,
                        sensor_id: sensor_id
                    }
                )
            })?;
        packet_iter.collect()
    }
}