fn data_paralelism_native() {
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let len = data.lock().unwrap().len();
    let mid = len / 2;


    let left_data = Arc::clone(&data);
    let left_handle = thread::spawn(move || {
        let mut left = left_data.lock().unwrap();
        for x in &mut left[..mid] {
            *x *= 2;
        }
    });

    let right_data = Arc::clone(&data);
    let right_handle = thread::spawn(move || {
        let mut right = right_data.lock().unwrap();
        for x in &mut right[mid..] {
            *x *= 2;
        }
    });

    left_handle.join().unwrap();
    right_handle.join().unwrap();

    println!("{:?}", *data.lock().unwrap()); // [2, 4, 6, 8, 10]
    
    
    // the main problem, I want you to notice, even though it seems like
    // a fine data paralelism, it has a huge problem.
    // because you are locking data vector at the moment of updating the value
    // it basically means, only one thread at a time can update value
    // even though logically it seems correct it's a overhead
}

fn data_paralelism_rayon() {

    use rayon::prelude::*;

    let mut data = vec![1, 2, 3, 4, 5];

    data.par_iter_mut().for_each(|x| {
        *x *= 2;
    });
    
    // from code perspective it seems trivial, but I want you to realize how much heavy lifting happens behind the hood:
    //The Rayon library uses work stealing to dynamically balance the workload among threads, 
    //providing better performance compared to a static division of work among threads.
    
    // on top it creates a separate scope to escape need to lock data
    
    
    
    // Concept of work stealing and separate scope.
    
    

    println!("{:?}", data); // [2, 4, 6, 8, 10]
}

#[allow(unused_variables)]
fn task_paralelism_native() {

    use std::thread;
    use std::time::{Duration, Instant};
    use std::sync::mpsc;
    
    fn download_file(file: &str) -> String {
        thread::sleep(Duration::from_millis(100));
        format!("{} downloaded", file)
    }
    
    fn resize_image(image: &str) -> String {
        thread::sleep(Duration::from_millis(100));
        format!("{} resized", image)
    }

    let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    let images = vec!["image1.jpg", "image2.jpg", "image3.jpg"];

    // Sequential execution
    let start = Instant::now();
    let downloaded_files: Vec<String> = files.iter().map(|file| download_file(file)).collect();
    let resized_images: Vec<String> = images.iter().map(|image| resize_image(image)).collect();
    let duration = start.elapsed();
    println!("Time elapsed in sequential execution: {:?}", duration);

    // Parallel execution using task parallelism with native Rust threads
    let start = Instant::now();
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    let download_handle = thread::spawn(move || {
        let result: Vec<String> = files.iter().map(|file| download_file(file)).collect();
        tx1.send(result).unwrap();
    });
    
    let resize_handle = thread::spawn(move || {
        let result: Vec<String> = images.iter().map(|image| resize_image(image)).collect();
        tx2.send(result).unwrap();
    });
    
    let downloaded_files = rx1.recv().unwrap();
    let resized_images = rx2.recv().unwrap();

    download_handle.join().unwrap();
    resize_handle.join().unwrap();

    let duration = start.elapsed();
    println!("Time elapsed in parallel execution (native Rust threads): {:?}", duration);

    println!("Downloaded files: {:?}", downloaded_files);
    println!("Resized images: {:?}", resized_images);
}

#[allow(unused_variables)]
fn task_paralelism_rayon(){
    extern crate rayon;
    use rayon::prelude::*;
    use std::time::{Duration, Instant};
    use std::thread;
    
    fn download_file(file: &str) -> String {
        thread::sleep(Duration::from_millis(100));
        format!("{} downloaded", file)
    }
    
    fn resize_image(image: &str) -> String {
        thread::sleep(Duration::from_millis(100));
        format!("{} resized", image)
    }
    

    let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    let images = vec!["image1.jpg", "image2.jpg", "image3.jpg"];

    // Sequential execution
    let start = Instant::now();
    let downloaded_files: Vec<String> = files.iter().map(|file| download_file(file)).collect();
    let resized_images: Vec<String> = images.iter().map(|image| resize_image(image)).collect();
    let duration = start.elapsed();
    println!("Time elapsed in sequential execution: {:?}", duration);

    // Parallel execution using task parallelism
    let start = Instant::now();
    let (downloaded_files, resized_images): (Vec<String>, Vec<String>) = rayon::join(
        || files.par_iter().map(|file| download_file(file)).collect(),
        || images.par_iter().map(|image| resize_image(image)).collect(),
    );
    let duration = start.elapsed();
    println!("Time elapsed in parallel execution: {:?}", duration);

    println!("Downloaded files: {:?}", downloaded_files);
    println!("Resized images: {:?}", resized_images);
}

