use crate::math;

pub type Color = math::vec3::Vec3;

impl Color {
    pub fn write_color(&self, output: &mut String) {
        let ir = (255.999 * self.x()) as i32;
        let ig = (255.999 * self.y()) as i32;
        let ib = (255.999 * self.z()) as i32;
        output.push_str(&format!("{} {} {}\n", ir, ig, ib));
    }
}
