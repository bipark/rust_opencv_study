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

pub fn blur_image() {
    let img = load_image(IMREAD_ANYCOLOR);
    let mut target = Mat::default();
    blur(&img, &mut target, Size_ { width: (32), height: (32) }, Point_ { x: (-1), y: (-1) }, BORDER_DEFAULT).unwrap();
    imwrite("result.png", &target, &Vector::new()).ok();
}

pub fn morphology_image(kind:i8) {
    let img = load_image(IMREAD_ANYCOLOR);
    let mut target = Mat::default();
    let anchor = Point::new(0, 0);
    let size = Size::new(2,2);
    let border = Scalar::new(0.0, 0.0, 0.0, 0.0);

    let kernel = get_structuring_element(MORPH_CROSS, size, anchor).unwrap();
    if kind == 0 {
        dilate(&img, &mut target, &kernel, anchor, 5, BORDER_CONSTANT, border).unwrap();
    } else if kind == 1 {
        erode(&img, &mut target, &kernel, anchor, 5, BORDER_CONSTANT, border).unwrap();
    } else if kind == 2 {
        morphology_ex(&img, &mut target, MORPH_OPEN, &kernel, anchor, 5, BORDER_CONSTANT, border).unwrap();
    } else if kind == 3 {
        morphology_ex(&img, &mut target, MORPH_CLOSE, &kernel, anchor, 5, BORDER_CONSTANT, border).unwrap();
    }
    
    imshow("IMAGE", &target).unwrap();
    wait_key(1).unwrap();
    destroy_all_windows().unwrap();    
}