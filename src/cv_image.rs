use opencv::{
    core::*,
    imgcodecs::*,
    imgproc::*,
    highgui::*, 
};

fn load_image(flag: i32) -> Mat {
    let img = imread("webcam.png", flag).unwrap();
    return img;
}

pub fn show_image() {
    let img = load_image(IMREAD_ANYCOLOR);
    imshow("IMAGE", &img).unwrap();
    wait_key(1).unwrap();
    destroy_all_windows().unwrap();    
}

pub fn crop_image() {
    let img = load_image(IMREAD_ANYCOLOR);
    let cropped = Mat::roi(&img, Rect_ { x: (100), y: (100), width: (300), height: (300) }).unwrap();
    imwrite("result.png", &cropped, &Vector::new()).ok();
}

pub fn threshold_image() {
    let src = load_image(IMREAD_GRAYSCALE);
    let mut target = Mat::default();
    threshold(&src, &mut target, 128.0, 255.0, THRESH_BINARY).unwrap();
    imwrite("result.png", &target, &Vector::new()).ok();
}