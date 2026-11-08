//! Application window definitions for xapp.

/// Window state
#[derive(Debug, Clone, PartialEq)]
pub enum WindowState {
    /// Normal window
    Normal,
    /// Maximized window
    Maximized,
    /// Fullscreen window
    Fullscreen,
    /// Minimized window
    Minimized,
}

/// Window kind
#[derive(Debug, Clone, PartialEq)]
pub enum WindowKind {
    /// Application window
    App,
    /// Dialog window
    Dialog,
    /// Preferences window
    Preferences,
}

/// Window config
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Width
    pub width: Option<u32>,
    /// Height
    pub height: Option<u32>,
    /// Min width
    pub min_width: Option<u32>,
    /// Min height
    pub min_height: Option<u32>,
    /// Max width
    pub max_width: Option<u32>,
    /// Max height
    pub max_height: Option<u32>,
    /// Resizable
    pub resizable: bool,
    /// Decorated
    pub decorated: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            resizable: true,
            decorated: true,
        }
    }
}

/// Application window
#[derive(Debug, Clone)]
pub struct AppWindow {
    /// Window ID
    pub id: String,
    /// App ID
    pub app_id: String,
    /// Window title
    pub title: Option<String>,
    /// Window state
    pub state: WindowState,
    /// Window kind
    pub kind: WindowKind,
    /// Window config
    pub config: WindowConfig,
    /// Width
    pub width: Option<u32>,
    /// Height
    pub height: Option<u32>,
    /// Created timestamp
    pub created_at: u64,
    /// Last active timestamp
    pub last_active: u64,
}

impl AppWindow {
    /// Create new app window
    pub fn new(app_id: String) -> Self {
        Self {
            id: format!("window-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            app_id,
            title: None,
            state: WindowState::Normal,
            kind: WindowKind::App,
            config: WindowConfig::default(),
            width: None,
            height: None,
            created_at: Self::now(),
            last_active: Self::now(),
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
        self.touch();
    }
    
    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = Some(width);
        self.height = Some(height);
        self.config.width = Some(width);
        self.config.height = Some(height);
        self.touch();
    }
    
    /// Maximize window
    pub fn maximize(&mut self) {
        self.state = WindowState::Maximized;
        self.touch();
    }
    
    /// Minimize window
    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
        self.touch();
    }
    
    /// Fullscreen window
    pub fn fullscreen(&mut self) {
        self.state = WindowState::Fullscreen;
        self.touch();
    }
    
    /// Restore window
    pub fn restore(&mut self) {
        self.state = WindowState::Normal;
        self.touch();
    }
    
    /// Set kind
    pub fn set_kind(&mut self, kind: WindowKind) {
        self.kind = kind;
    }
    
    /// Set config
    pub fn set_config(&mut self, config: WindowConfig) {
        self.config = config;
    }
    
    /// Touch window
    pub fn touch(&mut self) {
        self.last_active = Self::now();
    }
    
    /// Check if normal
    pub fn is_normal(&self) -> bool {
        self.state == WindowState::Normal
    }
    
    /// Check if maximized
    pub fn is_maximized(&self) -> bool {
        self.state == WindowState::Maximized
    }
    
    /// Check if fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.state == WindowState::Fullscreen
    }
    
    /// Check if minimized
    pub fn is_minimized(&self) -> bool {
        self.state == WindowState::Minimized
    }
    
    /// Get window age in seconds
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
