use active_win_pos_rs::get_active_window;
use anyhow::Result;
use arboard::Clipboard;
use enigo::{Enigo, Key as EnigoKey, KeyboardControllable};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct OSIntegration;

// Mock Clipboard for Testing
lazy_static::lazy_static! {
    static ref MOCK_CLIPBOARD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

impl OSIntegration {
    pub fn get_active_app_name() -> String {
        match get_active_window() {
            Ok(window) => window.app_name,
            Err(_) => "Unknown".to_string(),
        }
    }

    pub fn paste_text(text: &str) -> Result<()> {
        println!("[DEBUG] paste_text called with {} chars", text.len());

        // Runtime Mock Check for Integration Tests
        if env::var("VIBEFLOW_TEST_MODE").is_ok() {
            *MOCK_CLIPBOARD.lock().unwrap() = text.to_string();
            return Ok(());
        }

        let mut clipboard = Clipboard::new().map_err(|e| {
            println!("[ERROR] Failed to create clipboard: {:?}", e);
            e
        })?;

        // Security: Save previous state
        let original_content = clipboard.get_text().unwrap_or_default();
        println!(
            "[DEBUG] Saved original clipboard content ({} chars)",
            original_content.len()
        );

        // Set new content
        clipboard.set_text(text.to_owned()).map_err(|e| {
            println!("[ERROR] Failed to set clipboard text: {:?}", e);
            e
        })?;
        println!("[DEBUG] Set clipboard text successfully");

        // Small delay to ensure clipboard is ready
        thread::sleep(Duration::from_millis(100));

        let mut enigo = Enigo::new();
        println!("[DEBUG] Simulating Ctrl+V...");
        enigo.key_down(EnigoKey::Control);
        enigo.key_click(EnigoKey::Layout('v'));
        enigo.key_up(EnigoKey::Control);
        println!("[DEBUG] Ctrl+V simulation complete");

        // Security: Restore logic after 500ms
        thread::sleep(Duration::from_millis(500));

        // Restore
        let _ = clipboard.set_text(original_content);
        println!("[DEBUG] Restored original clipboard content");

        Ok(())
    }

    pub fn _get_mock_paste() -> String {
        MOCK_CLIPBOARD.lock().unwrap().clone()
    }
}
