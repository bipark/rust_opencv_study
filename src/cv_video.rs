use opencv::{
    core::*,
    highgui::*,
    videoio::*,
};

pub fn open_video() {
    let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
    cam.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, 800.0).unwrap();
    cam.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, 600.0).unwrap();
    let opened = VideoCapture::is_opened(&cam).unwrap();
    if opened {
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
}

pub fn save_video() {
    let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
    cam.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, 640.0).unwrap();
    cam.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, 480.0).unwrap();

    let fourcc = VideoWriter::fourcc('D', 'I', 'V', 'X').unwrap();
    let mut writer = VideoWriter::new("result.avi", fourcc, 20.0, Size { width: (640), height: (480) }, true).unwrap();
    let capture_opened = VideoCapture::is_opened(&cam).unwrap();
    let writer_opened = VideoWriter::is_opened(&writer).unwrap();

    if capture_opened && writer_opened {
        loop {
            let mut frame = Mat::default();
            cam.read(&mut frame).unwrap();
            writer.write(&mut frame).unwrap();
    
            imshow("Video", &mut frame).unwrap();
            let key = wait_key(10).unwrap();
            if key > 0 && key != 255 {
                writer.release().unwrap();
                break;
            }
    
        }    
    } else {
        println!("Not open cam....");
    }
}