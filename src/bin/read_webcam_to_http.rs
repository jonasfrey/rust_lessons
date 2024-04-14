// // cargo
// actix-web = "4.0"
// rscam = "0.5.1"
// tokio = { version = "1", features = ["full"] }
// sudo apt install libv4l-dev 

// test.html 
// <script>
// let f_update_img = async function(){

//     let o_img = await fetch('http://localhost:8080');
//     let o_blob = await o_img.blob();
//     let o = document.querySelector("img");
//     if(!o){
//     	o = document.createElement("img")
//     	document.body.appendChild(o)
//     }
//     o.src = URL.createObjectURL(o_blob);
//     return true
// }
//         console.log("update")

// while(true){
//         console.log("update")
//     await f_update_img()
// }
// </script>

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::Error;

fn capture_frame() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut camera = rscam::new("/dev/video0")?;
    camera.start(&rscam::Config {
        interval: (1, 30),  // 30 fps.
        // resolution: (640, 480),
        format: b"MJPG",
        ..Default::default()
    })?;

    let frame = camera.capture()?;
    Ok(frame.to_vec())
}


async fn image() -> impl Responder {
    match capture_frame() {
        Ok(frame) => HttpResponse::Ok().content_type("image/jpeg").body(frame),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
