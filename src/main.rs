use opencv::{
    highgui,
    prelude::*,
    videoio,
    core,
    Result,
};

fn main() -> Result<()> {
    // Open the default camera (usually camera 0)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; 
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open default camera!");
    }

    let window = "Webcam Feed";
    highgui::named_window(window, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window, 800, 600)?;

    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }

        // Break loop if 'q' is pressed
        let key = highgui::wait_key(10)?;
        if key == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_window(window)?;

    Ok(())
}

