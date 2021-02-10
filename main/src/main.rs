use screen_capture;
use std::io::Write;     
use std::fs::File;
use lodepng;
use bmp;

fn main() {

    println!("test");

    let result = screen_capture::DXGIManager::new(0);

    if let Ok(mut capturer) = result {
       for i in 0..20 {
        let vec = capturer.capture_frame();
        if let Ok((img, (x, y))) = vec {
            
            let mut final_img: Vec<bmp::Pixel> = vec![];

            println!("{:?} {:?}", x, y);



            if(i == 10){
                println!("printing");
                for j in img {
                    final_img.push( bmp::Pixel::new(j.r, j.g, j.b));
                }
    
                let mut img = bmp::Image::new(x as u32, y as u32);

            for row in 0..x {
                for column in 0.. y {
                    let s = (column * x) + row;
                    img.set_pixel(row as u32, column as u32, *final_img.get(s).expect("failed"));
                }
            }


                let _ = img.save("img.bmp");
                
                // let mut result_img = lodepng::encode_memory(&final_img, x, y, lodepng::ColorType::RGBA, 8).expect("failed encoding");
                
                // let a = image::load_from_memory_with_format(&final_img, image::ImageFormat::Bmp).expect("failedddd");

                 print!("came here");
               //  std::fs::write("out11.png", final_img).expect("failed write");
            }
        }
        
        std::thread::sleep_ms(30);
       }
    }

}