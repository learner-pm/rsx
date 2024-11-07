use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::util::progress::ProgressSpinner;

pub fn search_files(
    query: &str,
    dir: &Path,
    file_count: &Arc<AtomicUsize>,
    loading: &ProgressSpinner,
) -> Vec<PathBuf> {
    let result = Arc::new(Mutex::new(Vec::new()));

    let entries: Vec<DirEntry> = if let Ok(entries) = fs::read_dir(dir) {
        entries.filter_map(|e| e.ok()).collect()
    } else {
        let x = result.lock().unwrap().clone();

        return x;
    };

    entries.into_par_iter().for_each(|entry| {
        let path = entry.path();
        if path.is_file()
            && path
                .file_name()
                .map_or(false, |name| name.to_string_lossy().contains(query))
        {
            let mut result = result.lock().unwrap();
            let count = file_count.fetch_add(1, Ordering::SeqCst);
            loading.update_message(format!("已查询到 {} 个文件", count));
            result.push(path);
            //sleep(Duration::from_millis(1000));
        } else if path.is_dir() {
            let mut result = result.lock().unwrap();
            let subdir_results = search_files(query, &path, &file_count, loading);
            result.extend(subdir_results);
        }
    });

    let r = result.lock().unwrap().clone();

    r
}
