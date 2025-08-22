use opencv::{
    highgui,
    core::Mat,
    Result,
};

pub struct WebcamWindow {
    name: String,
    width: i32,
    height: i32,
}

impl WebcamWindow {
    pub fn new(name: &str, width: i32, height: i32) -> Result<Self> {
        let webcam = Self {
            name: name.to_string(),
            width,
            height,
        };

        highgui::named_window(name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(name, webcam.width, webcam.height)?;

        Ok(webcam)
    }

    pub fn show(&self, frame: &Mat) -> Result<()> {
        highgui::imshow(&self.name, frame)?;
        Ok(())
    }

    pub fn wait_key(&self, delay: i32) -> Result<i32> {
        highgui::wait_key(delay)
    }

    pub fn close(&self) -> Result<()> {
        highgui::destroy_window(&self.name)
    }
}