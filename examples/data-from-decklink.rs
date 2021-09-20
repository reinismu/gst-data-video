use glib::Error;
use gst::prelude::*;

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init().unwrap();
    gst::debug_set_default_threshold(gst::DebugLevel::Error);

    let pipeline = gst::Pipeline::new(None);

    let system_clock = gst::SystemClock::obtain();
    system_clock
        .set_property("clock-type", &gst::ClockType::Realtime)
        .unwrap();

    pipeline.set_clock(Some(&system_clock)).unwrap();

    // let src = gst::ElementFactory::make("datasrc", None).unwrap();

    let src = gst::ElementFactory::make("decklinkvideosrc", None).unwrap();
    src.set_property_from_str("device-number", "2");
    src.set_property_from_str("mode", "1080p25");

    let video_filter = gst::ElementFactory::make("capsfilter", None).unwrap();
    video_filter
        .set_property(
            "caps",
            &gst::Caps::builder("video/x-raw")
                .field("width", &1920)
                .field("height", &1080)
                .field("framerate", &gst::Fraction::new(25, 1))
                .build(),
        )
        .unwrap();

    let datasink = gst::ElementFactory::make("datasink", None).unwrap();

    datasink
        .connect("data-received", true, |val| {
            let content = val[1].get::<String>().unwrap();
            println!("Received data! {:?}", content);
            None
        })
        .unwrap();

    pipeline
        .add_many(&[&src, &video_filter, &datasink])
        .unwrap();

    gst::Element::link_many(&[&src, &video_filter, &datasink]).unwrap();

    Ok(pipeline)
}

fn main() -> Result<(), Error> {
    let pipeline = create_pipeline().unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Error(err) => println!(
                "Error received {:?}: {}",
                err.src().map(|s| String::from(s.path_string())),
                err.error(),
            ),
            gst::MessageView::StateChanged(state) => println!(
                "State changed from {:?} to {:?}, {:?}",
                state.old(),
                state.current(),
                state.src()
            ),
            gst::MessageView::Latency(latency) => println!("Latency {:?}", latency.src()),
            gst::MessageView::Element(element) => {
                println!("Element: {:?}", element);
            }
            _ => {
                println!("Other");
            }
        }
    }

    Ok(())
}
