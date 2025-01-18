use num_traits::Zero;

pub struct Triangle<T> (T, T, T);

impl<T: Ord + Zero + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        if a + b < c || a + c < b || b + c < a {
            return None;
        }
        // the previous condition implies: a > 0 && b > 0 && c > 0
        Some(Triangle(a, b, c))
    }

    pub fn is_equilateral(&self) -> bool {
        return self.0 == self.1 && self.1 == self.2;
    }

    pub fn is_scalene(&self) -> bool {
        return self.0 != self.1 && self.0 != self.2 && self.1 != self.2 
    }

    pub fn is_isosceles(&self) -> bool {
        return !self.is_scalene()
    }
}
