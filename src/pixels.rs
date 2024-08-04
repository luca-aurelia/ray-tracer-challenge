pub use pixels::*;

#[cfg(test)]
pub type Pixels = TestPixels;

#[cfg(not(test))]
pub type Pixels = pixels::Pixels;

pub const NUM_COMPONENTS_IN_COLOR: usize = 4;

pub struct TestPixels {
    pub width: u32,
    pub height: u32,
    components: Vec<u8>,
}

impl TestPixels {
    pub fn new(width: u32, height: u32) -> Self {
        TestPixels {
            width,
            height,
            components: vec![0; (width * height * NUM_COMPONENTS_IN_COLOR as u32) as usize],
        }
    }
    pub fn frame(&self) -> &[u8] {
        &self.components
    }
    pub fn frame_mut(&mut self) -> &mut [u8] {
        &mut self.components
    }
    pub fn render(&self) -> Result<(), TestPixelsError> {
        Ok(())
    }
    pub fn resize_surface(&mut self, width: u32, height: u32) -> Result<(), TestPixelsError> {
        self.width = width;
        self.height = height;
        self.components = vec![0; (width * height * NUM_COMPONENTS_IN_COLOR as u32) as usize];
        Ok(())
    }
}

#[derive(Debug)]
pub struct TestPixelsError;

impl std::fmt::Display for TestPixelsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestPixelsError")
    }
}

impl std::error::Error for TestPixelsError {}
