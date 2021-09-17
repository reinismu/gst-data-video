use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst::{gst_debug, gst_info, gst_warning};
use gst_base::prelude::*;
use gst_base::subclass::prelude::*;

use std::sync::Mutex;
use std::{i32, u32};

use once_cell::sync::Lazy;

use bytes::BufMut;

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "datasrc",
        gst::DebugColorFlags::empty(),
        Some("Encode data in video stream"),
    )
});

struct State {
    info: Option<gst_video::VideoInfo>,
}

impl Default for State {
    fn default() -> State {
        State { info: None }
    }
}

// Struct containing all the element data
#[derive(Default)]
pub struct DataSrc {
    state: Mutex<State>,
}

impl DataSrc {}

#[glib::object_subclass]
impl ObjectSubclass for DataSrc {
    const NAME: &'static str = "DataSrc";
    type Type = super::DataSrc;
    type ParentType = gst_base::PushSrc;
}

impl ObjectImpl for DataSrc {
    // Called right after construction of a new instance
    fn constructed(&self, obj: &Self::Type) {
        // Call the parent class' ::constructed() implementation first
        self.parent_constructed(obj);

        // Initialize live-ness and notify the base class that
        // we'd like to operate in Time format
        obj.set_live(false);
        obj.set_format(gst::Format::Time);
    }
}

impl ElementImpl for DataSrc {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Data Source",
                "Source/Data",
                "Read data into stream",
                "Reinis Muižnieks <muiznieks.reinis@gmail.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            let caps = gst::Caps::new_simple(
                "video/x-raw",
                &[
                    ("format", &gst_video::VideoFormat::Uyvy.to_str()),
                    ("width", &gst::IntRange::<i32>::new(1920, i32::MAX)),
                    ("height", &gst::IntRange::<i32>::new(1080, i32::MAX)),
                    (
                        "framerate",
                        &gst::FractionRange::new(
                            gst::Fraction::new(0, 1),
                            gst::Fraction::new(i32::MAX, 1),
                        ),
                    ),
                ],
            );

            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &caps,
            )
            .unwrap();

            vec![src_pad_template]
        });
        PAD_TEMPLATES.as_ref()
    }
}

impl BaseSrcImpl for DataSrc {
    // Called when starting, so we can initialize all stream-related state to its defaults
    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        // Reset state
        *self.state.lock().unwrap() = Default::default();

        gst_info!(CAT, obj: element, "Started");

        Ok(())
    }

    // Called when shutting down the element so we can release all stream-related state
    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        // Reset state
        *self.state.lock().unwrap() = Default::default();

        gst_info!(CAT, obj: element, "Stopped");

        Ok(())
    }

    fn set_caps(&self, element: &Self::Type, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        let info = gst_video::VideoInfo::from_caps(caps).map_err(|_| {
            gst::loggable_error!(CAT, "Failed to build `VideoInfo` from caps {}", caps)
        })?;

        gst_debug!(CAT, obj: element, "Configuring for caps {}", caps);

        element.set_blocksize(info.width() * info.height() * 4);

        let mut state = self.state.lock().unwrap();

        *state = State { info: Some(info) };

        drop(state);

        let _ = element.post_message(gst::message::Latency::builder().src(element).build());

        Ok(())
    }
}

impl PushSrcImpl for DataSrc {
    fn create(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError> {
        let state = self.state.lock().unwrap();
        let info = match state.info {
            None => {
                gst::element_error!(element, gst::CoreError::Negotiation, ["Have no caps yet"]);
                return Err(gst::FlowError::NotNegotiated);
            }
            Some(ref info) => info.clone(),
        };

        let buffer_size = (info.width() as usize) * (info.height() as usize) * 4;

        // Text to encode
        let input = "Hello world";

        let mut buffer = gst::Buffer::with_size(buffer_size).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();

            // Map the buffer writable and create the actual samples
            let mut map = buffer.map_writable().unwrap();
            let mut data = map.as_mut_slice();

            data.put_u32(input.len() as u32);
            data.put(input.as_bytes());
        }
        drop(state);

        gst_warning!(
            CAT,
            obj: element,
            "Created buffer with size {}",
            buffer_size
        );

        Ok(buffer)
    }
}