pub mod jsonio;
pub mod req;
pub mod wssock;

use crate::process::req::Req;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use url::percent_encoding::percent_decode;

pub struct ReqBuf;

impl ReqBuf {
    pub fn start(addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(4);
        for stream in listener.incoming() {
            let stream = stream.expect("Could not get stream");
            pool.execute(|| {
                ReqBuf::connect(stream);
            });
        }
    }

    fn connect(mut stream: TcpStream) {
        let mut buffer = [0; 10000];
        stream.read(&mut buffer).unwrap();

        let decoded = percent_decode(&buffer).decode_utf8().unwrap();

        let (status_line, filename) = Req::on_request(&decoded)
            .unwrap_or_else(|| ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html"));

        let contents: String = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);

        stream
            .write(response.as_bytes())
            .expect("failed to return html page");
        stream.flush().unwrap();
    }
}

/*
Part of the thread handling is from the Rust book as boilerplate.
TODO: Implement thread handling to better suit the project
*/

enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Failed to to threads");
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    job.call_box();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
