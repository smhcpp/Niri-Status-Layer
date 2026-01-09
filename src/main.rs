use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, CssProvider};
// Use Edge instead of Anchor for 0.7.1+
use gtk4_layer_shell::{Layer, LayerShell, Edge};

fn main() {
    let app = Application::builder()
        .application_id("com.mortimertz.status_layer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .build();

        // 1. Initialize Layer Shell
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        
        // 2. Center the window (Edge replaces Anchor in new API)
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, false);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Right, false);
        window.set_default_size(300, 100);

        // 3. Apply Styling
        let provider = CssProvider::new();
        provider.load_from_data("
            window {
                background-color: rgba(30, 30, 46, 0.8);
                border-radius: 12px;
                border: 2px solid #585b70;
            }
            label {
                color: #cdd6f4;
                font-size: 24px;
                font-weight: bold;
                padding: 20px;
            }
        ");
        
        // Use the modern way to add the CSS provider to avoid warnings
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let label = Label::new(Some("System Ready"));
        label.set_margin_start(20);
        label.set_margin_end(20);
        window.set_child(Some(&label));
        
        window.present();
    });

    app.run();
}
