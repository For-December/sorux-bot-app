use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref WRAPPER_BOT_LOGIN_CHANNEL: (Arc<Mutex<Sender<bool>>>, Arc<Mutex<Receiver<bool>>>) = {
        let (tx, rx) = mpsc::channel::<bool>();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
}
