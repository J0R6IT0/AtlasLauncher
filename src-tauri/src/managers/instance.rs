use std::sync::{Arc, Mutex};

pub struct InstanceManager {
    downloading: Mutex<Vec<Arc<DownloadTask>>>,
}

impl InstanceManager {
    pub async fn init() -> Self {
        Self {
            downloading: Mutex::new(vec![]),
        }
    }

    /// Adds an instance to the downloads queue
    pub async fn create_instance(&self, name: &str, version: &str) -> Result<(), &'static str> {
        let task = DownloadTask::new(name, version).await;

        let tasks_number = match self.downloading.lock() {
            Ok(mut mutex_lock) => {
                mutex_lock.push(Arc::new(task));
                mutex_lock.len()
            }
            Err(_) => return Err("Error adding download task"),
        };
        if tasks_number == 1 {
            self.process_tasks().await?;
        }

        Ok(())
    }

    /// Start processing tasks
    async fn process_tasks(&self) -> Result<(), &'static str> {
        while self.download_tasks_count()? > 0 {
            let task = self.downloading.lock().unwrap().first().unwrap().clone();
            task.download().await?;
            self.downloading.lock().unwrap().remove(0);
        }
        Ok(())
    }

    /// Returns the number of active download tasks
    fn download_tasks_count(&self) -> Result<u32, &'static str> {
        match self.downloading.lock() {
            Ok(mutex_lock) => Ok(mutex_lock.len() as u32),

            Err(_) => Err("Error adding download task"),
        }
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
