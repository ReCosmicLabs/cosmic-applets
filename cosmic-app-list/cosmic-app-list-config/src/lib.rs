// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{
    self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub const APP_ID: &str = "com.system76.CosmicAppList";

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub enum ToplevelFilter {
    #[default]
    ActiveWorkspace,
    ConfiguredOutput,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, CosmicConfigEntry)]
#[version = 1]
pub struct AppListConfig {
    pub filter_top_levels: Option<ToplevelFilter>,
    pub favorites: Vec<String>,
    pub enable_drag_source: bool,
    /// App IDs that never show up in the list, running or not.
    pub ignored: Vec<String>,
    /// Draw the rule between pinned apps and running ones.
    pub show_divider: bool,
    /// Open the window list of an app with several windows when the pointer rests on its
    /// icon for this many milliseconds; None keeps the click-only behavior.
    pub hover_popup_delay_ms: Option<u32>,
    /// Clicking the icon of an app with several windows raises the last one that was focused
    /// instead of opening the window list (the list stays available on hover).
    pub click_last_window: bool,
    /// Show an unread counter on the icon, taken from the "(N)" prefix apps like Discord and
    /// WhatsApp put in their window title.
    pub title_badge: bool,
}

impl Default for AppListConfig {
    fn default() -> Self {
        Self {
            filter_top_levels: None,
            favorites: Vec::new(),
            enable_drag_source: true,
            ignored: Vec::new(),
            show_divider: true,
            hover_popup_delay_ms: None,
            click_last_window: false,
            title_badge: false,
        }
    }
}

impl AppListConfig {
    pub fn add_pinned(&mut self, id: String, config: &Config) {
        if !self.favorites.contains(&id) {
            self.favorites.push(id);
            let _ = self.write_entry(config);
        }
    }

    pub fn remove_pinned(&mut self, id: &str, config: &Config) {
        if let Some(pos) = self.favorites.iter().position(|e| e == id) {
            self.favorites.remove(pos);
            let _ = self.write_entry(config);
        }
    }

    pub fn update_pinned(&mut self, favorites: Vec<String>, config: &Config) {
        self.favorites = favorites;
        let _ = self.write_entry(config);
    }
}
