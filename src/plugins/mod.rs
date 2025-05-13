// src/plugins/mod.rs
pub mod core;
pub mod extra;

pub fn init_plugins(loader: &mut core::plugin_loader::PluginLoader) {
    #[cfg(feature = "clock")]
    loader.register_plugin(extra::clock::ClockPlugin::new());
}