#[allow(dead_code)]
fn pipeline_paralelism()
    {
        use std::sync::mpsc::{channel, Sender, Receiver};
        use std::thread;
        use std::time::Duration;
        use std::time::Instant;
        
        
        enum Message {
            Download(String),
            Process(String),
            Upload(String),
            Exit,
        }
        
        struct Downloader {
            tx: Sender<Message>,
        }
        
        impl Downloader {
            fn run(&self, files: &[&str]) {
                for file in files {
                    thread::sleep(Duration::from_millis(100)); // Simulate download time
                    let downloaded_file = format!("{} downloaded", file);
                    self.tx.send(Message::Download(downloaded_file)).unwrap();
                }
                self.tx.send(Message::Exit).unwrap();
            }
        }
        
        struct Processor {
            tx: Sender<Message>,
            rx: Receiver<Message>,
        }
        
        impl Processor {
            fn run(&self) {
                loop {
                    match self.rx.recv().unwrap() {
                        Message::Download(file) => {
                            thread::sleep(Duration::from_millis(100)); // Simulate processing time
                            let processed_file = format!("{} processed", file);
                            self.tx.send(Message::Process(processed_file)).unwrap();
                        }
                        Message::Exit => {
                            self.tx.send(Message::Exit).unwrap();
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        
        struct Uploader {
            rx: Receiver<Message>,
        }
        
        impl Uploader {
            fn run(&self) -> Vec<String> {
                let mut uploaded_files = Vec::new();
        
                loop {
                    match self.rx.recv().unwrap() {
                        Message::Process(file) => {
                            thread::sleep(Duration::from_millis(100)); // Simulate upload time
                            let uploaded_file = format!("{} uploaded", file);
                            uploaded_files.push(uploaded_file);
                        }
                        Message::Exit => {
                            break;
                        }
                        _ => {}
                    }
                }
        
                uploaded_files
            }
        }
        
        fn sequential(files: &[&str]) -> Vec<String> {
            let mut uploaded_files = Vec::new();
        
            for file in files {
                thread::sleep(Duration::from_millis(100)); // Simulate download time
                let downloaded_file = format!("{} downloaded", file);
        
                thread::sleep(Duration::from_millis(100)); // Simulate processing time
                let processed_file = format!("{} processed", downloaded_file);
        
                thread::sleep(Duration::from_millis(100)); // Simulate upload time
                let uploaded_file = format!("{} uploaded", processed_file);
        
                uploaded_files.push(uploaded_file);
            }
        
            uploaded_files
        }
        
        let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    
        // Sequential version
        let start = Instant::now();
        let uploaded_files_sequential = sequential(&files);
        let duration_sequential = start.elapsed();
        println!("Sequential duration: {:?}", duration_sequential);
        println!("Sequential uploaded files: {:?}", uploaded_files_sequential);
    
        // Parallel version
        let start = Instant::now();
    
        let (downloader_tx, processor_rx) = channel();
        let (processor_tx, uploader_rx) = channel();
    
        let downloader = Downloader { tx: downloader_tx };
        let processor = Processor { tx: processor_tx, rx: processor_rx };
        let uploader = Uploader { rx: uploader_rx };
    
        let files_clone = files.clone();
        let downloader_thread = thread::spawn(move || downloader.run(&files_clone));
        let processor_thread = thread::spawn(move || processor.run());
    
        let uploaded_files_parallel = uploader.run();
    
        downloader_thread.join().unwrap();
        processor_thread.join().unwrap();
    
        let duration_parallel = start.elapsed();
        println!("Parallel duration: {:?}", duration_parallel);
        println!("Parallel uploaded files: {:?}", uploaded_files_parallel);
    }

fn main() {
    data_paralelism_native();
    data_paralelism_rayon();
    task_paralelism_native();
    task_paralelism_rayon();
    pipeline_paralelism();
}
