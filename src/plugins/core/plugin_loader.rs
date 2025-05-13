use gtk4::Widget;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PanelPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Background,
}

#[derive(Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub position: PanelPosition,
    pub order: u32,
    pub priority: u32,
    pub deps: Vec<&'static str>,
}

pub trait Plugin {
    fn metadata(&self) -> PluginMetadata;
    fn on_start(&mut self);
    fn on_stop(&mut self);
    fn on_reload(&mut self);
    fn on_cleanup(&mut self);

    fn get_widget(&self) -> Option<(&Widget, AppendMethod)>;
}

#[derive(Clone, Copy)]
pub enum AppendMethod {
    Append,
    Prepend,
    SetContent,
}

pub struct PluginLoader {
    registry: HashMap<String, Box<dyn Plugin>>,
    disabled_plugins: Vec<String>,
    logger: Box<dyn Fn(&str) + Send + Sync>,
}

impl PluginLoader {
    pub fn new(logger: impl Fn(&str) + Send + Sync + 'static) -> Self {
        PluginLoader {
            registry: HashMap::new(),
            disabled_plugins: vec![],
            logger: Box::new(logger),
        }
    }

    pub fn register_plugin<T: 'static + Plugin>(&mut self, plugin: T) {
        let name = plugin.metadata().name.clone();
        self.registry.insert(name, Box::new(plugin));
    }

    pub fn load_plugins(&mut self) {
        let mut keys: Vec<String> = self.registry.keys().cloned().collect();
        keys.sort_by_key(|key| {
            let plugin = self.registry.get(key).unwrap();
            let meta = plugin.metadata();
            (meta.priority, meta.order)
        });

        for key in keys {
            // First get metadata immutably
            let meta = self.registry.get(&key).unwrap().metadata();

            if self.disabled_plugins.contains(&meta.name) {
                continue;
            }

            if !self.check_dependencies(self.registry.get(&key).unwrap().as_ref()) {
                continue;
            }

            // Now borrow mutably just once
            if let Some(plugin) = self.registry.get_mut(&key) {
                plugin.on_start();
                self.log(format!("Plugin started: {}", meta.name));
            }
        }
    }

    fn check_dependencies(&self, plugin: &dyn Plugin) -> bool {
        for dep in plugin.metadata().deps.iter() {
            if !self.registry.contains_key(*dep) {
                self.log(format!("Missing dependency: {}", dep));
                return false;
            }
        }
        true
    }

    pub fn get_panel_widgets(&self) -> HashMap<PanelPosition, Vec<(&Widget, AppendMethod)>> {
        let mut result: HashMap<PanelPosition, Vec<(&Widget, AppendMethod)>> = HashMap::new();

        for (_, plugin) in &self.registry {
            if let Some((widget, method)) = plugin.get_widget() {
                let pos = plugin.metadata().position;
                result.entry(pos).or_default().push((widget, method));
            }
        }

        result
    }

    fn log(&self, msg: String) {
        (self.logger)(&msg);
    }
}
