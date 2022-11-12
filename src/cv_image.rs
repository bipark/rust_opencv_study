use opencv::{
    imgcodecs::*,
    highgui::*,
};

pub fn show_image() {
    let img = imread("webcam.png", IMREAD_ANYCOLOR).unwrap();
    imshow("IMAGE", &img).unwrap();
    wait_key(1).unwrap();
    destroy_all_windows().unwrap();
}
