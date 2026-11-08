//! XApp application definitions for xapp.

use crate::settings::Settings;
use crate::window::{AppWindow, WindowKind};
use crate::prefs::PrefsWindow;

/// App state
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Starting app
    Starting,
    /// Running app
    Running,
    /// Suspended app
    Suspended,
    /// Stopped app
    Stopped,
}

/// App config
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// App name
    pub name: String,
    /// App version
    pub version: String,
    /// App description
    pub description: Option<String>,
    /// App author
    pub author: Option<String>,
    /// App license
    pub license: Option<String>,
    /// App homepage
    pub homepage: Option<String>,
    /// App icon
    pub icon: Option<String>,
    /// App categories
    pub categories: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "1.0.0".into(),
            description: None,
            author: None,
            license: None,
            homepage: None,
            icon: None,
            categories: vec![],
        }
    }
}

/// App info
#[derive(Debug, Clone)]
pub struct AppInfo {
    /// Process ID
    pub pid: Option<u32>,
    /// Window count
    pub window_count: usize,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// CPU usage percentage
    pub cpu_percent: f32,
    /// Created timestamp
    pub created_at: u64,
}

/// XApp application
#[derive(Debug, Clone)]
pub struct XApp {
    /// App ID
    pub id: String,
    /// App ID (reverse domain)
    pub app_id: String,
    /// App state
    pub state: AppState,
    /// App config
    pub config: AppConfig,
    /// App info
    pub info: AppInfo,
    /// App settings
    pub settings: Settings,
    /// App windows
    pub windows: Vec<AppWindow>,
    /// App preferences window
    pub prefs_window: Option<PrefsWindow>,
    /// Created timestamp
    pub created_at: u64,
    /// Last active timestamp
    pub last_active: u64,
}

impl XApp {
    /// Create new XApp
    pub fn new(app_id: String) -> Self {
        Self {
            id: format!("app-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            app_id,
            state: AppState::Starting,
            config: AppConfig::default(),
            info: AppInfo {
                pid: None,
                window_count: 0,
                memory_bytes: 0,
                cpu_percent: 0.0,
                created_at: Self::now(),
            },
            settings: Settings::new(String::new()),
            windows: vec![],
            prefs_window: None,
            created_at: Self::now(),
            last_active: Self::now(),
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: AppConfig) {
        self.config = config;
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.config.name = name;
    }
    
    /// Set version
    pub fn set_version(&mut self, version: String) {
        self.config.version = version;
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.config.description = Some(description);
    }
    
    /// Set icon
    pub fn set_icon(&mut self, icon: String) {
        self.config.icon = Some(icon);
    }
    
    /// Add category
    pub fn add_category(&mut self, category: String) {
        if !self.config.categories.contains(&category) {
            self.config.categories.push(category);
        }
    }
    
    /// Start app
    pub fn start(&mut self) {
        self.state = AppState::Running;
        self.info.pid = Some(std::process::id());
        self.touch();
    }
    
    /// Suspend app
    pub fn suspend(&mut self) {
        self.state = AppState::Suspended;
        self.touch();
    }
    
    /// Resume app
    pub fn resume(&mut self) {
        self.state = AppState::Running;
        self.touch();
    }
    
    /// Stop app
    pub fn stop(&mut self) {
        self.state = AppState::Stopped;
        self.touch();
    }
    
    /// Create window
    pub fn create_window(&mut self, kind: WindowKind) -> AppWindow {
        let mut window = AppWindow::new(self.app_id.clone());
        window.set_kind(kind);
        self.windows.push(window.clone());
        self.info.window_count = self.windows.len();
        self.touch();
        window
    }
    
    /// Remove window
    pub fn remove_window(&mut self, window_id: &str) -> bool {
        if let Some(pos) = self.windows.iter().position(|w| w.id == window_id) {
            self.windows.remove(pos);
            self.info.window_count = self.windows.len();
            self.touch();
            return true;
        }
        false
    }
    
    /// Get window
    pub fn get_window(&self, window_id: &str) -> Option<&AppWindow> {
        self.windows.iter().find(|w| w.id == window_id)
    }
    
    /// Get window mutably
    pub fn get_window_mut(&mut self, window_id: &str) -> Option<&mut AppWindow> {
        self.windows.iter_mut().find(|w| w.id == window_id)
    }
    
    /// Create preferences window
    pub fn create_prefs(&mut self) -> PrefsWindow {
        let prefs = PrefsWindow::new(self.app_id.clone());
        self.prefs_window = Some(prefs.clone());
        self.touch();
        prefs
    }
    
    /// Set preferences window
    pub fn set_prefs_window(&mut self, prefs: PrefsWindow) {
        self.prefs_window = Some(prefs);
    }
    
    /// Get preferences window
    pub fn get_prefs_window(&self) -> Option<&PrefsWindow> {
        self.prefs_window.as_ref()
    }
    
    /// Get preferences window mutably
    pub fn get_prefs_window_mut(&mut self) -> Option<&mut PrefsWindow> {
        self.prefs_window.as_mut()
    }
    
    /// Update info
    pub fn update_info(&mut self, memory_bytes: u64, cpu_percent: f32) {
        self.info.memory_bytes = memory_bytes;
        self.info.cpu_percent = cpu_percent;
        self.touch();
    }
    
    /// Touch app
    pub fn touch(&mut self) {
        self.last_active = Self::now();
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == AppState::Running
    }
    
    /// Check if suspended
    pub fn is_suspended(&self) -> bool {
        self.state == AppState::Suspended
    }
    
    /// Check if stopped
    pub fn is_stopped(&self) -> bool {
        self.state == AppState::Stopped
    }
    
    /// Get app age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get idle seconds
    pub fn get_idle_seconds(&self) -> u64 {
        Self::now() - self.last_active
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
