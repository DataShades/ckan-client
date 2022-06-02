use crate::{types::Portal, action::FdpClient};

use std::sync::Mutex;

#[derive(Default)]
pub struct PortalState(Mutex<Portal>);

impl PortalState {
    pub fn replace(&self, portal: Portal) {
        *self.0.lock().unwrap() = portal;
    }

    pub fn clone(&self) -> Portal {
        let portal = self.0.lock().unwrap();
        portal.clone()
    }

    pub fn client(&self) -> Result<impl FdpClient, String> {
        match self.clone() {
            Portal {token: Some(ref token), url: Some(ref url)} => {
                Ok(ckanapi::CKAN::new(url).with_token(token))
            },
            _ => Err("URL and token must be defined".into()),

        }
    }
}
