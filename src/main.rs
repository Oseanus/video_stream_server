mod window;

use opencv::{
    prelude::*,
    videoio,
    core,
    Result,
};

use window::WebcamWindow;

fn main() -> Result<()> {
    // Open the default camera (usually camera 0)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; 
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open default camera!");
    }

    let window = WebcamWindow::new("Webcam Feed", 800, 600)?;

    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            window.show(&frame)?;
        }

        // Break loop if 'q' is pressed
        let key = window.wait_key(10)?;
        if key == 'q' as i32 {
            break;
        }
    }

    window.close()?;

    Ok(())
}

