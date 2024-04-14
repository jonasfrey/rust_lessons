use opencv::{
    prelude::*,
    videoio,
};
use std::process::Command;
use std::str;
// sudo apt-get install v4l-utils
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut available_cameras = Vec::new();
    for index in 0..10 {
        // Try to open the video capture device
        let mut cam = videoio::VideoCapture::new(index, videoio::CAP_ANY)?;
        println!("{:?}", cam);
        // Check if the camera was successfully opened
        if videoio::VideoCapture::is_opened(&cam)? {
            println!("Camera {} is available", index);
            available_cameras.push(index);
        } else {
            // Assuming cameras are indexed sequentially, break the loop when a camera cannot be opened
            break;
        }
    }
    
    if available_cameras.is_empty() {
        println!("No cameras available");
    } else {
        println!("Available cameras: {:?}", available_cameras);
    }

    let output = Command::new("v4l2-ctl")
    .args(&["--list-devices"])
    .output()
    .expect("failed to execute process");
    // Convert the output to a UTF-8 string
    let output_str = str::from_utf8(&output.stdout).unwrap();

    // Split the output by double newline, which separates devices in the `v4l2-ctl` output
    let devices: Vec<&str> = output_str.split("\n\n").collect();

    for (index, device) in devices.iter().enumerate() {
        // Each device entry has the format "device name\n\t/dev/videoX\n", so we split by newline
        // to get the name and path separately. Here we're just printing them.
        let lines: Vec<&str> = device.split('\n').collect();
        if !lines.is_empty() {
            println!("Device {}: {}", index, lines[0]); // Device name
            // Further processing can be done here to handle device paths, etc.
        }
    }
    
    Ok(())
}
