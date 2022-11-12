use console::Term;

mod canny;
mod cv_image;
mod cv_video;

//------------------------------------------------------------------------------//
fn main() {
    let stdout = Term::buffered_stdout();
    loop {
        if let Ok(input) = stdout.read_char() {
            println!("Your input... {}", input);
            match input {
                'c' => canny::my_canny(),
                's' => cv_image::show_image(),
                'r' => cv_image::crop_image(),
                'v' => cv_video::open_video(),
                'x' => {
                    break;
                },
                _ => continue,
            }
        }    
    }
}

