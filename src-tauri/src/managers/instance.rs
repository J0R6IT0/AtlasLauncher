use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
};

pub struct InstanceManager {
    downloading: Mutex<Vec<Arc<DownloadTask>>>,
    downloading_counter: AtomicU32,
}

impl InstanceManager {
    pub async fn init() -> Self {
        Self {
            downloading: Mutex::new(vec![]),
            downloading_counter: AtomicU32::new(0),
        }
    }

    /// Adds an instance to the downloads queue
    pub async fn create_instance(&self, name: &str, version: &str) -> Result<(), &'static str> {
        let task = DownloadTask::new(name, version).await;

        match self.downloading.lock() {
            Ok(mut mutex_lock) => {
                mutex_lock.push(Arc::new(task));
                self.downloading_counter.fetch_add(1, Ordering::SeqCst)
            }
            Err(_) => return Err("Error adding download task"),
        };
        if self.downloading_counter.load(Ordering::SeqCst) == 1 {
            self.process_tasks().await?;
        }

        Ok(())
    }

    /// Start processing tasks
    async fn process_tasks(&self) -> Result<(), &'static str> {
        while self.downloading_counter.load(Ordering::SeqCst) > 0 {
            let task = self.downloading.lock().unwrap().first().unwrap().clone();
            task.download().await?;
            self.downloading.lock().unwrap().remove(0);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct DownloadTask {
    name: String,
    version: String,
}

impl DownloadTask {
    pub async fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    /// Starts the download
    pub async fn download(&self) -> Result<(), &'static str> {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        Ok(())
    }
}
