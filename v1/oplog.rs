use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs::File;
use uuid::Uuid;


#[derive(Debug)]
pub struct OpLog {
    seqno: Uuid,
    log_arc: Arc<Mutex<HashMap<i32, message::ProtocolMessage>>>,
    path: String,
    lf: File,
}

impl OpLog {
    pub fn new(fpath: String) -> OpLog {
        let mut log_file = File::open(fpath)?;
        OpLog {
            seqno = Uuid.new_v4(),
            log_arc = Arc::new(Mutex::new(HashMap::new())),
            path = fpath,
            lf = log_file
        }
    }
}