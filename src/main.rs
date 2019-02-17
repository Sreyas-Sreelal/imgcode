mod raster;
mod gcode;
use crate::raster::RasterImageProcessor;

fn main() -> Result<(),std::io::Error> {
    let mut processor = RasterImageProcessor::new("image-samples/circle.png");
    //println!("height {} width {}",processor.img.height(),processor.img.width());
    processor.trace_shape();
    /*
    let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(processor.img.width(), processor.img.height());
    for (x,y) in &processor.processed_pixels{
        imgbuf.put_pixel(*x,*y,*processor.img.get_pixel(*x, *y));
    }
    imgbuf.save("check.jpg")?;
    */
    let mut writer = gcode::GcodeWriter::new("sample.gcode");
    writer.decode(processor)?;
    Ok(())
}
