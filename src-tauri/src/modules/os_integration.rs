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
        // ... (existing paste_text) ...
        println!("[DEBUG] paste_text: {}", text);

        let mut clipboard = Clipboard::new()?;
        let original_content = clipboard.get_text().unwrap_or_default();
        clipboard.set_text(text.to_owned())?;

        thread::sleep(Duration::from_millis(100));
        let mut enigo = Enigo::new();
        enigo.key_down(EnigoKey::Control);
        enigo.key_click(EnigoKey::Layout('v'));
        enigo.key_up(EnigoKey::Control);

        thread::sleep(Duration::from_millis(500));
        let _ = clipboard.set_text(original_content);
        Ok(())
    }

    pub fn execute_command(command: crate::modules::llm::Command) -> Result<()> {
        let mut enigo = Enigo::new();
        match command {
            crate::modules::llm::Command::Delete => {
                // Ctrl+BackSpace or multiple Backspaces to "Delete that"
                // Let's do Ctrl+Shift+Left -> Backspace for "Delete word"
                enigo.key_down(EnigoKey::Control);
                enigo.key_down(EnigoKey::Shift);
                enigo.key_click(EnigoKey::LeftArrow);
                enigo.key_up(EnigoKey::Shift);
                enigo.key_up(EnigoKey::Control);
                enigo.key_click(EnigoKey::Backspace);
            }
            crate::modules::llm::Command::Bold => {
                // Select word and Ctrl+B
                enigo.key_down(EnigoKey::Control);
                enigo.key_down(EnigoKey::Shift);
                enigo.key_click(EnigoKey::LeftArrow);
                enigo.key_up(EnigoKey::Shift);
                enigo.key_click(EnigoKey::Layout('b'));
                enigo.key_up(EnigoKey::Control);
            }
            crate::modules::llm::Command::Italic => {
                enigo.key_down(EnigoKey::Control);
                enigo.key_down(EnigoKey::Shift);
                enigo.key_click(EnigoKey::LeftArrow);
                enigo.key_up(EnigoKey::Shift);
                enigo.key_click(EnigoKey::Layout('i'));
                enigo.key_up(EnigoKey::Control);
            }
            crate::modules::llm::Command::SelectAll => {
                enigo.key_down(EnigoKey::Control);
                enigo.key_click(EnigoKey::Layout('a'));
                enigo.key_up(EnigoKey::Control);
            }
            crate::modules::llm::Command::Enter => {
                enigo.key_click(EnigoKey::Return);
            }
        }
        Ok(())
    }
}
