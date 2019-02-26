use self::notify::raw_watcher;
use self::notify::Op;
use self::notify::RawEvent;
use notify;
use notify::{RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::RecvError;

#[allow(dead_code)]
enum WatchEvent {
    NewFile,
    NewFolder,
    Modification,
    Deletion,
}

#[derive(Debug, Clone)]
pub struct DriveEvent {
    pub op: Op,
    pub path: PathBuf,
    pub cookie: Option<u32>,
}

#[derive(Debug, Default, Clone)]
pub struct DriveWatcher {
    current_event: Option<DriveEvent>,
    raw_events: Vec<Option<DriveEvent>>,
    events: Vec<Option<DriveEvent>>,
    error_events: Vec<RecvError>,
}

impl DriveWatcher {
    pub fn new() -> DriveWatcher {
        DriveWatcher {
            current_event: None,
            raw_events: Vec::new(),
            events: Vec::new(),
            error_events: Vec::new(),
        }
    }

    pub fn current_event(&self) -> &Option<DriveEvent> {
        &self.current_event
    }

    pub fn raw_events(&self) -> &Vec<Option<DriveEvent>> {
        &self.raw_events
    }

    pub fn error_events(&self) -> &Vec<RecvError> {
        &self.error_events
    }

    fn add_drive_event(&mut self, drive_event: Option<DriveEvent>) {
        self.raw_events.push(drive_event);
    }

    pub fn watch(&mut self, path: &str) -> notify::Result<()> {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = raw_watcher(tx).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path, RecursiveMode::Recursive)?;

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.

        loop {
            match rx.recv() {
                Ok(RawEvent {
                    path: Some(path),
                    op: Ok(op),
                    cookie,
                }) => {
                    println!("{:?} {:?} ({:?})", op, path, cookie);
                    let drive_event = DriveEvent { op, path, cookie };

                    if self.current_event.is_some() {
                        let current_event = self.current_event.to_owned();
                        self.add_drive_event(current_event);
                    }
                    self.current_event = Some(drive_event);
                },
                Ok(event) => {
                    println!("{:?}", event);

                    let drive_event = DriveEvent {
                        op: event.op.expect(""),
                        path: event.path.expect("").clone(),
                        cookie: event.cookie,
                    };
                    self.events.push(Some(drive_event));
                },
                Err(e) => {
                    println!("watch error: {:?}", e);
                    self.error_events.push(e);
                },
            }
        }
    }

    pub fn watch_drive_notify(&mut self, path: &str) {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = raw_watcher(tx).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path, RecursiveMode::Recursive).unwrap();

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.

        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                let drive_event = DriveEvent { op, path, cookie };

                if self.current_event.is_some() {
                    let current_event = self.current_event.to_owned();
                    self.add_drive_event(current_event);
                }
                self.current_event = Some(drive_event);
            },
            Ok(event) => {
                println!("{:?}", event);

                let drive_event = DriveEvent {
                    op: event.op.expect(""),
                    path: event.path.expect("").clone(),
                    cookie: event.cookie,
                };
                self.events.push(Some(drive_event));
            },
            Err(e) => {
                println!("watch error: {:?}", e);
                self.error_events.push(e);
            },
        }
    }
}
