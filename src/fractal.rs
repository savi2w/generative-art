extern crate image;
extern crate num_complex;

pub fn generate() {
    let x_axis = 1800;
    let y_axis = 1800;

    let scale_x = 3.0 / x_axis as f32;
    let scale_y = 3.0 / y_axis as f32;

    let mut image_buffer = image::ImageBuffer::new(x_axis, y_axis);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([red, 0, blue]);
    }

    for x in 0..x_axis {
        for y in 0..y_axis {
            let cx = y as f32 * scale_x - 1.5;
            let cy = x as f32 * scale_y - 1.5;

            let r_complex = num_complex::Complex::new(-0.4, 0.6);
            let mut complex = num_complex::Complex::new(cx, cy);

            let mut index = 0;
            while index < 255 && complex.norm() <= 2.0 {
                complex = complex * complex + r_complex;
                index += 1;
            }

            let pixel = image_buffer.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;

            *pixel = image::Rgb([data[0], index as u8, data[2]]);
        }
    }

    image_buffer.save("art-book/fractal.png").unwrap();
}
