//! xapp manager.

use std::collections::HashMap;
use crate::app::{XApp, AppState, AppConfig};
use crate::window::{AppWindow, WindowKind};
use crate::prefs::PrefsWindow;
use crate::settings::{Settings, SettingValue};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max apps
    pub max_apps: usize,
    /// Max windows per app
    pub max_windows_per_app: usize,
    /// Enable preferences windows
    pub enable_prefs: bool,
    /// Enable settings persistence
    pub enable_settings_persistence: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_apps: 100,
            max_windows_per_app: 10,
            enable_prefs: true,
            enable_settings_persistence: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total apps
    pub total_apps: usize,
    /// Running apps
    pub running_apps: usize,
    /// Suspended apps
    pub suspended_apps: usize,
    /// Total windows
    pub total_windows: usize,
    /// Total preferences windows
    pub total_prefs_windows: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// xapp manager
#[derive(Debug, Clone)]
pub struct XappManager {
    /// Apps
    apps: HashMap<String, XApp>,
    /// Windows
    windows: HashMap<String, AppWindow>,
    /// Preferences windows
    prefs_windows: HashMap<String, PrefsWindow>,
    /// Settings per app
    settings: HashMap<String, Settings>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for XappManager {
    fn default() -> Self {
        Self {
            apps: HashMap::new(),
            windows: HashMap::new(),
            prefs_windows: HashMap::new(),
            settings: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_apps: 0,
                running_apps: 0,
                suspended_apps: 0,
                total_windows: 0,
                total_prefs_windows: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl XappManager {
    /// Create new xapp manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            apps: HashMap::new(),
            windows: HashMap::new(),
            prefs_windows: HashMap::new(),
            settings: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_apps: 0,
                running_apps: 0,
                suspended_apps: 0,
                total_windows: 0,
                total_prefs_windows: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // App methods
    
    /// Register app
    pub fn register_app(&mut self, app_id: String) -> String {
        let app = XApp::new(app_id.clone());
        let id = app.id.clone();
        
        // Initialize settings for app
        let mut settings = Settings::new(app_id);
        settings.set("theme".into(), SettingValue::String("light".into()));
        settings.set("fontsize".into(), SettingValue::Integer(12));
        
        self.apps.insert(id.clone(), app);
        self.settings.insert(id.clone(), settings);
        self.update_statistics();
        id
    }
    
    /// Get app
    pub fn get_app(&self, app_id: &str) -> Option<&XApp> {
        self.apps.get(app_id)
    }
    
    /// Get app mutably
    pub fn get_app_mut(&mut self, app_id: &str) -> Option<&mut XApp> {
        self.apps.get_mut(app_id)
    }
    
    /// Get all apps
    pub fn list_apps(&self) -> Vec<&XApp> {
        self.apps.values().collect()
    }
    
    /// Remove app
    pub fn remove_app(&mut self, app_id: &str) -> bool {
        self.apps.remove(app_id);
        self.settings.remove(app_id);
        self.update_statistics();
        true
    }
    
    /// Set app config
    pub fn set_app_config(&mut self, app_id: &str, config: AppConfig) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.set_config(config);
            return true;
        }
        false
    }
    
    /// Set app name
    pub fn set_app_name(&mut self, app_id: &str, name: String) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.set_name(name);
            return true;
        }
        false
    }
    
