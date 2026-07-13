use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

type Store = Arc<Mutex<HashMap<String, String>>>;

type Task = Box<dyn FnOnce() + Send + 'static>;

struct Runner {
    handle: Option<thread::JoinHandle<()>>,
}

pub struct WorkerPool {
    runners: Vec<Runner>,
    dispatch: Option<mpsc::Sender<Task>>,
}

impl WorkerPool {
    pub fn new(size: usize) -> WorkerPool {
        let (dispatch, queue) = mpsc::channel::<Task>();
        let queue = Arc::new(Mutex::new(queue));

        let mut runners = Vec::with_capacity(size);
        for _ in 0..size {
            let queue = Arc::clone(&queue);
            let handle = thread::spawn(move || loop {
                let Ok(task) = queue.lock().unwrap().recv() else {
                    break;
                };
                task();
            });
            runners.push(Runner { handle: Some(handle)});
        }

        WorkerPool { runners, dispatch: Some(dispatch) }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,
    {
        self.dispatch.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        drop(self.dispatch.take());

        for runner in &mut self.runners {
            if let Some(handle) = runner.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

pub fn run(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let pool = WorkerPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let store = Arc::clone(&store);
        pool.execute(move || handle_connection(stream, store));
    }
}

fn handle_connection(stream: TcpStream, store: Store) {
    let mut writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let Ok(line) = line else { break };

        let mut parts = line.trim().splitn(3, ' ');
        let reply = match parts.next().unwrap_or("") {
            "SET" => {
                let key = parts.next().unwrap_or("").to_string();
                let value = parts.next().unwrap_or("").to_string();
                store.lock().unwrap().insert(key, value);
                "OK\n".to_string()
            }
            "GET" => {
                let key = parts.next().unwrap_or("");
                let found = store.lock().unwrap().get(key).cloned();
                match found {
                    Some(value) => format!("{value}\n"),
                    None => "NOT FOUND\n".to_string()
                }
            }
            "DELETE" => {
                let key = parts.next().unwrap_or("");
                store.lock().unwrap().remove(key);
                "OK\n".to_string()
            }
            _ => "UNKNOWN COMMAND\n".to_string(),
        };

        if writer.write_all(reply.as_bytes()).is_err() {
            break;
        }
    }
}