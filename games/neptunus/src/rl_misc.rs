use raylib::math::{Vector2, Vector3, Vector4, Rectangle};

#[inline]
pub fn vec2<T: Into<f32>>(x: T, y: T) -> Vector2 {
    Vector2 {
        x: x.into(),
        y: y.into()
    }
}

pub fn vec3<T: Into<f32>>(x: T, y: T, z: T) -> Vector3 {
    Vector3 {
        x: x.into(),
        y: y.into(),
        z: z.into()
    }
}

pub fn vec4<T: Into<f32>>(x: T, y: T, z: T, w: T) -> Vector4 {
    Vector4 {
        x: x.into(),
        y: y.into(),
        z: z.into(),
        w: w.into()
    }
}

pub fn rect<T: Into<f32>>(x: T, y: T, width: T, height: T) -> Rectangle {
    Rectangle {
        x: x.into(),
        y: y.into(),
        width: width.into(),
        height: height.into()
    }
}