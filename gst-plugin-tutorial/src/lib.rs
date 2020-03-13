extern crate chrono;
#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_video as gst_video;
extern crate once_cell;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    Ok(())
}

gst_plugin_define!(
    rstutorial,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!(chrono::Utc::now().format("%Y-%m-%d").to_string())
);
