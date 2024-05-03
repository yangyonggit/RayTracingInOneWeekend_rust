use vectorlib::{Vector3, Color};
use image::{ImageBuffer, Rgba};

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }

    fn origin(&self) -> Vector3 {
        self.origin
    }

    fn direction(&self) -> Vector3 {
        self.direction
    }
}

pub fn write_color(pixel: &mut Rgba<u8>, color: Color) {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    *pixel = Rgba([r, g, b, 255]);
}

pub fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, &ray) > 0.0 {
        return Color::new_f64(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new_f64(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_f64(0.5, 0.7, 1.0) * t
}

pub fn ray_normal_color(ray: Ray) -> Color {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let normal_vec = (ray.at(t) - Vector3::new(0.,0.,-1.)).make_unit_vector();
        return Color::new_vector3((normal_vec + Vector3::new(1.0,1.0,1.0)) * 0.5);
    }
    let unit_direction = ray.direction.make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new_f64(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_f64(0.5, 0.7, 1.0) * t
}

pub fn ray_background_color(ray: Ray) -> Color {

    let unit_direction = ray.direction.make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new_f64(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_f64(0.5, 0.7, 1.0) * t
}

pub fn hit_sphere(center: Vector3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin;
    let a = r.direction().dot(r.direction());
    let b = -2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / (2.0*a);
    }
}

pub fn render_image(filename: String,ray_fn: fn(Ray) -> Color){
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as u32;


    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center  = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;


    // Or alternatively, define the type directly with generics
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let pixel_center = pixel00_loc + pixel_delta_u * (x as f64) + pixel_delta_v * (y as f64);
        let ray_direction = pixel_center - camera_center;

        let r = Ray::new(camera_center, ray_direction);

        let color = ray_fn(r);

        write_color(pixel, color);
    }

    imgbuf.save(filename).unwrap()
}

fn main() {

    render_image("red_sphere.png".to_string(),ray_color);
    render_image("normal_sphere.png".to_string(),ray_normal_color);
    render_image("blue_background.png".to_string(),ray_background_color);



}
