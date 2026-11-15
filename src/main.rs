use crate::color::Color;
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
mod vec3;
mod color;
mod ray;

fn main() {
    const MAX_COLOR: u16 = 256;
    let img_width: u16 = 256;
    let img_height: u16 = 256;


    println!("P3\n{} {}\n{}", img_width, img_height, MAX_COLOR);
    for i in 0..img_height {
        for j in 0..img_width {
            eprint!("\rScanline remaining: {}", (img_height - j));
            let pixel_color = Color::new(
                i as f64 / (img_width - 1) as f64,
                j as f64 / (img_height - 1) as f64, 
                0.0_f64
            );
            
            println!("{}", pixel_color.write_color());
        }
    }
    eprint!("\rDone\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_default() {
        let v = Vec3::default();

        assert_eq!(v, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(6.0, 9.0, 12.0);

        assert_eq!(v / 3.0, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);

        assert_eq!(-v, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(2.0, 3.0, 6.0);

        assert_eq!(v.length_squared(), 49.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(2.0, 3.0, 6.0);

        assert_eq!(v.length(), 7.0);
    }

    #[test]
    fn test_dot_prod() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1.dot_prod(v2), Vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_cross_prod() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1.cross_prod(v2), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(0.0, 3.0, 4.0);

        let unit = Vec3::unit_vector(v);

        assert_eq!(unit, Vec3::new(0.0, 0.6, 0.8));
        assert!((unit.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(format!("{}", v), "(1, 2, 3)");
    }

    #[test]
    fn test_color_out() {
        let c = Color::new(1.0, 0.5, 0.0);
        
        assert_eq!(c.write_color(), "255 127 0");
    }


    #[test]
    fn test_ray_new() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), direction);
    }

    #[test]
    fn test_ray_origin() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin(), Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_ray_direction() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.direction(), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(
            ray.at(2.0),
            Point3::new(9.0, 12.0, 15.0)
        );
    }

    #[test]
    fn test_ray_at_zero() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.at(0.0), origin);
    }

    #[test]
    fn test_ray_at_negative_t() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(
            ray.at(-1.0),
            Point3::new(-3.0, -3.0, -3.0)
        );
    }
}
