use std::collections::VecDeque;
use std::sync::{
    Arc, Mutex, Condvar,
    atomic::{AtomicBool, AtomicUsize, Ordering}
};
use std::thread;
use std::time::Duration;

// Task Code: Below is the code that sets the structure for tasks
// It includes priority levels, metadata, and payload handling.

pub type TaskId = u64;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
pub struct TaskMetadata {
    pub created_at: u64,
    pub cancelled: Arc<AtomicBool>,
}

impl TaskMetadata {
    pub fn new() -> Self {
        Self {
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub struct Task {
    pub id: TaskId,
    pub priority: Priority,
    pub payload: Box<dyn TaskPayload>,
    pub metadata: TaskMetadata,
}

pub trait TaskPayload: Send + Sync + 'static {
    fn execute(&self) -> Result<TaskResult, TaskError>;
    fn cancel(&self) -> Result<(), TaskError>;
}

#[derive(Debug)]
pub struct TaskResult {
    pub message: String,
}

#[derive(Debug)]
pub struct TaskError {
    pub message: String,
}

//Work Queue Code: This section implements the work queue
// It includes task submission, fetching, and shutdown mechanisms.

pub struct WorkQueue {
    queues: Mutex<[VecDeque<Arc<Task>>; 3]>,
    cv: Condvar,

    pub queued_tasks: AtomicUsize,
    pub completed_tasks: AtomicUsize,
    pub cancelled_tasks: AtomicUsize,

    shutdown: AtomicBool,
}

impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queues: Mutex::new([
                VecDeque::new(), // High
                VecDeque::new(), // Medium
                VecDeque::new(), // Low
            ]),
            cv: Condvar::new(),

            queued_tasks: AtomicUsize::new(0),
            completed_tasks: AtomicUsize::new(0),
            cancelled_tasks: AtomicUsize::new(0),

            shutdown: AtomicBool::new(false),
        })
    }

    fn idx(priority: Priority) -> usize {
        match priority {
            Priority::High => 0,
            Priority::Medium => 1,
            Priority::Low => 2,
        }
    }

    pub fn submit(&self, task: Arc<Task>) {
        let mut queues = self.queues.lock().unwrap();
        queues[Self::idx(task.priority)].push_back(task);
        self.queued_tasks.fetch_add(1, Ordering::Relaxed);
        self.cv.notify_one();
    }

    pub fn fetch(&self) -> Option<Arc<Task>> {
        let mut queues = self.queues.lock().unwrap();

        loop {
            if self.shutdown.load(Ordering::Acquire) {
                return None;
            }

            for q in queues.iter_mut() {
                if let Some(task) = q.pop_front() {
                    return Some(task);
                }
            }

            // Sleep until new tasks arrive
            queues = self.cv.wait(queues).unwrap();
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        self.cv.notify_all();
    }
}

// Worker Code: This section defines worker threads
// that process tasks from the work queue.


pub struct Worker {
    pub id: usize,
    pub alive: Arc<AtomicBool>,
    pub task_count: Arc<AtomicUsize>,
    pub error_count: Arc<AtomicUsize>,
}

impl Worker {
    pub fn spawn(queue: Arc<WorkQueue>, id: usize) -> Arc<Self> {
        let worker = Arc::new(Self {
            id,
            alive: Arc::new(AtomicBool::new(true)),
            task_count: Arc::new(AtomicUsize::new(0)),
            error_count: Arc::new(AtomicUsize::new(0)),
        });

        let worker_ref = worker.clone();

        thread::spawn(move || {
            while worker_ref.alive.load(Ordering::Acquire) {
                let task = match queue.fetch() {
                    Some(t) => t,
                    None => break,
                };

                // Handle cancellation
                if task.metadata.cancelled.load(Ordering::Acquire) {
                    let _ = task.payload.cancel();
                    queue.cancelled_tasks.fetch_add(1, Ordering::Relaxed);
                    continue;
                }

                worker_ref.task_count.fetch_add(1, Ordering::Relaxed);

                // Panic-safe execution
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| task.payload.execute())) {
                    Ok(Ok(_)) => {
                        queue.completed_tasks.fetch_add(1, Ordering::Relaxed);
                    }
                    Ok(Err(e)) => {
                        worker_ref.error_count.fetch_add(1, Ordering::Relaxed);
                        eprintln!("Worker {} error: {:?}", worker_ref.id, e);
                    }
                    Err(_) => {
                        worker_ref.error_count.fetch_add(1, Ordering::Relaxed);
                        eprintln!("Worker {} panicked!", worker_ref.id);
                        worker_ref.alive.store(false, Ordering::Release);
                        break; // Supervisor will restart it
                    }
                }
            }
        });

        worker
    }
}

// Supervisor Code: This section manages worker threads,
// ensuring the desired number of active workers.
// ===============================================================
// ====================== SUPERVISOR =============================
// ===============================================================
//

use std::sync::Mutex as StdMutex;

pub struct WorkerSupervisor {
    workers: StdMutex<Vec<Arc<Worker>>>,
    want_workers: usize,
    queue: Arc<WorkQueue>,
}

impl WorkerSupervisor {
    pub fn new(queue: Arc<WorkQueue>, want: usize) -> Arc<Self> {
        Arc::new(Self {
            workers: StdMutex::new(Vec::new()),
            want_workers: want,
            queue,
        })
    }

    pub fn start(self: &Arc<Self>) {
        let this = self.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            let mut workers = this.workers.lock().unwrap();

            // Remove dead workers
            workers.retain(|w| w.alive.load(Ordering::Acquire));

            // Start new workers
            while workers.len() < this.want_workers {
                let id = workers.len();
                let w = Worker::spawn(this.queue.clone(), id);
                workers.push(w);
            }
        });
    }
}

// Demo Task Code: A simple task implementation for demonstration
// Can be edited or replaced with more complex tasks.

struct PrintTask {
    msg: String,
}

impl TaskPayload for PrintTask {
    fn execute(&self) -> Result<TaskResult, TaskError> {
        println!("{}", self.msg);
        Ok(TaskResult { message: "ok".into() })
    }

    fn cancel(&self) -> Result<(), TaskError> {
        println!("Cancelled: {}", self.msg);
        Ok(())
    }
}

// Main: the entry point of the application


fn main() {
    let queue = WorkQueue::new();
    let supervisor = WorkerSupervisor::new(queue.clone(), 8);
    supervisor.start();

    // Send some test tasks
    for i in 0..1000 {
        let task = Arc::new(Task {
            id: i,
            priority: if i % 3 == 0 { Priority::High } else { Priority::Low },
            payload: Box::new(PrintTask {
                msg: format!("Task {}", i),
            }),
            metadata: TaskMetadata::new(),
        });

        queue.submit(task);
    }

    // Let workers work
    thread::sleep(Duration::from_secs(3));

    queue.shutdown();

    println!("\n--- Metrics ---");
    println!("Queued: {}", queue.queued_tasks.load(Ordering::Relaxed));
    println!("Completed: {}", queue.completed_tasks.load(Ordering::Relaxed));
    println!("Cancelled: {}", queue.cancelled_tasks.load(Ordering::Relaxed));
}
