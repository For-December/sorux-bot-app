use std::{
    collections::HashMap,
    process::Child,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROVIDER_BOT_LOGIN_CHANNEL: (Arc<Mutex<Sender<bool>>>, Arc<Mutex<Receiver<bool>>>) = {
        let (tx, rx) = mpsc::channel::<bool>();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
}

lazy_static! {
    pub static ref CHILD_PROCESS_MAP: Mutex<HashMap<String, Child>> = Mutex::new(HashMap::new());
}

lazy_static! {
    pub static ref WRAPPER_LOGS_CHANNEL: (Arc<Mutex<Sender<String>>>, Arc<Mutex<Receiver<String>>>) = {
        let (tx, rx) = mpsc::channel::<String>();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
}
