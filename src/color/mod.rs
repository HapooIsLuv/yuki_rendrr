use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(self) -> String{
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let int_r = (255.999 * r) as u32;
        let int_g = (255.999 * g) as u32;
        let int_b = (255.999 * b)  as u32;

        format!("{} {} {}", int_r, int_g, int_b)
    }
}
