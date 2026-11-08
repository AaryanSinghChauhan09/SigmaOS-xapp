//! Settings definitions for xapp.

use std::collections::HashMap;

/// Setting kind
#[derive(Debug, Clone, PartialEq)]
pub enum SettingKind {
    /// Boolean setting
    Boolean,
    /// Integer setting
    Integer,
    /// String setting
    String,
    /// Enum setting
    Enum,
}

/// Setting value
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    /// Boolean value
    Boolean(bool),
    /// Integer value
    Integer(i64),
    /// String value
    String(String),
    /// Enum value
    Enum(String),
}

impl SettingValue {
    /// Get boolean value
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            SettingValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    /// Get integer value
    pub fn get_int(&self) -> Option<i64> {
        match self {
            SettingValue::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    /// Get string value
    pub fn get_string(&self) -> Option<&String> {
        match self {
            SettingValue::String(s) => Some(s),
            SettingValue::Enum(s) => Some(s),
            _ => None,
        }
    }
}

/// Setting
#[derive(Debug, Clone)]
pub struct Setting {
    /// Setting key
    pub key: String,
    /// Setting kind
    pub kind: SettingKind,
    /// Setting value
    pub value: SettingValue,
    /// Setting default value
    pub default: Option<SettingValue>,
    /// Setting description
    pub description: Option<String>,
    /// Created timestamp
    pub created_at: u64,
}

impl Setting {
    /// Create new setting
    pub fn new(key: String, kind: SettingKind, value: SettingValue) -> Self {
        Self {
            key,
            kind,
            value,
            default: None,
            description: None,
            created_at: Self::now(),
        }
    }
    
    /// Set default
    pub fn set_default(&mut self, default: SettingValue) {
        self.default = Some(default);
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    
    /// Reset to default
    pub fn reset(&mut self) -> bool {
        if let Some(default) = &self.default {
            self.value = default.clone();
            return true;
        }
        false
    }
    
    /// Check if modified
    pub fn is_modified(&self) -> bool {
        if let Some(default) = &self.default {
            self.value != *default
        } else {
            false
        }
    }
    
    /// Get setting age in seconds
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

/// Settings
#[derive(Debug, Clone)]
pub struct Settings {
    /// App ID
    pub app_id: String,
    /// Settings map
    pub settings: HashMap<String, Setting>,
    /// Created timestamp
    pub created_at: u64,
}

impl Settings {
    /// Create new settings
    pub fn new(app_id: String) -> Self {
        Self {
            app_id,
            settings: HashMap::new(),
            created_at: Self::now(),
        }
    }
    
    /// Set setting
    pub fn set(&mut self, key: String, value: SettingValue) {
        let kind = match value {
            SettingValue::Boolean(_) => SettingKind::Boolean,
            SettingValue::Integer(_) => SettingKind::Integer,
            SettingValue::String(_) => SettingKind::String,
            SettingValue::Enum(_) => SettingKind::Enum,
        };
        
        let setting = Setting::new(key.clone(), kind, value);
        self.settings.insert(key, setting);
    }
    
    /// Get setting
    pub fn get(&self, key: &str) -> Option<&SettingValue> {
        self.settings.get(key).map(|s| &s.value)
    }
    
    /// Get setting mutably
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Setting> {
        self.settings.get_mut(key)
    }
    
    /// Remove setting
    pub fn remove(&mut self, key: &str) -> bool {
        self.settings.remove(key).is_some()
    }
    
    /// Clear settings
    pub fn clear(&mut self) {
        self.settings.clear();
    }
    
    /// Get setting count
    pub fn get_count(&self) -> usize {
        self.settings.len()
    }
    
    /// Get modified settings
    pub fn get_modified(&self) -> Vec<&Setting> {
        self.settings.values()
            .filter(|s| s.is_modified())
            .collect()
    }
    
    /// Reset all settings
    pub fn reset_all(&mut self) -> usize {
        let mut reset = 0;
        for setting in self.settings.values_mut() {
            if setting.reset() {
                reset += 1;
            }
        }
        reset
    }
    
    /// Get settings age in seconds
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
