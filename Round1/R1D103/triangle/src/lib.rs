enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
    Degenerate,
}

pub struct Triangle {
    triangle_type: TriangleType,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if !((sides[0] > 0) && (sides[1] > 0) && (sides[2] > 0)) {
            return None;
        }

        if !((sides[0] <= (sides[1] + sides[2]))
            && (sides[1] <= (sides[0] + sides[2]))
            && (sides[2] <= (sides[1] + sides[0])))
        {
            return None;
        }

        if (sides[0] == sides[1]) && (sides[1] == sides[2]) {
            Some(Triangle {
                triangle_type: TriangleType::Equilateral,
            })
        } else if (sides[0] == sides[1]) || (sides[2] == sides[1]) || (sides[2] == sides[0]) {
            Some(Triangle {
                triangle_type: TriangleType::Isosceles,
            })
        } else if (sides[0] == (sides[1] + sides[2]))
            || (sides[1] == (sides[0] + sides[2]))
            || (sides[2] == (sides[1] + sides[0]))
        {
            Some(Triangle {
                triangle_type: TriangleType::Degenerate,
            })
        } else {
            Some(Triangle {
                triangle_type: TriangleType::Scalene,
            })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        if let TriangleType::Equilateral = self.triangle_type {
            true
        } else {
            false
        }
    }

    pub fn is_scalene(&self) -> bool {
        if let TriangleType::Scalene = self.triangle_type {
            true
        } else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        if let TriangleType::Isosceles = self.triangle_type {
            true
        } else {
            false
        }
    }
}
