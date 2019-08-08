
use image::{   RgbImage, DynamicImage, GrayImage, Rgb};
use imageproc::map::map_pixels;

use std::path::Path;

use nalgebra::{Point3, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::{FirstPerson};
//use kiss3d::event::{Action, Key, WindowEvent};


fn threshold_diff(new_frame: &GrayImage, old_frame: &GrayImage ) -> RgbImage {
    let pixel_diffs:RgbImage = map_pixels(new_frame, |x, y, p| {
        let prior_pixel:i16 = old_frame.get_pixel(x,y)[0] as i16;
        let new_pixel:i16 = p[0] as i16;
        let delta = new_pixel.max(prior_pixel) - new_pixel.min(prior_pixel);
        //let green_val = new_pixel as u8;
        if delta > 15 {
            if new_pixel > prior_pixel {
                Rgb([255u8,0, 0])
            } else if prior_pixel > new_pixel {
                Rgb([0, 0, 255u8])
            } else {
                Rgb([255u8, 0, 255u8])
            }
        }
        else {  Rgb([0, 0, 0]) }
    });

    pixel_diffs
}

fn main() {

    let eye = Point3::new(0.1, 1.1, 2.5);
    let at = Point3::origin();
    let mut camera = FirstPerson::new(eye, at);
    let light_pos = Point3::new(-2.0f32, 3.0f32, 2.0f32);
    let x_translate_step = Translation3::new(0.01, 0.0, 0.0);

    let mut window = Window::new_with_size("play", 640,480);
    window.set_light( Light::Absolute(light_pos));

    let mut cube1      = window.add_cube(1.0,  1.0, 1.0);
    cube1.set_color(1.0, 1.0, 1.0);

    let mut cube2      = window.add_cube(1.0, 1.0, 1.0);
    cube2.set_local_translation(Translation3::new(1.0, 0.0, -2.0));
    cube2.set_color(1.0, 1.0, 1.0);

    let mut cube3      = window.add_cube(1.0, 1.0, 1.0);
    cube3.set_local_translation(Translation3::new(2.0, 0.0, -4.0));
    cube3.set_color(1.0, 1.0, 1.0);

    //throw away garbage renders when context is first opened
    for _i in 0..5 {
        window.render_with_camera(&mut camera);
    }

    let mut prior_frame_opt:Option<GrayImage> = None;
    for i in 0..100 {
        if window.render_with_camera(&mut camera) {
            let img:DynamicImage  =  image::DynamicImage::ImageRgb8(window.snap_image());
            let lum_img = img.to_luma();

            if let Some(prior_frame) = prior_frame_opt {
                let pixel_diffs = threshold_diff(&lum_img, &prior_frame);

                let img_name = format!("./out/diff_{}.png", i);
                let img_path = Path::new(&img_name);
                pixel_diffs.save(img_path).unwrap();
            }
            prior_frame_opt = Some(lum_img);

//            let img_name = format!("./out/frame_{}.png", i);
//            let img_path = Path::new(&img_name);
//            img.save(img_path).unwrap();
        }

        camera.translate_mut( &x_translate_step);
    }

//    while window.render_with_camera(&mut camera)  {
//        for event in window.events().iter() {
//            match event.value {
//                WindowEvent::Key(Key::Left, Action::Release, _) => {
//                    camera.translate_mut( &translate_x_minus);
//                }
//                WindowEvent::Key(Key::Right, Action::Release, _) => {
//                    camera.translate_mut( &translate_x_plus);
//                }
//                WindowEvent::Key(Key::Q, Action::Release, _) => {
//                    return;
//                }
//                _ => {}
//            }
//        }
//    }
}




