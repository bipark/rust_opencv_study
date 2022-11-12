use opencv::{
    core::*,
    imgcodecs::*,
    imgproc::*,
};

pub fn my_canny() {
    let src_img = imread("webcam.png", IMREAD_GRAYSCALE).unwrap();
    let mut edge_img = src_img.clone();
    let result_find_edge = canny(&src_img, &mut edge_img, 100.0, 100.0, 3, false);
    match result_find_edge {
        Ok(_) => {
            imwrite("result.png", &edge_img, &Vector::new()).ok();
            println!("opencv canny finish!");
        },
        Err(code) => {
            panic!("code: {:?}", code);
        }
    };
}