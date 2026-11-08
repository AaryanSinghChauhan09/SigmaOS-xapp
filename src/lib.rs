//! SigmaOS-xapp — XApp application framework component.
//!
//! Models the XApp library used by elementary OS apps:
//! application windows, header bars, preferences, settings,
//! and app lifecycle for the SigmaOS desktop.

pub mod window;
pub mod headerbar;
pub mod prefs;
pub mod settings;
pub mod app;
pub mod manager;
pub mod options;

pub use window::{AppWindow, WindowState, WindowConfig, WindowKind};
pub use headerbar::{HeaderBar, HeaderBarButton, HeaderBarStyle};
pub use prefs::{PrefsWindow, PrefsPage, PrefsRow, PrefsGroup};
pub use settings::{Settings, Setting, SettingKind, SettingValue};
pub use app::{XApp, AppState, AppConfig, AppInfo};
pub use manager::{XappManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_window_state() {
        let states = vec![WindowState::Normal, WindowState::Maximized, WindowState::Fullscreen, WindowState::Minimized];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_window_kind() {
        let kinds = vec![WindowKind::App, WindowKind::Dialog, WindowKind::Preferences];
        assert_eq!(kinds.len(), 3);
    }
    
    #[test]
    fn test_header_bar_style() {
        let styles = vec![HeaderBarStyle::Default, HeaderBarStyle::Flat, HeaderBarStyle::Raised];
        assert_eq!(styles.len(), 3);
    }
    
    #[test]
    fn test_setting_kind() {
        let kinds = vec![
            SettingKind::Boolean,
            SettingKind::Integer,
            SettingKind::String,
            SettingKind::Enum,
        ];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_app_state() {
        let states = vec![AppState::Starting, AppState::Running, AppState::Suspended, AppState::Stopped];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_app_window_creation() {
        let window = AppWindow::new("org.sigmaos.App".into());
        assert_eq!(window.app_id, "org.sigmaos.App".to_string());
        assert_eq!(window.state, WindowState::Normal);
        assert_eq!(window.kind, WindowKind::App);
    }
    
    #[test]
    fn test_app_window_title() {
        let mut window = AppWindow::new("app".into());
        window.set_title("My App".into());
        assert_eq!(window.title, Some("My App".to_string()));
    }
    
    #[test]
    fn test_app_window_size() {
        let mut window = AppWindow::new("app".into());
        window.set_size(800, 600);
        assert_eq!(window.width, Some(800));
        assert_eq!(window.height, Some(600));
    }
    
    #[test]
    fn test_app_window_lifecycle() {
        let mut window = AppWindow::new("app".into());
        window.maximize();
        assert_eq!(window.state, WindowState::Maximized);
        window.minimize();
        assert_eq!(window.state, WindowState::Minimized);
        window.restore();
        assert_eq!(window.state, WindowState::Normal);
    }
    
    #[test]
    fn test_header_bar_creation() {
        let bar = HeaderBar::new();
        assert_eq!(bar.style, HeaderBarStyle::Default);
        assert!(bar.buttons.is_empty());
    }
    
    #[test]
    fn test_header_bar_buttons() {
        let mut bar = HeaderBar::new();
        bar.add_button(HeaderBarButton::new("menu".into(), "Menu".into()));
        bar.add_button(HeaderBarButton::new("settings".into(), "Settings".into()));
        
        assert_eq!(bar.buttons.len(), 2);
        assert_eq!(bar.buttons[0].id, "menu".to_string());
    }
    
    #[test]
    fn test_header_bar_title() {
        let mut bar = HeaderBar::new();
        bar.set_title("Window Title".into());
        assert_eq!(bar.title, Some("Window Title".to_string()));
        
        bar.set_subtitle("Subtitle".into());
        assert_eq!(bar.subtitle, Some("Subtitle".to_string()));
    }
    
    #[test]
    fn test_setting_creation() {
        let setting = Setting::new("theme".into(), SettingKind::String, SettingValue::String("dark".into()));
        assert_eq!(setting.key, "theme".to_string());
        assert_eq!(setting.kind, SettingKind::String);
    }
    
    #[test]
    fn test_setting_values() {
        let bool_setting = Setting::new("enabled".into(), SettingKind::Boolean, SettingValue::Boolean(true));
        assert_eq!(bool_setting.value.get_bool(), Some(true));
        
        let int_setting = Setting::new("count".into(), SettingKind::Integer, SettingValue::Integer(42));
        assert_eq!(int_setting.value.get_int(), Some(42));
    }
    
    #[test]
    fn test_prefs_window() {
        let mut prefs = PrefsWindow::new("org.sigmaos.App".into());
        assert_eq!(prefs.app_id, "org.sigmaos.App".to_string());
        assert!(prefs.pages.is_empty());
    }
    
    #[test]
    fn test_prefs_pages() {
        let mut prefs = PrefsWindow::new("app".into());
        prefs.add_page(PrefsPage::new("general".into(), "General".into()));
        prefs.add_page(PrefsPage::new("advanced".into(), "Advanced".into()));
        
        assert_eq!(prefs.pages.len(), 2);
    }
    
    #[test]
    fn test_prefs_groups() {
        let mut page = PrefsPage::new("general".into(), "General".into());
        page.add_group(PrefsGroup::new("appearance".into(), "Appearance".into()));
        page.add_group(PrefsGroup::new("behavior".into(), "Behavior".into()));
        
        assert_eq!(page.groups.len(), 2);
    }
    
    #[test]
    fn test_prefs_rows() {
        let mut group = PrefsGroup::new("appearance".into(), "Appearance".into());
        group.add_row(PrefsRow::new("theme".into(), "Theme".into()));
        group.add_row(PrefsRow::new("font".into(), "Font".into()));
        
        assert_eq!(group.rows.len(), 2);
    }
    
    #[test]
    fn test_settings() {
        let mut settings = Settings::new("org.sigmaos.App".into());
        settings.set("theme".into(), SettingValue::String("dark".into()));
        settings.set("fontsize".into(), SettingValue::Integer(12));
        
        assert_eq!(settings.get("theme"), Some(&SettingValue::String("dark".into())));
        assert_eq!(settings.get("fontsize"), Some(&SettingValue::Integer(12)));
    }
    
    #[test]
    fn test_xapp_creation() {
        let app = XApp::new("org.sigmaos.App".into());
        assert_eq!(app.app_id, "org.sigmaos.App".to_string());
        assert_eq!(app.state, AppState::Starting);
    }
    
    #[test]
    fn test_xapp_lifecycle() {
        let mut app = XApp::new("app".into());
        app.start();
        assert_eq!(app.state, AppState::Running);
        app.suspend();
        assert_eq!(app.state, AppState::Suspended);
        app.resume();
        assert_eq!(app.state, AppState::Running);
        app.stop();
        assert_eq!(app.state, AppState::Stopped);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = XappManager::new();
        assert!(manager.list_apps().is_empty());
        assert!(manager.list_windows().is_empty());
    }
    
    #[test]
    fn test_register_app() {
        let mut manager = XappManager::new();
        let app_id = manager.register_app("org.sigmaos.App".into());
        
        assert!(manager.get_app(&app_id).is_some());
        assert_eq!(manager.list_apps().len(), 1);
    }
    
    #[test]
    fn test_create_window() {
        let mut manager = XappManager::new();
        manager.initialize();
        let app_id = manager.register_app("org.sigmaos.App".into());
        
        let window_id = manager.create_window(&app_id, WindowKind::App);
        assert!(manager.get_window(&window_id).is_some());
    }
    
    #[test]
    fn test_create_prefs() {
        let mut manager = XappManager::new();
        manager.initialize();
        let app_id = manager.register_app("org.sigmaos.App".into());
        
        let prefs_id = manager.create_prefs(&app_id);
        assert!(manager.get_prefs(&prefs_id).is_some());
    }
    
    #[test]
    fn test_mgr_settings() {
        let mut manager = XappManager::new();
        manager.initialize();
        let app_id = manager.register_app("org.sigmaos.App".into());
        
        assert!(manager.set_setting(&app_id, "theme".into(), SettingValue::String("dark".into())));
        assert_eq!(manager.get_setting(&app_id, "theme"), Some(&SettingValue::String("dark".into())));
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = XappManager::new();
        manager.register_app("org.sigmaos.App".into());
        
        let export = manager.export_data();
        
        let mut manager2 = XappManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_apps().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = XappManager::new();
        manager.initialize();
        manager.register_app("org.sigmaos.App".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_apps, 1);
    }
}
