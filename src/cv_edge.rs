use opencv::{
    core::*,
    imgcodecs::*,
    imgproc::*,
};

pub fn canny_image() {
    let src_img = imread("webcam.png", IMREAD_GRAYSCALE).unwrap();
    let mut edge_img = Mat::default();
    canny(&src_img, &mut edge_img, 100.0, 100.0, 3, false).unwrap();
    imwrite("result.png", &edge_img, &Vector::new()).ok();
}

pub fn sobel_image() {
    let src_img = imread("webcam.png", IMREAD_COLOR).unwrap();
    let mut target = Mat::default();
    sobel(&src_img, &mut target, CV_8U, 1, 1, 3, 1.0, 1.0, BORDER_DEFAULT).unwrap();
    imwrite("result.png", &target, &Vector::new()).ok();
}