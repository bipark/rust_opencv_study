use opencv::{
    core::*,
    imgcodecs::*,
    highgui::*,
};

fn load_image() -> Mat {
    let img = imread("webcam.png", IMREAD_ANYCOLOR).unwrap();
    return img;
}

pub fn show_image() {
    let img = load_image();
    imshow("IMAGE", &img).unwrap();
    wait_key(1).unwrap();
    destroy_all_windows().unwrap();    
}

pub fn crop_image() {
    let img = load_image();
    let cropped = Mat::roi(&img, Rect_ { x: (100), y: (100), width: (300), height: (300) }).unwrap();
    imwrite("result.png", &cropped, &Vector::new()).ok();
}