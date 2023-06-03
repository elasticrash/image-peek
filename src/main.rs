use image::GenericImageView;
use std::env;
use yansi::Paint;

#[derive(Debug)]
enum MatchDimension {
    BBh,
    BS,
    SB,
    BBw,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let vip_char = '█';

    let img = image::open(filename).unwrap();

    let (term_height, term_width) = termsize::get().map(|size| (size.rows, size.cols)).unwrap();
    let (image_width, image_height) = img.dimensions();

    let scaler = if image_height > image_width && term_height > term_width {
        (
            MatchDimension::BBh,
            image_height,
            term_height,
            image_width as f32 / image_height as f32,
        )
    }  else if image_height < image_width && term_height > term_width {
        (
            MatchDimension::BS,
            image_width,
            term_height,
            image_height as f32 / image_width as f32,
        )
    } else if image_height > image_width && term_height < term_width {
        (
            MatchDimension::SB,
            image_height,
            term_width,
            image_width as f32 / image_height as f32,
        )
    } else if image_height < image_width && term_height < term_width {
        (
            MatchDimension::BBw,
            image_width,
            term_width,
            image_height as f32 / image_width as f32,
        )
    } else {
        panic!("Something went wrong")
    };

    match scaler.0 {
        MatchDimension::BBh => {
            let ratio: f32 =  image_height as f32 / term_height as f32;

            for y in 0..term_height as u32 {
                if y % 2 == 0 {
                    continue;
                }

                for x in 0..(term_height as f32 * scaler.3) as u32 {
                    let new_x = (x as f32 * ratio) as u32;
                    let new_y = (y as f32 * ratio) as u32;

                    let x_d = if new_x < img.dimensions().0 {
                        new_x
                    } else {
                        img.dimensions().0 - 1
                    };
                    let y_d = if new_y < img.dimensions().1 {
                        new_y
                    } else {
                        img.dimensions().1 - 1
                    };

                    let pixel = img.get_pixel(x_d, y_d);
                    print!("{}", Paint::rgb(pixel[0], pixel[1], pixel[2], vip_char));
                }
            }
        }
        MatchDimension::BS => {
            let ratio: f32 =  image_width as f32 / term_width as f32;

            for y in 0..(term_width as f32 * scaler.3) as u32 {
                if y % 2 == 0 {
                    continue;
                }
                for x in 0..term_width as u32 {
                    let new_x = (x as f32 * ratio) as u32;
                    let new_y = (y as f32 * ratio) as u32;
                    let x_d = if new_x < img.dimensions().0 {
                        new_x
                    } else {
                        img.dimensions().0 - 1
                    };
                    let y_d = if new_y < img.dimensions().1 {
                        new_y
                    } else {
                        img.dimensions().1 - 1
                    };
                    let pixel = img.get_pixel(x_d, y_d);
                    print!("{}", Paint::rgb(pixel[0], pixel[1], pixel[2], vip_char));
                }
                println!();
            }
        }
        MatchDimension::SB => {
            let ratio: f32 =  image_height as f32 / term_height as f32;
            
            for y in 0..term_height as u32 {
                 if y % 2 == 0 {
                    continue;
                }

                for x in 0..(term_height as f32 * scaler.3) as u32 {
                    let new_x = (x as f32 * ratio) as u32;
                    let new_y = (y as f32 * ratio) as u32;
                    let x_d = if new_x < img.dimensions().0 {
                        new_x
                    } else {
                        img.dimensions().0 - 1
                    };
                    let y_d = if new_y < img.dimensions().1 {
                        new_y
                    } else {
                        img.dimensions().1 - 1
                    };
                    let pixel = img.get_pixel(x_d, y_d);
                    print!("{}", Paint::rgb(pixel[0], pixel[1], pixel[2], vip_char));
                }
                println!();
            }
        }
        MatchDimension::BBw => {
            let ratio: f32 =  image_height as f32 / term_height as f32;
            
            for y in 0..(term_height as f32 * scaler.3) as u32 {
                if y % 2 == 0 {
                    continue;
                }
                for x in 0..term_height as u32 {
                    let new_x = (x as f32 * ratio) as u32;
                    let new_y = (y as f32 * ratio) as u32;
                    let x_d = if new_x < img.dimensions().0 {
                        new_x
                    } else {
                        img.dimensions().0 - 1
                    };
                    let y_d = if new_y < img.dimensions().1 {
                        new_y
                    } else {
                        img.dimensions().1 - 1
                    };
                    let pixel = img.get_pixel(x_d, y_d);
                    print!("{}", Paint::rgb(pixel[0], pixel[1], pixel[2], vip_char));
                }
                println!();
            }
        }
    }
}
