use crate::plugins::core::plugin_loader::{AppendMethod, PanelPosition, Plugin};
use chrono::Local;
use glib::{ControlFlow, timeout_add_seconds_local};
use gtk4::Label;
use gtk4::prelude::*;

pub struct ClockPlugin {
    label: Label,
    timer_id: Option<glib::SourceId>,
}

impl ClockPlugin {
    pub fn new() -> Self {
        Self {
            label: Label::new(Some("00:00:00")),
            timer_id: None,
        }
    }
}

impl Plugin for ClockPlugin {
    fn metadata(&self) -> crate::plugins::core::plugin_loader::PluginMetadata {
        crate::plugins::core::plugin_loader::PluginMetadata {
            name: "clock".to_string(),
            position: PanelPosition::TopCenter,
            order: 10,
            priority: 5,
            deps: vec![],
        }
    }

    fn on_start(&mut self) {
        let label_clone = self.label.clone();
        self.timer_id = Some(timeout_add_seconds_local(1, move || {
            let now = Local::now();
            label_clone.set_text(&now.format("%H:%M:%S").to_string());
            ControlFlow::Continue
        }));
    }

    fn on_stop(&mut self) {
        if let Some(timer) = self.timer_id.take() {
            timer.remove();
        }
    }

    fn on_reload(&mut self) {
        self.on_stop();
        self.on_start();
    }

    fn on_cleanup(&mut self) {
        self.on_stop();
    }

    fn get_widget(&self) -> Option<(&gtk4::Widget, AppendMethod)> {
        Some((
            &self.label.upcast_ref::<gtk4::Widget>(),
            AppendMethod::Append,
        ))
    }
}
