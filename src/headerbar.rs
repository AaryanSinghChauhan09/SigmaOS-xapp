//! Header bar definitions for xapp.

/// Header bar style
#[derive(Debug, Clone, PartialEq)]
pub enum HeaderBarStyle {
    /// Default style
    Default,
    /// Flat style
    Flat,
    /// Raised style
    Raised,
}

/// Header bar button
#[derive(Debug, Clone)]
pub struct HeaderBarButton {
    /// Button ID
    pub id: String,
    /// Button icon
    pub icon: String,
    /// Button label
    pub label: String,
    /// Button tooltip
    pub tooltip: Option<String>,
    /// Button sensitive
    pub sensitive: bool,
    /// Button visible
    pub visible: bool,
    /// Created timestamp
    pub created_at: u64,
}

impl HeaderBarButton {
    /// Create new header bar button
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            icon: String::new(),
            label,
            tooltip: None,
            sensitive: true,
            visible: true,
            created_at: Self::now(),
        }
    }
    
    /// Set icon
    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon;
    }
    
    /// Set tooltip
    pub fn set_tooltip(&mut self, tooltip: String) {
        self.tooltip = Some(tooltip);
    }
    
    /// Set sensitive
    pub fn set_sensitive(&mut self, sensitive: bool) {
        self.sensitive = sensitive;
    }
    
    /// Set visible
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// Check if sensitive
    pub fn is_sensitive(&self) -> bool {
        self.sensitive
    }
    
    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Get button age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Header bar
#[derive(Debug, Clone)]
pub struct HeaderBar {
    /// Header bar ID
    pub id: String,
    /// Header bar title
    pub title: Option<String>,
    /// Header bar subtitle
    pub subtitle: Option<String>,
    /// Header bar style
    pub style: HeaderBarStyle,
    /// Header bar buttons
    pub buttons: Vec<HeaderBarButton>,
    /// Header bar show close button
    pub show_close_button: bool,
    /// Header bar show menu button
    pub show_menu_button: bool,
    /// Created timestamp
    pub created_at: u64,
}

impl HeaderBar {
    /// Create new header bar
    pub fn new() -> Self {
        Self {
            id: format!("headerbar-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            title: None,
            subtitle: None,
            style: HeaderBarStyle::Default,
            buttons: vec![],
            show_close_button: true,
            show_menu_button: false,
            created_at: Self::now(),
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
    
    /// Set subtitle
    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
    }
    
    /// Set style
    pub fn set_style(&mut self, style: HeaderBarStyle) {
        self.style = style;
    }
    
    /// Add button
    pub fn add_button(&mut self, button: HeaderBarButton) {
        self.buttons.push(button);
    }
    
    /// Remove button
    pub fn remove_button(&mut self, index: usize) -> bool {
        if index < self.buttons.len() {
            self.buttons.remove(index);
            return true;
        }
        false
    }
    
    /// Get button
    pub fn get_button(&self, index: usize) -> Option<&HeaderBarButton> {
        self.buttons.get(index)
    }
    
    /// Get button mutably
    pub fn get_button_mut(&mut self, index: usize) -> Option<&mut HeaderBarButton> {
        self.buttons.get_mut(index)
    }
    
    /// Clear buttons
    pub fn clear_buttons(&mut self) {
        self.buttons.clear();
    }
    
    /// Set show close button
    pub fn set_show_close_button(&mut self, show: bool) {
        self.show_close_button = show;
    }
    
    /// Set show menu button
    pub fn set_show_menu_button(&mut self, show: bool) {
        self.show_menu_button = show;
    }
    
    /// Get button count
    pub fn get_button_count(&self) -> usize {
        self.buttons.len()
    }
    
    /// Get visible buttons
    pub fn get_visible_buttons(&self) -> Vec<&HeaderBarButton> {
        self.buttons.iter().filter(|b| b.is_visible()).collect()
    }
    
    /// Get header bar age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for HeaderBar {
    fn default() -> Self {
        Self::new()
    }
}
