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

#[derive(Debug)]
pub struct View {
    term_width: u32,
    term_height: u32,
    image_width: u32,
    image_height: u32,
}

#[derive(Debug, PartialEq)]
pub struct PrintSize {
    width: u32,
    height: u32,
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
    } else if image_height < image_width && term_height > term_width {
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

    let ps = size(View {
        term_width: term_width as u32,
        term_height: term_height as u32,
        image_width: image_width as u32,
        image_height: image_height as u32,
    });

    match scaler.0 {
        MatchDimension::BBh | MatchDimension::SB => {
            let ratio: f32 = image_height as f32 / term_height as f32;

            for y in 0..ps.height {
                if y % 2 == 0 {
                    continue;
                }

                for x in 0..ps.width {
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
        MatchDimension::BS | MatchDimension::BBw => {
            let ratio: f32 = image_width as f32 / term_width as f32;

            for y in 0..ps.height {
                if y % 2 == 0 {
                    continue;
                }
                for x in 0..ps.width {
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

fn size(view: View) -> PrintSize {
    println!("TERMINAL: {:?}    {:?}", view.term_width, view.term_height);
    println!("IMAGE:    {:?}    {:?}", view.image_width, view.image_height);
    let new_width = (view.image_width as f32 / view.image_height as f32) * view.term_height as f32;
    let size_a = PrintSize {
        width: new_width as u32,
        height: view.term_height,
    };
    println!("PSIZE A:  {:?}", size_a);

    let new_height = (view.image_height as f32 / view.image_width as f32) * view.term_width as f32;

    let size_b = PrintSize {
        width: view.term_width,
        height: new_height as u32,
    };

    println!("PSIZE B:  {:?}", size_b);

    // get bigger
    let comparison = if size_a.width > size_b.width && size_a.height > size_b.height {
        (size_a, size_b)
    } else {
        (size_b, size_a)
    };

    println!("BIGGER:   {:?}", comparison);

    // if it doesnt fit get smaller
    let fit = if comparison.0.width > view.term_width || comparison.0.height > view.term_height {
        (comparison.1, comparison.0)
    } else {
        (comparison.0, comparison.1)
    };

    println!("FIT:     {:?}", fit);

    // check if smaller can fit twice
    let double_fit = if fit.0.width * 2 <= view.term_width && fit.0.height <= view.term_height {
        PrintSize {
            width: fit.0.width * 2,
            height: fit.0.height * 2,
        }
    } else {
        fit.0
    };

    println!("DOUBLE:  {:?}", double_fit);

    double_fit
}

#[cfg(test)]
mod tests {
    use crate::size;
    use crate::PrintSize;
    use crate::View;
    #[test]
    fn test_size_wide_terminal_wide_image() {
        let view = View {
            term_width: 200,
            term_height: 50,
            image_width: 1000,
            image_height: 800,
        };

        println!("view: {:?}", view);
        assert_eq!(
            size(view),
            PrintSize {
                width: 124,
                height: 100
            }
        );
    }
}