    /// Set app version
    pub fn set_app_version(&mut self, app_id: &str, version: String) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.set_version(version);
            return true;
        }
        false
    }
    
    /// Set app description
    pub fn set_app_description(&mut self, app_id: &str, description: String) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.set_description(description);
            return true;
        }
        false
    }
    
    /// Set app icon
    pub fn set_app_icon(&mut self, app_id: &str, icon: String) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.set_icon(icon);
            return true;
        }
        false
    }
    
    /// Add app category
    pub fn add_app_category(&mut self, app_id: &str, category: String) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.add_category(category);
            return true;
        }
        false
    }
    
    /// Start app
    pub fn start_app(&mut self, app_id: &str) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.start();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Suspend app
    pub fn suspend_app(&mut self, app_id: &str) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.suspend();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Resume app
    pub fn resume_app(&mut self, app_id: &str) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.resume();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop app
    pub fn stop_app(&mut self, app_id: &str) -> bool {
        if let Some(app) = self.apps.get_mut(app_id) {
            app.stop();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get apps by state
    pub fn get_apps_by_state(&self, state: &AppState) -> Vec<&XApp> {
        self.apps.values()
            .filter(|a| a.state == *state)
            .collect()
    }
    
    /// Get running apps
    pub fn get_running_apps(&self) -> Vec<&XApp> {
        self.get_apps_by_state(&AppState::Running)
    }
    
    /// Get suspended apps
    pub fn get_suspended_apps(&self) -> Vec<&XApp> {
        self.get_apps_by_state(&AppState::Suspended)
    }
    
    /// Search apps
    pub fn search_apps(&self, query: &str) -> Vec<&XApp> {
        let query_lower = query.to_lowercase();
        self.apps.values()
            .filter(|a| a.app_id.to_lowercase().contains(&query_lower) ||
                      a.config.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Window methods
    
    /// Create window
    pub fn create_window(&mut self, app_id: &str, kind: WindowKind) -> String {
        let window = AppWindow::new(app_id.to_string());
        let window_id = window.id.clone();
        
        let mut window = window;
        window.set_kind(kind);
        
        // Add to app
        if let Some(app) = self.apps.get_mut(app_id) {
            app.windows.push(window.clone());
            app.info.window_count = app.windows.len();
        }
        
        self.windows.insert(window_id.clone(), window);
        self.update_statistics();
        window_id
    }
    
    /// Get window
    pub fn get_window(&self, window_id: &str) -> Option<&AppWindow> {
        self.windows.get(window_id)
    }
    
    /// Get window mutably
    pub fn get_window_mut(&mut self, window_id: &str) -> Option<&mut AppWindow> {
        self.windows.get_mut(window_id)
    }
    
    /// Get all windows
    pub fn list_windows(&self) -> Vec<&AppWindow> {
        self.windows.values().collect()
    }
    
    /// Remove window
    pub fn remove_window(&mut self, window_id: &str) -> bool {
        self.windows.remove(window_id);
        // Also remove from app
        for app in self.apps.values_mut() {
            app.windows.retain(|w| w.id != window_id);
            app.info.window_count = app.windows.len();
        }
        self.update_statistics();
        true
    }
    
    /// Get windows by app
    pub fn get_windows_by_app(&self, app_id: &str) -> Vec<&AppWindow> {
        self.windows.values()
            .filter(|w| w.app_id == app_id)
            .collect()
    }
    
    /// Get windows by kind
    pub fn get_windows_by_kind(&self, kind: &WindowKind) -> Vec<&AppWindow> {
        self.windows.values()
            .filter(|w| w.kind == *kind)
            .collect()
    }
    
    /// Set window title
    pub fn set_window_title(&mut self, window_id: &str, title: String) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.set_title(title);
            return true;
        }
        false
    }
    
    /// Set window size
    pub fn set_window_size(&mut self, window_id: &str, width: u32, height: u32) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.set_size(width, height);
            return true;
        }
        false
    }
    
    /// Maximize window
    pub fn maximize_window(&mut self, window_id: &str) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.maximize();
            return true;
        }
        false
    }
    
    /// Minimize window
    pub fn minimize_window(&mut self, window_id: &str) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.minimize();
            return true;
        }
        false
    }
    
    /// Fullscreen window
    pub fn fullscreen_window(&mut self, window_id: &str) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.fullscreen();
            return true;
        }
        false
    }
    
    /// Restore window
    pub fn restore_window(&mut self, window_id: &str) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.restore();
            return true;
        }
        false
    }
    
    // Preferences methods
    
    /// Create preferences window
    pub fn create_prefs(&mut self, app_id: &str) -> String {
        if !self.options.enable_prefs {
            return String::new();
        }
        
        let prefs = PrefsWindow::new(app_id.to_string());
        let prefs_id = prefs.id.clone();
        
        // Add to app
        if let Some(app) = self.apps.get_mut(app_id) {
            app.create_prefs();
        }
        
        self.prefs_windows.insert(prefs_id.clone(), prefs);
        self.update_statistics();
        prefs_id
    }
    
    /// Get preferences window
    pub fn get_prefs(&self, prefs_id: &str) -> Option<&PrefsWindow> {
        self.prefs_windows.get(prefs_id)
    }
    
    /// Get preferences window mutably
    pub fn get_prefs_mut(&mut self, prefs_id: &str) -> Option<&mut PrefsWindow> {
        self.prefs_windows.get_mut(prefs_id)
    }
    
    /// Get all preferences windows
    pub fn list_prefs(&self) -> Vec<&PrefsWindow> {
        self.prefs_windows.values().collect()
    }
    
    /// Remove preferences window
    pub fn remove_prefs(&mut self, prefs_id: &str) -> bool {
        self.prefs_windows.remove(prefs_id);
        self.update_statistics();
        true
    }
    
    /// Get preferences by app
    pub fn get_prefs_by_app(&self, app_id: &str) -> Vec<&PrefsWindow> {
        self.prefs_windows.values()
            .filter(|p| p.app_id == app_id)
            .collect()
    }
    
    // Settings methods
    
    /// Get settings for app
    pub fn get_settings(&self, app_id: &str) -> Option<&Settings> {
        self.settings.get(app_id)
    }
    
    /// Get settings mutably
    pub fn get_settings_mut(&mut self, app_id: &str) -> Option<&mut Settings> {
        self.settings.get_mut(app_id)
    }
    
    /// Set setting for app
    pub fn set_setting(&mut self, app_id: &str, key: String, value: SettingValue) -> bool {
        if let Some(settings) = self.settings.get_mut(app_id) {
            settings.set(key, value);
            return true;
        }
        false
    }
    
    /// Get setting for app
    pub fn get_setting(&self, app_id: &str, key: &str) -> Option<&SettingValue> {
        self.settings.get(app_id)
            .and_then(|s| s.get(key))
    }
    
    /// Remove setting for app
    pub fn remove_setting(&mut self, app_id: &str, key: &str) -> bool {
        if let Some(settings) = self.settings.get_mut(app_id) {
            settings.remove(key);
            return true;
        }
        false
    }
    
    /// Clear settings for app
    pub fn clear_settings(&mut self, app_id: &str) -> bool {
        if let Some(settings) = self.settings.get_mut(app_id) {
            settings.clear();
            return true;
        }
        false
    }
    
    /// Reset settings for app
    pub fn reset_settings(&mut self, app_id: &str) -> usize {
        if let Some(settings) = self.settings.get_mut(app_id) {
            settings.reset_all()
        } else {
            0
        }
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_apps = self.apps.len();
        self.statistics.running_apps = self.apps.values()
            .filter(|a| a.is_running())
            .count();
        self.statistics.suspended_apps = self.apps.values()
            .filter(|a| a.is_suspended())
            .count();
        self.statistics.total_windows = self.windows.len();
        self.statistics.total_prefs_windows = self.prefs_windows.len();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            apps: self.apps.values().cloned().collect(),
            windows: self.windows.values().cloned().collect(),
            prefs_windows: self.prefs_windows.values().cloned().collect(),
            settings: self.settings.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.apps.clear();
        for app in export.apps {
            self.apps.insert(app.id.clone(), app);
        }
        
        self.windows.clear();
        for window in export.windows {
            self.windows.insert(window.id.clone(), window);
        }
        
        self.prefs_windows.clear();
        for prefs in export.prefs_windows {
            self.prefs_windows.insert(prefs.id.clone(), prefs);
        }
        
        self.settings.clear();
        for settings in export.settings {
            self.settings.insert(settings.app_id.clone(), settings);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Apps
    pub apps: Vec<XApp>,
    /// Windows
    pub windows: Vec<AppWindow>,
    /// Preferences windows
    pub prefs_windows: Vec<PrefsWindow>,
    /// Settings
    pub settings: Vec<Settings>,
}
