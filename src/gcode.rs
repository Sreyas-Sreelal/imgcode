use std::fs::File;
use std::io::Write;
use crate::RasterImageProcessor;

pub struct GcodeWriter {
    handle: std::fs::File,
}

impl GcodeWriter {
    pub fn new(out_path: &str) -> Self {
        let file = match File::create(out_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Output file can't be created!! {}",err);
            }
        };

        GcodeWriter {
            handle: file
        }
    }
    
    pub fn decode(&mut self,processor: RasterImageProcessor) -> Result<(),std::io::Error> {
        self.handle.write(b"G17 G21 G90 G54\n")?;
        self.handle.write(b"G00 X0. Y0.\n")?;
        self.handle.write(b"Z0.\n")?;
        for (x,y) in processor.processed_pixels {
            self.handle.write(format!("X{}. Y{}.\n",x,y).as_bytes())?;
        }
        self.handle.write(b"Z1.\n")?;
        self.handle.write(b"X0. Y0.\nM2")?;
        Ok(())
    }
}