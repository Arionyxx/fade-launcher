use crate::app::AppInfo;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    fs,
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};
use walkdir::WalkDir;
use regex::Regex;

pub struct AppSearcher {
    indexed_apps: Arc<Mutex<Vec<AppInfo>>>,
    recent_apps: Arc<Mutex<Vec<AppInfo>>>,
    scan_paths: Vec<PathBuf>,
    file_extensions: Vec<String>,
    name_regex: Regex,
}

impl AppSearcher {
    pub fn new() -> Self {
        let scan_paths = Self::get_default_scan_paths();
        let file_extensions = vec![
            ".exe".to_string(),
            ".msi".to_string(),
            ".bat".to_string(),
            ".cmd".to_string(),
            ".com".to_string(),
            ".lnk".to_string(),
        ];

        Self {
            indexed_apps: Arc::new(Mutex::new(Vec::new())),
            recent_apps: Arc::new(Mutex::new(Vec::new())),
            scan_paths,
            file_extensions,
            name_regex: Regex::new(r"[^\w\s-_.]").unwrap(),
        }
    }
    
    fn get_default_scan_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // Common Windows application directories
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            paths.push(PathBuf::from(program_files));
        }
        
        if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
            paths.push(PathBuf::from(program_files_x86));
        }
        
        // Windows Apps directory
        if let Some(local_app_data) = dirs::data_local_dir() {
            paths.push(local_app_data.join("Microsoft").join("WindowsApps"));
        }
        
        // Start Menu shortcuts
        if let Some(start_menu) = dirs::data_dir() {
            paths.push(start_menu.join("Microsoft").join("Windows").join("Start Menu").join("Programs"));
        }
        
        // Common shortcuts location
        if let Some(roaming) = dirs::data_dir() {
            paths.push(roaming.join("Microsoft").join("Windows").join("Start Menu").join("Programs"));
        }
        
        // User's desktop
        if let Some(desktop) = dirs::desktop_dir() {
            paths.push(desktop);
        }
        
        // System PATH directories
        if let Ok(path_env) = std::env::var("PATH") {
            for path_str in path_env.split(';') {
                if !path_str.is_empty() {
                    paths.push(PathBuf::from(path_str));
                }
            }
        }
        
        paths
    }
    
    pub fn start_initial_scan(&mut self) {
        let indexed_apps = Arc::clone(&self.indexed_apps);
        let scan_paths = self.scan_paths.clone();
        let file_extensions = self.file_extensions.clone();
        
        // Start background scanning thread
        thread::spawn(move || {
            let apps = Self::scan_for_applications(&scan_paths, &file_extensions);
            
            if let Ok(mut indexed) = indexed_apps.lock() {
                *indexed = apps;
                println!("App indexing completed. Found {} applications", indexed.len());
            }
        });
    }
    
    fn scan_for_applications(scan_paths: &[PathBuf], file_extensions: &[String]) -> Vec<AppInfo> {
        let mut apps = Vec::new();
        let mut seen_names = HashMap::new();
        
        for base_path in scan_paths {
            if !base_path.exists() {
                continue;
            }
            
            println!("Scanning: {:?}", base_path);
            
            let walker = WalkDir::new(base_path)
                .max_depth(3) // Limit depth to avoid too much scanning
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok());
                
            for entry in walker {
                let path = entry.path();
                
                if !path.is_file() {
                    continue;
                }
                
                if let Some(extension) = path.extension() {
                    let ext = format!(".{}", extension.to_string_lossy().to_lowercase());
                    
                    if file_extensions.contains(&ext) {
                        if let Some(app_info) = Self::create_app_info(path) {
                            // Avoid duplicates based on name
                            let key = app_info.name.to_lowercase();
                            if !seen_names.contains_key(&key) {
                                seen_names.insert(key, true);
                                apps.push(app_info);
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by name for better organization
        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        
        apps
    }
    
    fn create_app_info(path: &Path) -> Option<AppInfo> {
        let file_name = path.file_stem()?.to_string_lossy();
        
        // Skip common system files and uninteresting executables
        let skip_patterns = [
            "unins", "uninst", "setup", "install", "update", "crash", "error",
            "helper", "service", "daemon", "background", "launcher"
        ];
        
        let name_lower = file_name.to_lowercase();
        if skip_patterns.iter().any(|pattern| name_lower.contains(pattern)) {
            return None;
        }
        
        // Clean up the display name
        let clean_name = Self::clean_app_name(&file_name);
        
        // Try to get file description or version info on Windows
        let description = Self::get_file_description(path);
        
        Some(AppInfo {
            name: clean_name,
            path: path.to_string_lossy().to_string(),
            icon_path: None, // TODO: Extract icon
            description,
            score: 0.0,
        })
    }
    
    fn clean_app_name(raw_name: &str) -> String {
        // Remove common suffixes and clean up name
        let cleaned = raw_name
            .replace("_", " ")
            .replace("-", " ")
            .replace(".", " ");
            
        // Capitalize first letter of each word
        cleaned
            .split_whitespace()
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                if !chars.is_empty() {
                    chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    #[cfg(windows)]
    fn get_file_description(path: &Path) -> Option<String> {
        // On Windows, we could use Windows API to get file version info
        // For now, return None, but this could be enhanced
        None
    }
    
    #[cfg(not(windows))]
    fn get_file_description(_path: &Path) -> Option<String> {
        None
    }
    
    pub fn search(&self, query: &str, limit: usize) -> Vec<AppInfo> {
        if query.is_empty() {
            return self.get_recent_apps(limit);
        }
        
        let indexed_apps = self.indexed_apps.lock().ok()?;
        let query_lower = query.to_lowercase();
        
        let mut scored_results: Vec<(AppInfo, f32)> = indexed_apps
            .iter()
            .filter_map(|app| {
                let score = Self::calculate_relevance_score(&app.name, &app.path, &query_lower);
                if score > 0.0 {
                    Some((app.clone(), score))
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by score (highest first)
        scored_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Update scores and return results
        scored_results
            .into_iter()
            .take(limit)
            .map(|(mut app, score)| {
                app.score = score;
                app
            })
            .collect()
    }
    
    fn calculate_relevance_score(name: &str, path: &str, query: &str) -> f32 {
        let name_lower = name.to_lowercase();
        let path_lower = path.to_lowercase();
        let mut score = 0.0;
        
        // Exact match gets highest score
        if name_lower == query {
            score += 100.0;
        }
        
        // Starts with query gets high score
        if name_lower.starts_with(query) {
            score += 50.0;
        }
        
        // Contains query gets medium score
        if name_lower.contains(query) {
            score += 25.0;
        }
        
        // Path contains query gets lower score
        if path_lower.contains(query) {
            score += 10.0;
        }
        
        // Bonus for shorter names (more specific matches)
        if score > 0.0 {
            score += 20.0 / (name.len() as f32 + 1.0);
        }
        
        score
    }
    
    pub fn get_recent_apps(&self, limit: usize) -> Vec<AppInfo> {
        if let Ok(recent) = self.recent_apps.lock() {
            recent.iter().take(limit).cloned().collect()
        } else {
            // Fallback to some popular apps from index
            if let Ok(indexed) = self.indexed_apps.lock() {
                indexed.iter().take(limit).cloned().collect()
            } else {
                Vec::new()
            }
        }
    }
    
    pub fn add_to_recent(&self, app: AppInfo) {
        if let Ok(mut recent) = self.recent_apps.lock() {
            // Remove if already exists
            recent.retain(|a| a.path != app.path);
            
            // Add to front
            recent.insert(0, app);
            
            // Keep only last 20
            recent.truncate(20);
        }
    }
    
    pub fn launch_app(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Launching: {}", path);
        
        let path_buf = PathBuf::from(path);
        
        if path_buf.extension().and_then(|s| s.to_str()) == Some("lnk") {
            // For shortcuts, use Windows shell to open
            Command::new("cmd")
                .args(&["/C", "start", "", path])
                .spawn()?;
        } else {
            // For executables, launch directly
            Command::new(path)
                .spawn()?;
        }
        
        Ok(())
    }
}