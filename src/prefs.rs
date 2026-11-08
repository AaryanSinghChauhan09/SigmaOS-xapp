//! Preferences window definitions for xapp.

/// Preferences page
#[derive(Debug, Clone)]
pub struct PrefsPage {
    /// Page ID
    pub id: String,
    /// Page title
    pub title: String,
    /// Page icon
    pub icon: Option<String>,
    /// Page groups
    pub groups: Vec<PrefsGroup>,
    /// Page visible
    pub visible: bool,
    /// Created timestamp
    pub created_at: u64,
}

impl PrefsPage {
    /// Create new preferences page
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            icon: None,
            groups: vec![],
            visible: true,
            created_at: Self::now(),
        }
    }
    
    /// Set icon
    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }
    
    /// Add group
    pub fn add_group(&mut self, group: PrefsGroup) {
        self.groups.push(group);
    }
    
    /// Remove group
    pub fn remove_group(&mut self, index: usize) -> bool {
        if index < self.groups.len() {
            self.groups.remove(index);
            return true;
        }
        false
    }
    
    /// Get group
    pub fn get_group(&self, index: usize) -> Option<&PrefsGroup> {
        self.groups.get(index)
    }
    
    /// Get group mutably
    pub fn get_group_mut(&mut self, index: usize) -> Option<&mut PrefsGroup> {
        self.groups.get_mut(index)
    }
    
    /// Clear groups
    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }
    
    /// Set visible
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// Get group count
    pub fn get_group_count(&self) -> usize {
        self.groups.len()
    }
    
    /// Get page age in seconds
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

/// Preferences group
#[derive(Debug, Clone)]
pub struct PrefsGroup {
    /// Group ID
    pub id: String,
    /// Group title
    pub title: String,
    /// Group description
    pub description: Option<String>,
    /// Group rows
    pub rows: Vec<PrefsRow>,
    /// Created timestamp
    pub created_at: u64,
}

impl PrefsGroup {
    /// Create new preferences group
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
            rows: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    
    /// Add row
    pub fn add_row(&mut self, row: PrefsRow) {
        self.rows.push(row);
    }
    
    /// Remove row
    pub fn remove_row(&mut self, index: usize) -> bool {
        if index < self.rows.len() {
            self.rows.remove(index);
            return true;
        }
        false
    }
    
    /// Get row
    pub fn get_row(&self, index: usize) -> Option<&PrefsRow> {
        self.rows.get(index)
    }
    
    /// Get row mutably
    pub fn get_row_mut(&mut self, index: usize) -> Option<&mut PrefsRow> {
        self.rows.get_mut(index)
    }
    
    /// Clear rows
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }
    
    /// Get row count
    pub fn get_row_count(&self) -> usize {
        self.rows.len()
    }
    
    /// Get group age in seconds
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

/// Preferences row
#[derive(Debug, Clone)]
pub struct PrefsRow {
    /// Row ID
    pub id: String,
    /// Row title
    pub title: String,
    /// Row subtitle
    pub subtitle: Option<String>,
    /// Row icon
    pub icon: Option<String>,
    /// Row sensitive
    pub sensitive: bool,
    /// Created timestamp
    pub created_at: u64,
}

impl PrefsRow {
    /// Create new preferences row
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            subtitle: None,
            icon: None,
            sensitive: true,
            created_at: Self::now(),
        }
    }
    
    /// Set subtitle
    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
    }
    
    /// Set icon
    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }
    
    /// Set sensitive
    pub fn set_sensitive(&mut self, sensitive: bool) {
        self.sensitive = sensitive;
    }
    
    /// Get row age in seconds
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

/// Preferences window
#[derive(Debug, Clone)]
pub struct PrefsWindow {
    /// Window ID
    pub id: String,
    /// App ID
    pub app_id: String,
    /// Window title
    pub title: String,
    /// Window subtitle
    pub subtitle: Option<String>,
    /// Window pages
    pub pages: Vec<PrefsPage>,
    /// Current page index
    pub current_page: usize,
    /// Created timestamp
    pub created_at: u64,
    /// Last active timestamp
    pub last_active: u64,
}

impl PrefsWindow {
    /// Create new preferences window
    pub fn new(app_id: String) -> Self {
        Self {
            id: format!("prefs-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            app_id,
            title: String::new(),
            subtitle: None,
            pages: vec![],
            current_page: 0,
            created_at: Self::now(),
            last_active: Self::now(),
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.touch();
    }
    
    /// Set subtitle
    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
        self.touch();
    }
    
    /// Add page
    pub fn add_page(&mut self, page: PrefsPage) {
        self.pages.push(page);
    }
    
    /// Remove page
    pub fn remove_page(&mut self, index: usize) -> bool {
        if index < self.pages.len() {
            self.pages.remove(index);
            if self.current_page >= self.pages.len() {
                self.current_page = self.pages.len().saturating_sub(1);
            }
            return true;
        }
        false
    }
    
    /// Get page
    pub fn get_page(&self, index: usize) -> Option<&PrefsPage> {
        self.pages.get(index)
    }
    
    /// Get page mutably
    pub fn get_page_mut(&mut self, index: usize) -> Option<&mut PrefsPage> {
        self.pages.get_mut(index)
    }
    
    /// Get current page
    pub fn get_current_page(&self) -> Option<&PrefsPage> {
        self.pages.get(self.current_page)
    }
    
    /// Set current page
    pub fn set_current_page(&mut self, index: usize) -> bool {
        if index < self.pages.len() {
            self.current_page = index;
            self.touch();
            return true;
        }
        false
    }
    
    /// Next page
    pub fn next_page(&mut self) -> bool {
        if self.current_page + 1 < self.pages.len() {
            self.current_page += 1;
            self.touch();
            return true;
        }
        false
    }
    
    /// Previous page
    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.touch();
            return true;
        }
        false
    }
    
    /// Clear pages
    pub fn clear_pages(&mut self) {
        self.pages.clear();
        self.current_page = 0;
    }
    
    /// Touch window
    pub fn touch(&mut self) {
        self.last_active = Self::now();
    }
    
    /// Get page count
    pub fn get_page_count(&self) -> usize {
        self.pages.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
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
