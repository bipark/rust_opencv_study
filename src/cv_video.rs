use opencv::{
    core::*,
    highgui::*,
    videoio::*,
};

pub fn open_video() {
    let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
    loop {
		let mut frame = Mat::default();
		cam.read(&mut frame).unwrap();
        imshow("Video", &mut frame).unwrap();
		let key = wait_key(10).unwrap();
		if key > 0 && key != 255 {
			break;
		}
	}
}