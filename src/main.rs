mod plugins;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use plugins::core::plugin_loader::{AppendMethod, PanelPosition, PluginLoader};
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    // Initialize GTK
    gtk4::init().expect("Failed to initialize GTK.");
    let logger = Arc::new(Mutex::new(|msg: &str| println!("[PLUGIN] {}", msg)));
    let logger_clone = Arc::clone(&logger);

    let mut loader = PluginLoader::new(move |msg| {
        let logger = logger_clone.lock().unwrap();
        logger(msg);
    });

    // Register plugins (clock plugin is conditionally compiled based on feature)
    #[cfg(feature = "clock")]
    loader.register_plugin(plugins::extra::clock::ClockPlugin::new());

    // Load all plugins
    loader.load_plugins();

    // Build UI
    let app = Application::builder()
        .application_id("com.example.waypanel")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Waypanel Clone")
            .default_width(800)
            .default_height(30)
            .build();

        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.auto_exclusive_zone_enable();

        let bottom_box = GtkBox::new(Orientation::Horizontal, 0);

        let widgets = loader.get_panel_widgets();

        let top_box = GtkBox::new(Orientation::Horizontal, 0);
        let top_start = GtkBox::new(Orientation::Horizontal, 0);
        let top_center = GtkBox::new(Orientation::Horizontal, 0);
        let top_end = GtkBox::new(Orientation::Horizontal, 0);

        let bottom_box = GtkBox::new(Orientation::Horizontal, 0);
        let bottom_start = GtkBox::new(Orientation::Horizontal, 0);
        let bottom_center = GtkBox::new(Orientation::Horizontal, 0);
        let bottom_end = GtkBox::new(Orientation::Horizontal, 0);

        // Use spacers to push content apart
        let spacer1 = gtk4::Separator::new(Orientation::Horizontal);
        let spacer2 = gtk4::Separator::new(Orientation::Horizontal);
        spacer1.set_vexpand(false);
        spacer2.set_vexpand(false);
        spacer1.set_hexpand(true);
        spacer2.set_hexpand(true);

        top_box.append(&top_start);
        top_box.append(&spacer1);
        top_box.append(&top_center);
        top_box.append(&spacer2);
        top_box.append(&top_end);

        for (pos, items) in widgets {
            for (widget, method) in items {
                match pos {
                    PanelPosition::TopLeft => top_start.append(widget),
                    PanelPosition::TopCenter => top_center.append(widget),
                    PanelPosition::TopRight => top_end.append(widget),
                    PanelPosition::BottomLeft => bottom_box.append(widget),
                    PanelPosition::BottomCenter => bottom_center.append(widget),
                    PanelPosition::BottomRight => bottom_end.append(widget),
                    PanelPosition::Background => continue,
                }
            }
        }

        let panel_box = GtkBox::new(Orientation::Vertical, 1);
        panel_box.append(&top_box);
        panel_box.append(&bottom_box);

        window.set_child(Some(&panel_box));
        window.show();
    });

    app.run();
}
