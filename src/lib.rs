use std::{sync::{mpsc, Arc, Mutex}, thread::{spawn, JoinHandle}};

pub struct ThreadPool {
  workers : Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>
}

struct Worker {
  id: usize,
  thread: Option<JoinHandle<()>>
}

type Job = Box<dyn FnOnce() + Send + 'static >;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
      assert!(size > 0);
      let mut workers = Vec::with_capacity(4);
      let (sender, reveiver) = mpsc::channel();
      let reveiver = Arc::new(Mutex::new(reveiver));
      for id in 0..size {
        workers.push(Worker::new(id+1, Arc::clone(&reveiver)))
      };
      ThreadPool {
        workers,
        sender: Some(sender)
      }
    }

    pub fn execute<F>(&self, f: F) 
    where 
      F : FnOnce() + Send + 'static
    {
      self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Worker {
    fn new (id: usize, reveiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      let thread = spawn(move|| loop {
        let result = reveiver.lock().unwrap().recv();
        match result {
            Ok(job) => {
              println!("worker {id} got a job!");
              job()
            },
            Err(_) => {
              println!("worker {id} left");
              break;
            }
        }
      });
      Worker { id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
      drop(self.sender.take());
        for worker in &mut self.workers {
          if let Some(t) = worker.thread.take() {
            println!("worker {} shut down", worker.id);
            t.join().unwrap()
          }
        }
    }
}