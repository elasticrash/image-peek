use image::GenericImageView;
use std::env;
use yansi::Paint;

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

    match args.len() {
        2 => {
            let filename = &args[1];
            draw(filename, false);
        }
        3 => {
            let filename = &args[1];
            let bw = &args[2];
            let black_white = bw == "bw";
            draw(filename, black_white);
        }
        _ => {
            println!("Usage: image-peek <filename> <logging>");
        }
    }
}

fn draw(filename: &str, black_white: bool) {
    let light_shade = '░';
    let medium_shade = '▒';
    let dark_shade = '▓';
    let full_block = '█';
    
    let img = image::open(filename).unwrap();

    let (term_height, term_width) = termsize::get().map(|size| (size.rows, size.cols)).unwrap();
    let (image_width, image_height) = img.dimensions();

    let ps = size(
        View {
            term_width: term_width as u32,
            term_height: term_height as u32,
            image_width,
            image_height,
        },
        false,
    );

    let ratio = if ps.width > ps.height {
        image_width as f32 / ps.width as f32
    } else {
        image_height as f32 / ps.height as f32
    };

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
            if !black_white {
                let pixel = img.get_pixel(x_d, y_d);
                print!("{}", Paint::rgb(pixel[0], pixel[1], pixel[2], full_block));
            } else {
                let pixel = img.get_pixel(x_d, y_d);
                let avg = ((pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3) as u8;
                if avg < 64 {
                    print!("{}", Paint::rgb(0, 0, 0, full_block));
                } else if avg < 128 {
                    print!("{}", Paint::rgb(0, 0, 0, dark_shade));
                } else if avg < 192 {
                    print!("{}", Paint::rgb(0, 0, 0, medium_shade));
                } else {
                    print!("{}", Paint::rgb(0, 0, 0, light_shade));
                }
            }
        }
        println!();
    }
}

fn size(view: View, exlog: bool) -> PrintSize {
    if exlog {
        println!("TERMINAL: {:?}    {:?}", view.term_width, view.term_height);
        println!(
            "IMAGE:    {:?}    {:?}",
            view.image_width, view.image_height
        );
    }
    let new_width = (view.image_width as f32 / view.image_height as f32) * view.term_height as f32;
    let size_a = PrintSize {
        width: new_width as u32,
        height: view.term_height,
    };

    if exlog {
        println!("PSIZE A:  {size_a:?}");
    }
    let new_height = (view.image_height as f32 / view.image_width as f32) * view.term_width as f32;

    let size_b = PrintSize {
        width: view.term_width,
        height: new_height as u32,
    };
    if exlog {
        println!("PSIZE B:  {size_b:?}");
    }
    // get bigger
    let comparison = if size_a.width > size_b.width && size_a.height > size_b.height {
        (size_a, size_b)
    } else {
        (size_b, size_a)
    };
    if exlog {
        println!("BIGGER:   {comparison:?}");
    }
    // if it doesnt fit get smaller
    let fit = if comparison.0.width > view.term_width || comparison.0.height > view.term_height {
        (comparison.1, comparison.0)
    } else {
        (comparison.0, comparison.1)
    };
    if exlog {
        println!("FIT:     {fit:?}");
    }
    // check if smaller can fit twice
    let double_fit = if fit.0.width * 2 <= view.term_width && fit.0.height <= view.term_height {
        PrintSize {
            width: fit.0.width * 2,
            height: fit.0.height * 2,
        }
    } else {
        fit.0
    };
    if exlog {
        println!("DOUBLE:  {double_fit:?}");
    }
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
            size(view, true),
            PrintSize {
                width: 124,
                height: 100
            }
        );
    }

    #[test]
    fn test_size_narrow_terminal_wide_image() {
        let view = View {
            term_width: 50,
            term_height: 200,
            image_width: 1000,
            image_height: 800,
        };

        println!("view: {:?}", view);
        assert_eq!(
            size(view, true),
            PrintSize {
                width: 50,
                height: 40
            }
        );
    }

    #[test]
    fn test_size_wide_terminal_narrow_image() {
        let view = View {
            term_width: 200,
            term_height: 50,
            image_width: 800,
            image_height: 1000,
        };

        println!("view: {:?}", view);
        assert_eq!(
            size(view, true),
            PrintSize {
                width: 80,
                height: 100
            }
        );
    }

    #[test]
    fn test_size_narrow_terminal_narrow_image() {
        let view = View {
            term_width: 50,
            term_height: 200,
            image_width: 800,
            image_height: 1000,
        };

        println!("view: {:?}", view);
        assert_eq!(
            size(view, true),
            PrintSize {
                width: 50,
                height: 62
            }
        );
    }
}
