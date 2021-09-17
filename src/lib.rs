use gst::glib;

mod datasink;
mod datasrc;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    datasink::register(plugin)?;
    datasrc::register(plugin)?;
    Ok(())
}

gst::plugin_define!(
    gstdatavideo,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
