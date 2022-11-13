use console::Term;

mod cv_edge;
mod cv_image;
mod cv_video;
mod cv_contour;

//------------------------------------------------------------------------------//
fn main() {
    let stdout = Term::buffered_stdout();
    loop {
        if let Ok(input) = stdout.read_char() {
            println!("Your input... {}", input);
            match input {
                'c' => cv_edge::canny_image(),
                'l' => cv_edge::sobel_image(),
                's' => cv_image::show_image(),
                'r' => cv_image::crop_image(),
                't' => cv_image::threshold_image(),
                'b' => cv_image::blur_image(),
                'v' => cv_video::open_video(),
                'p' => cv_video::save_video(),
                'o' => cv_contour::contour_image(),
                'x' => {
                    break;
                },
                _ => continue,
            }
        }    
    }
}

