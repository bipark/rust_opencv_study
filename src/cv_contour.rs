use opencv::{
    core::*,
    imgcodecs::*,
    imgproc::*,
    highgui::*, 
    types::*,
};

fn load_image(flag: i32) -> Mat {
    let img = imread("shapes.png", flag).unwrap();
    return img;
}

pub fn contour_image() {
    let src = load_image(IMREAD_ANYCOLOR);

    let mut grey = Mat::default();
    cvt_color(&src, &mut grey, COLOR_RGB2GRAY, 1).unwrap();

    // Threshold
    let mut target1 = Mat::default();
    threshold(&grey, &mut target1, 128.0, 255.0, THRESH_BINARY).unwrap();

    // bitwise_not
    let mut target2 = Mat::default();
    bitwise_not(&target1, &mut target2, &no_array()).unwrap();

    // find contours
    let offset = Point::new(0, 0);
    let mut contours = VectorOfMat::new();
    find_contours(&target2, &mut contours, RETR_CCOMP, CHAIN_APPROX_NONE, offset).unwrap();

    // draw contours
    let mut target4 = src.clone();
    let color = Scalar::new(0.0, 0.0, 255.0, 0.0);
    let hierarchy = Mat::default();
    draw_contours(&mut target4, &contours, 0, color, 2, 3, &hierarchy, 1, offset).unwrap();

    // draw
    imshow("IMAGE", &target4).unwrap();
    wait_key(1).unwrap();
    destroy_all_windows().unwrap();    

}