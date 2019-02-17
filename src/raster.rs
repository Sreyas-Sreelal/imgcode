use image::imageops::FilterType;
use image::GenericImageView;

pub struct RasterImageProcessor {
    pub img: image::RgbaImage,
    pub current: (u32, u32),
    pub processed_pixels: Vec<(u32, u32)>,
}

impl RasterImageProcessor {
    pub fn new(path: &str) -> RasterImageProcessor {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(err) => {
                panic!("Error couldn't read image! {}", err);
            }
        };

        let img = img
            .resize(305, 305, FilterType::CatmullRom)
            .adjust_contrast(2.0)
            .to_rgba();
        //img.save("test.jpeg");

        println!("dimension {:?}", img.dimensions());
        //println!("color {:?}", img.);

        RasterImageProcessor {
            img,
            current: (0, 0),
            processed_pixels: Vec::new(),
        }
    }

    pub fn trace_shape(&mut self) {
        for x in 0..self.img.width() {
            for y in 0..self.img.height() {
                let pixel = self.img.get_pixel(x, y);
                let pixel_sum = pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32;
                if pixel_sum < 381
                    && self.is_edge((x as i32, y as i32))
                    && !self.processed_pixels.contains(&(x, y))
                {
                    self.processed_pixels.push((x, y));
                    self.current = (x, y);
                    println!("{:?}", self.current);
                    //return true;
                }
            }
        }
        //return false;
    }

    fn get_hue(&self, coord: (i32, i32)) -> u32 {
        if coord.0 < 0 || coord.1 < 0 {
            return 255 * 3;
        } else if self.img.in_bounds(coord.0 as u32, coord.1 as u32) {
            let px = self.img.get_pixel(coord.0 as u32, coord.1 as u32);
            let px_sum = px[0] as u32 + px[1] as u32;
            px_sum
        } else {
            255 * 3
        }
    }

    fn is_edge(&self, coord: (i32, i32)) -> bool {
        let mut hues: Vec<u32> = Vec::new();
        hues.push(self.get_hue((coord.0 - 1, coord.1)));
        hues.push(self.get_hue((coord.0, coord.1 - 1)));
        hues.push(self.get_hue((coord.0 + 1, coord.1)));
        hues.push(self.get_hue((coord.0, coord.1 + 1)));

        if *hues.iter().max().unwrap() > 127 * 3 {
            return true;
        }
        false
    }
}
