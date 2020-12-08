pub type Pixel = i16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    x: Pixel,
    y: Pixel,
}

impl From<Pixel> for Vertex {
    fn from(p: Pixel) -> Self {
        Vertex { x: p, y: p }
    }
}

impl From<(Pixel, Pixel)> for Vertex {
    fn from((x, y): (Pixel, Pixel)) -> Self {
        Vertex { x, y }
    }
}

impl Vertex {
    pub fn x(&self) -> Pixel {
        self.x
    }

    pub fn y(&self) -> Pixel {
        self.y
    }

    pub fn shift<T>(&self, other: T) -> Self
    where Vertex: From<T> {
        let other = Vertex::from(other);
        (self.x() + other.x(), self.y() + other.y()).into()
    }
}
