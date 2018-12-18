use std::cell::Cell;
use std::rc::Rc;
use ws::{CloseCode, Handler, Handshake, Result, Sender};

/*
Boilerplate for the web socket communication for the browser authentication.
TODO: Not Yet Implemented
*/

pub struct WSock {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
}

impl Handler for WSock {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("The server encountered an error: {:?}", err);
    }
}
