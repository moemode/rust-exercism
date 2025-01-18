use num_traits::Zero;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Ord + Zero + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        if a <= T::zero() || b <= T::zero() || c <= T::zero() {
            return None;
        }
        if a + b < c || a + c < b || b + c < a {
            return None;
        }
        Some(Triangle { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        return self.a == self.b && self.b == self.c;
    }

    pub fn is_scalene(&self) -> bool {
        return self.a != self.b && self.a != self.c && self.b != self.c 
    }

    pub fn is_isosceles(&self) -> bool {
        return !self.is_scalene()
    }
}
