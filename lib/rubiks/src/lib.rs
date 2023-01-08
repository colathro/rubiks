use rand::prelude::*;
use std::fmt;

/// Represents the Rubik's Cube and with mutating moves implmentations.
#[derive(Debug, PartialEq, Eq)]
pub struct Cube {
    /// Left Face.
    pub l: Face,

    /// Front Face.
    pub f: Face,

    /// Right Face.
    pub r: Face,

    /// Back Face.
    pub b: Face,

    /// Upper Face.
    pub u: Face,

    /// Lower Face.
    pub d: Face,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            l: Face::new(Color::Orange),
            f: Face::new(Color::Green),
            r: Face::new(Color::Red),
            b: Face::new(Color::Blue),
            u: Face::new(Color::White),
            d: Face::new(Color::Yellow),
        }
    }

    /// L
    // (
    //     self.u.b_l, self.u.m_l, self.u.t_l,
    //     self.f.t_l, self.f.m_l, self.f.b_l,
    //     self.d.t_l, self.d.m_l, self.d.b_l,
    //     self.b.b_r, self.b.m_r, self.b.t_r,
    // ) = (
    //     self.b.t_r, self.b.m_r, self.b.b_r,
    //     self.u.t_l, self.u.m_l, self.u.b_l,
    //     self.f.t_l, self.f.m_l, self.f.b_l,
    //     self.d.t_l, self.d.m_l, self.d.b_l,
    // );
    pub fn l(&mut self) {
        self.l.rotate();

        (
            self.u.b_l, self.u.m_l, self.u.t_l, self.f.t_l, self.f.m_l, self.f.b_l, self.d.t_l,
            self.d.m_l, self.d.b_l, self.b.b_r, self.b.m_r, self.b.t_r,
        ) = (
            self.b.t_r, self.b.m_r, self.b.b_r, self.u.t_l, self.u.m_l, self.u.b_l, self.f.t_l,
            self.f.m_l, self.f.b_l, self.d.t_l, self.d.m_l, self.d.b_l,
        );
    }

    /// L'
    // (
    //     self.u.t_l, self.u.m_l, self.u.b_l,
    //     self.f.t_l, self.f.m_l, self.f.b_l,
    //     self.d.b_l, self.d.m_l, self.d.t_l,
    //     self.b.b_r, self.b.m_r, self.b.t_r,
    // ) = (
    //     self.f.t_l, self.f.m_l, self.f.b_l,
    //     self.d.t_l, self.d.m_l, self.d.b_l,
    //     self.b.t_r, self.b.m_r, self.b.b_r,
    //     self.u.t_l, self.u.m_l, self.u.b_l,
    // );
    pub fn l_p(&mut self) {
        self.l.rotate_p();

        (
            self.u.t_l, self.u.m_l, self.u.b_l, self.f.t_l, self.f.m_l, self.f.b_l, self.d.b_l,
            self.d.m_l, self.d.t_l, self.b.b_r, self.b.m_r, self.b.t_r,
        ) = (
            self.f.t_l, self.f.m_l, self.f.b_l, self.d.t_l, self.d.m_l, self.d.b_l, self.b.t_r,
            self.b.m_r, self.b.b_r, self.u.t_l, self.u.m_l, self.u.b_l,
        );
    }

    /// F
    // (
    //     self.u.b_r, self.u.b_m, self.u.b_l,
    //     self.l.t_r, self.l.m_r, self.l.b_r,
    //     self.r.t_l, self.r.m_l, self.r.b_l,
    //     self.d.t_r, self.d.t_m, self.d.t_l,
    // ) = (
    //     self.l.t_r, self.l.m_r, self.l.b_r,
    //     self.d.t_l, self.d.t_m, self.d.t_r,
    //     self.u.b_l, self.u.b_m, self.u.b_r,
    //     self.r.t_l, self.r.m_l, self.r.b_l,
    // );
    pub fn f(&mut self) {
        self.f.rotate();
        (
            self.u.b_r, self.u.b_m, self.u.b_l, self.l.t_r, self.l.m_r, self.l.b_r, self.r.t_l,
            self.r.m_l, self.r.b_l, self.d.t_r, self.d.t_m, self.d.t_l,
        ) = (
            self.l.t_r, self.l.m_r, self.l.b_r, self.d.t_l, self.d.t_m, self.d.t_r, self.u.b_l,
            self.u.b_m, self.u.b_r, self.r.t_l, self.r.m_l, self.r.b_l,
        );
    }

    /// F'
    // (
    //     self.u.b_l, self.u.b_m, self.u.b_r,
    //     self.l.b_r, self.l.m_r, self.l.t_r,
    //     self.r.b_l, self.r.m_l, self.r.t_l,
    //     self.d.t_l, self.d.t_m, self.d.t_r,
    // ) = (
    //     self.r.t_l, self.r.m_l, self.r.b_l,
    //     self.u.b_l, self.u.b_m, self.u.b_r,
    //     self.d.t_l, self.d.t_m, self.d.t_r,
    //     self.l.t_r, self.l.m_r, self.l.b_r,
    // );
    pub fn f_p(&mut self) {
        self.f.rotate_p();
        (
            self.u.b_l, self.u.b_m, self.u.b_r, self.l.b_r, self.l.m_r, self.l.t_r, self.r.b_l,
            self.r.m_l, self.r.t_l, self.d.t_l, self.d.t_m, self.d.t_r,
        ) = (
            self.r.t_l, self.r.m_l, self.r.b_l, self.u.b_l, self.u.b_m, self.u.b_r, self.d.t_l,
            self.d.t_m, self.d.t_r, self.l.t_r, self.l.m_r, self.l.b_r,
        );
    }

    /// R
    // (
    //     self.u.t_r, self.u.m_r, self.u.b_r,
    //     self.b.t_l, self.b.m_l, self.b.b_l,
    //     self.d.t_r, self.d.m_r, self.d.b_r,
    //     self.f.t_r, self.f.m_r, self.f.b_r,
    // ) = (
    //     self.f.t_r, self.f.m_r, self.f.b_r,
    //     self.u.b_r, self.u.m_r, self.u.t_r,
    //     self.b.b_l, self.b.m_l, self.b.t_l,
    //     self.d.t_r, self.d.m_r, self.d.b_r,
    // );
    pub fn r(&mut self) {
        self.r.rotate();
        (
            self.u.t_r, self.u.m_r, self.u.b_r, self.b.t_l, self.b.m_l, self.b.b_l, self.d.t_r,
            self.d.m_r, self.d.b_r, self.f.t_r, self.f.m_r, self.f.b_r,
        ) = (
            self.f.t_r, self.f.m_r, self.f.b_r, self.u.b_r, self.u.m_r, self.u.t_r, self.b.b_l,
            self.b.m_l, self.b.t_l, self.d.t_r, self.d.m_r, self.d.b_r,
        );
    }

    /// R'
    // (
    //     self.u.t_r, self.u.m_r, self.u.b_r,
    //     self.b.t_l, self.b.m_l, self.b.b_l,
    //     self.d.t_r, self.d.m_r, self.d.b_r,
    //     self.f.t_r, self.f.m_r, self.f.b_r,
    // ) = (
    //     self.b.b_l, self.b.m_l, self.b.t_l,
    //     self.d.t_r, self.d.m_r, self.d.b_r,
    //     self.f.t_r, self.f.m_r, self.f.b_r,
    //     self.u.b_r, self.u.m_r, self.u.t_r,
    // );
    pub fn r_p(&mut self) {
        self.r.rotate_p();
        (
            self.u.t_r, self.u.m_r, self.u.b_r, self.b.b_l, self.b.m_l, self.b.t_l, self.d.t_r,
            self.d.m_r, self.d.b_r, self.f.t_r, self.f.m_r, self.f.b_r,
        ) = (
            self.b.b_l, self.b.m_l, self.b.t_l, self.d.t_r, self.d.m_r, self.d.b_r, self.f.t_r,
            self.f.m_r, self.f.b_r, self.u.t_r, self.u.m_r, self.u.b_r,
        );
    }

    /// B
    // (
    //     self.u.t_l, self.u.t_m, self.u.t_r,
    //     self.l.t_l, self.l.m_l, self.l.b_l,
    //     self.d.b_l, self.d.b_m, self.d.b_r,
    //     self.r.t_r, self.r.m_r, self.r.b_r,
    // ) = (
    //     self.r.t_r, self.r.m_r, self.r.b_r,
    //     self.u.t_r, self.u.t_m, self.u.t_l,
    //     self.l.t_l, self.l.m_l, self.l.b_l,
    //     self.d.b_r, self.d.b_m, self.d.b_l
    // )
    pub fn b(&mut self) {
        self.b.rotate();

        (
            self.u.t_l, self.u.t_m, self.u.t_r, self.l.t_l, self.l.m_l, self.l.b_l, self.d.b_l,
            self.d.b_m, self.d.b_r, self.r.t_r, self.r.m_r, self.r.b_r,
        ) = (
            self.r.t_r, self.r.m_r, self.r.b_r, self.u.t_r, self.u.t_m, self.u.t_l, self.l.t_l,
            self.l.m_l, self.l.b_l, self.d.b_r, self.d.b_m, self.d.b_l,
        )
    }

    /// B'
    // (
    //     self.u.t_l, self.u.t_m, self.u.t_r,
    //     self.l.t_l, self.l.m_l, self.l.b_l,
    //     self.d.b_l, self.d.b_m, self.d.b_r,
    //     self.r.t_r, self.r.m_r, self.r.b_r,
    // ) = (
    //     self.l.b_l, self.l.m_l, self.l.t_l,
    //     self.d.b_l, self.d.b_m, self.d.b_r,
    //     self.r.b_r, self.r.m_r, self.r.t_r,
    //     self.u.t_l, self.u.t_m, self.u.t_r,
    // )
    pub fn b_p(&mut self) {
        self.b.rotate_p();

        (
            self.u.t_l, self.u.t_m, self.u.t_r, self.l.t_l, self.l.m_l, self.l.b_l, self.d.b_l,
            self.d.b_m, self.d.b_r, self.r.t_r, self.r.m_r, self.r.b_r,
        ) = (
            self.l.b_l, self.l.m_l, self.l.t_l, self.d.b_l, self.d.b_m, self.d.b_r, self.r.b_r,
            self.r.m_r, self.r.t_r, self.u.t_l, self.u.t_m, self.u.t_r,
        )
    }

    /// U
    // (
    //     self.l.t_l, self.l.t_m, self.l.t_r,
    //     self.f.t_l, self.f.t_m, self.f.t_r,
    //     self.r.t_l, self.r.t_m, self.r.t_r,
    //     self.b.t_l, self.b.t_m, self.b.t_r,

    // ) = (
    //     self.f.t_l, self.f.t_m, self.f.t_r,
    //     self.r.t_l, self.r.t_m, self.r.t_r,
    //     self.b.t_l, self.b.t_m, self.b.t_r,
    //     self.l.t_l, self.l.t_m, self.l.t_r,
    // )
    pub fn u(&mut self) {
        self.u.rotate();

        (
            self.l.t_l, self.l.t_m, self.l.t_r, self.f.t_l, self.f.t_m, self.f.t_r, self.r.t_l,
            self.r.t_m, self.r.t_r, self.b.t_l, self.b.t_m, self.b.t_r,
        ) = (
            self.f.t_l, self.f.t_m, self.f.t_r, self.r.t_l, self.r.t_m, self.r.t_r, self.b.t_l,
            self.b.t_m, self.b.t_r, self.l.t_l, self.l.t_m, self.l.t_r,
        )
    }

    /// U'
    // (
    //     self.l.t_l, self.l.t_m, self.l.t_r,
    //     self.f.t_l, self.f.t_m, self.f.t_r,
    //     self.r.t_l, self.r.t_m, self.r.t_r,
    //     self.b.t_l, self.b.t_m, self.b.t_r,
    // ) = (
    //     self.b.t_l, self.b.t_m, self.b.t_r,
    //     self.l.t_l, self.l.t_m, self.l.t_r,
    //     self.f.t_l, self.f.t_m, self.f.t_r,
    //     self.r.t_l, self.r.t_m, self.r.t_r,
    // )
    pub fn u_p(&mut self) {
        self.u.rotate_p();

        (
            self.l.t_l, self.l.t_m, self.l.t_r, self.f.t_l, self.f.t_m, self.f.t_r, self.r.t_l,
            self.r.t_m, self.r.t_r, self.b.t_l, self.b.t_m, self.b.t_r,
        ) = (
            self.b.t_l, self.b.t_m, self.b.t_r, self.l.t_l, self.l.t_m, self.l.t_r, self.f.t_l,
            self.f.t_m, self.f.t_r, self.r.t_l, self.r.t_m, self.r.t_r,
        )
    }

    /// D
    // (
    //     self.f.b_l, self.f.b_m, self.f.b_r,
    //     self.r.b_l, self.r.b_m, self.r.b_r,
    //     self.b.b_l, self.b.b_m, self.b.b_r,
    //     self.l.b_l, self.l.b_m, self.l.b_r,
    // ) = (
    //     self.l.b_l, self.l.b_m, self.l.b_r,
    //     self.f.b_l, self.f.b_m, self.f.b_r,
    //     self.r.b_l, self.r.b_m, self.r.b_r,
    //     self.b.b_l, self.b.b_m, self.b.b_r,
    // )
    pub fn d(&mut self) {
        self.d.rotate();

        (
            self.f.b_l, self.f.b_m, self.f.b_r, self.r.b_l, self.r.b_m, self.r.b_r, self.b.b_l,
            self.b.b_m, self.b.b_r, self.l.b_l, self.l.b_m, self.l.b_r,
        ) = (
            self.l.b_l, self.l.b_m, self.l.b_r, self.f.b_l, self.f.b_m, self.f.b_r, self.r.b_l,
            self.r.b_m, self.r.b_r, self.b.b_l, self.b.b_m, self.b.b_r,
        )
    }

    /// D'
    // (
    //     self.f.b_l, self.f.b_m, self.f.b_r,
    //     self.r.b_l, self.r.b_m, self.r.b_r,
    //     self.b.b_l, self.b.b_m, self.b.b_r,
    //     self.l.b_l, self.l.b_m, self.l.b_r,
    // ) = (
    //     self.r.b_l, self.r.b_m, self.r.b_r,
    //     self.b.b_l, self.b.b_m, self.b.b_r,
    //     self.l.b_l, self.l.b_m, self.l.b_r,
    //     self.f.b_l, self.f.b_m, self.f.b_r,
    // )
    pub fn d_p(&mut self) {
        self.d.rotate_p();

        (
            self.f.b_l, self.f.b_m, self.f.b_r, self.r.b_l, self.r.b_m, self.r.b_r, self.b.b_l,
            self.b.b_m, self.b.b_r, self.l.b_l, self.l.b_m, self.l.b_r,
        ) = (
            self.r.b_l, self.r.b_m, self.r.b_r, self.b.b_l, self.b.b_m, self.b.b_r, self.l.b_l,
            self.l.b_m, self.l.b_r, self.f.b_l, self.f.b_m, self.f.b_r,
        )
    }

    /// Calls the given CubeTurn
    pub fn rotate(&mut self, turn: &CubeTurn) {
        match turn {
            CubeTurn::F => self.f(),
            CubeTurn::FP => self.f_p(),
            CubeTurn::L => self.l(),
            CubeTurn::LP => self.l_p(),
            CubeTurn::R => self.r(),
            CubeTurn::RP => self.r_p(),
            CubeTurn::B => self.b(),
            CubeTurn::BP => self.b_p(),
            CubeTurn::U => self.u(),
            CubeTurn::UP => self.u_p(),
            CubeTurn::D => self.d(),
            CubeTurn::DP => self.d_p(),
            CubeTurn::NIL => {}
        }
    }

    /// Calls the inverse of the passed CubeTurn
    pub fn rotate_p(&mut self, turn: &CubeTurn) {
        match turn {
            CubeTurn::F => self.f_p(),
            CubeTurn::FP => self.f(),
            CubeTurn::L => self.l_p(),
            CubeTurn::LP => self.l(),
            CubeTurn::R => self.r_p(),
            CubeTurn::RP => self.r(),
            CubeTurn::B => self.b_p(),
            CubeTurn::BP => self.b(),
            CubeTurn::U => self.u_p(),
            CubeTurn::UP => self.u(),
            CubeTurn::D => self.d_p(),
            CubeTurn::DP => self.d(),
            CubeTurn::NIL => {}
        }
    }

    /// Returns true if a valid cube state, false otherwise.
    pub fn validate_cube(&self) -> bool {
        true
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let l = format!("L: {}", self.l);
        let f = format!("F: {}", self.f);
        let r = format!("R: {}", self.r);
        let b = format!("B: {}", self.b);
        let u = format!("U: {}", self.u);
        let d = format!("D: {}", self.d);
        let text = format!("{l}{f}{r}{b}{u}{d}");
        write!(formatter, "{text}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Face {
    pub t_l: Color,
    pub t_m: Color,
    pub t_r: Color,
    pub m_l: Color,
    pub m_m: Color,
    pub m_r: Color,
    pub b_l: Color,
    pub b_m: Color,
    pub b_r: Color,
}

impl Face {
    /// Expensive initalizer for face of given color.
    pub fn new(color: Color) -> Face {
        Face {
            t_l: color.clone(),
            t_m: color.clone(),
            t_r: color.clone(),
            m_l: color.clone(),
            m_m: color.clone(),
            m_r: color.clone(),
            b_l: color.clone(),
            b_m: color.clone(),
            b_r: color.clone(),
        }
    }

    /// Rotates face clockwise.
    /// Does not mutate other faces.
    /// (
    ///     self.t_l, self.t_m, self.t_r,
    ///     self.m_l, self.m_m, self.m_r,
    ///     self.b_l, self.b_m, self.b_r,
    /// ) = (
    ///     self.b_l, self.m_l, self.t_l,
    ///     self.b_m, self.m_m, self.t_m,
    ///     self.b_r, self.m_r, self.t_r,
    /// );
    pub fn rotate(&mut self) {
        (
            self.t_l, self.t_m, self.t_r, self.m_l, self.m_m, self.m_r, self.b_l, self.b_m,
            self.b_r,
        ) = (
            self.b_l, self.m_l, self.t_l, self.b_m, self.m_m, self.t_m, self.b_r, self.m_r,
            self.t_r,
        );
    }

    /// ' (prime)
    /// Rotates face counter clockwise.
    /// Does not mutate other faces.
    /// (
    ///     self.t_l, self.t_m, self.t_r,
    ///     self.m_l, self.m_m, self.m_r,
    ///     self.b_l, self.b_m, self.b_r,
    /// ) = (
    ///    self.t_r, self.m_r, self.b_r,
    ///    self.t_m, self.m_m, self.b_m,
    ///    self.t_l, self.m_l, self.b_l,
    /// )
    pub fn rotate_p(&mut self) {
        (
            self.t_l, self.t_m, self.t_r, self.m_l, self.m_m, self.m_r, self.b_l, self.b_m,
            self.b_r,
        ) = (
            self.t_r, self.m_r, self.b_r, self.t_m, self.m_m, self.b_m, self.t_l, self.m_l,
            self.b_l,
        );
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let top_row = format!("{}{}{}", self.t_l, self.t_m, self.t_r);
        let mid_row = format!("{}{}{}", self.m_l, self.m_m, self.m_r);
        let bot_row = format!("{}{}{}", self.b_l, self.b_m, self.b_r);
        write!(
            f,
            "
{top_row}
{mid_row}
{bot_row}

"
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Orange,
    Green,
    Red,
    Blue,
    White,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Color::Orange => "O",
            Color::Green => "G",
            Color::Red => "R",
            Color::Blue => "B",
            Color::White => "W",
            Color::Yellow => "Y",
        };
        write!(f, "{text}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CubeTurn {
    F,
    FP,
    L,
    LP,
    R,
    RP,
    B,
    BP,
    U,
    UP,
    D,
    DP,
    NIL,
}

impl CubeTurn {
    pub fn generate_list() -> Vec<CubeTurn> {
        vec![
            CubeTurn::F,
            CubeTurn::FP,
            CubeTurn::L,
            CubeTurn::LP,
            CubeTurn::R,
            CubeTurn::RP,
            CubeTurn::B,
            CubeTurn::BP,
            CubeTurn::U,
            CubeTurn::UP,
            CubeTurn::D,
            CubeTurn::DP,
            CubeTurn::NIL,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // change color and rotate clockwise.
        cube1.f.t_l = Color::Blue;
        cube1.f.rotate();

        // manually rotate top right to expected state
        cube2.f.t_r = Color::Blue;

        assert_eq!(cube1.f, cube2.f);
    }

    #[test]
    fn rotate_face_clockwise_complete() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // change color and rotate clockwise 4 times should be identical to start
        cube1.f.t_l = Color::Blue;
        cube1.f.rotate();
        cube1.f.rotate();
        cube1.f.rotate();
        cube1.f.rotate();

        // manually rotate top right to expected state
        cube2.f.t_l = Color::Blue;

        assert_eq!(cube1.f, cube2.f);
    }

    #[test]
    fn rotate_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // change color and rotate clockwise.
        cube1.f.t_l = Color::Blue;
        cube1.f.rotate_p();

        // manually rotate top right to expected state
        cube2.f.b_l = Color::Blue;

        assert_eq!(cube1.f, cube2.f);
    }

    #[test]
    fn rotate_face_counter_clockwise_complete() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // change color and rotate clockwise.
        cube1.f.t_l = Color::Blue;
        cube1.f.rotate_p();
        cube1.f.rotate_p();
        cube1.f.rotate_p();
        cube1.f.rotate_p();

        // manually rotate top right to expected state
        cube2.f.t_l = Color::Blue;

        assert_eq!(cube1.f, cube2.f);
    }

    #[test]
    fn rotate_l_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.f.t_l = Color::Blue;
        cube1.f.m_l = Color::Orange;
        cube1.f.b_l = Color::Red;

        cube1.u.t_l = Color::White;
        cube1.u.m_l = Color::Green;
        cube1.u.b_l = Color::Red;

        cube1.d.t_l = Color::Orange;
        cube1.d.m_l = Color::Yellow;
        cube1.d.b_l = Color::Green;

        cube1.b.t_r = Color::Blue;
        cube1.b.m_r = Color::White;
        cube1.b.b_r = Color::Orange;

        // rotate l face clockwise
        cube1.l();

        // set cube 2 to expected state
        cube2.f.t_l = Color::White;
        cube2.f.m_l = Color::Green;
        cube2.f.b_l = Color::Red;

        cube2.u.t_l = Color::Orange;
        cube2.u.m_l = Color::White;
        cube2.u.b_l = Color::Blue;

        cube2.d.t_l = Color::Blue;
        cube2.d.m_l = Color::Orange;
        cube2.d.b_l = Color::Red;

        cube2.b.t_r = Color::Green;
        cube2.b.m_r = Color::Yellow;
        cube2.b.b_r = Color::Orange;

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_l_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.f.t_l = Color::Blue;
        cube1.f.m_l = Color::Orange;
        cube1.f.b_l = Color::Red;

        cube1.u.t_l = Color::White;
        cube1.u.m_l = Color::Green;
        cube1.u.b_l = Color::Red;

        cube1.d.t_l = Color::Orange;
        cube1.d.m_l = Color::Yellow;
        cube1.d.b_l = Color::Green;

        cube1.b.t_r = Color::Blue;
        cube1.b.m_r = Color::White;
        cube1.b.b_r = Color::Orange;

        // rotate l face clockwise
        cube1.l_p();

        // set cube 2 to expected state
        cube2.f.t_l = Color::Orange;
        cube2.f.m_l = Color::Yellow;
        cube2.f.b_l = Color::Green;

        cube2.u.t_l = Color::Blue;
        cube2.u.m_l = Color::Orange;
        cube2.u.b_l = Color::Red;

        cube2.d.t_l = Color::Orange;
        cube2.d.m_l = Color::White;
        cube2.d.b_l = Color::Blue;

        cube2.b.t_r = Color::Red;
        cube2.b.m_r = Color::Green;
        cube2.b.b_r = Color::White;

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_f_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.u.b_l = Color::White;
        cube1.u.b_m = Color::Green;
        cube1.u.b_r = Color::Red;

        cube1.r.t_l = Color::Blue;
        cube1.r.m_l = Color::White;
        cube1.r.b_l = Color::Orange;

        cube1.d.t_l = Color::Orange;
        cube1.d.t_m = Color::Yellow;
        cube1.d.t_r = Color::Green;

        cube1.l.t_r = Color::Blue;
        cube1.l.m_r = Color::White;
        cube1.l.b_r = Color::Orange;

        // rotate l face clockwise
        cube1.f();

        // set cube 2 to expected state
        cube2.u.b_r = Color::Blue;
        cube2.u.b_m = Color::White;
        cube2.u.b_l = Color::Orange;

        cube2.r.t_l = Color::White;
        cube2.r.m_l = Color::Green;
        cube2.r.b_l = Color::Red;

        cube2.d.t_r = Color::Blue;
        cube2.d.t_m = Color::White;
        cube2.d.t_l = Color::Orange;

        cube2.l.t_r = Color::Orange;
        cube2.l.m_r = Color::Yellow;
        cube2.l.b_r = Color::Green;

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_f_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.u.b_l = Color::White;
        cube2.l.b_r = Color::White;

        cube1.u.b_m = Color::Green;
        cube2.l.m_r = Color::Green;

        cube1.u.b_r = Color::Red;
        cube2.l.t_r = Color::Red;

        cube1.r.t_l = Color::Blue;
        cube2.u.b_l = Color::Blue;

        cube1.r.m_l = Color::White;
        cube2.u.b_m = Color::White;

        cube1.r.b_l = Color::Orange;
        cube2.u.b_r = Color::Orange;

        cube1.d.t_l = Color::Orange;
        cube2.r.b_l = Color::Orange;

        cube1.d.t_m = Color::Yellow;
        cube2.r.m_l = Color::Yellow;

        cube1.d.t_r = Color::Green;
        cube2.r.t_l = Color::Green;

        cube1.l.t_r = Color::Blue;
        cube2.d.t_l = Color::Blue;

        cube1.l.m_r = Color::White;
        cube2.d.t_m = Color::White;

        cube1.l.b_r = Color::Orange;
        cube2.d.t_r = Color::Orange;

        // rotate f face counter clockwise
        cube1.f_p();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_r_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.u.t_r = Color::White;
        cube2.b.b_l = Color::White;

        cube1.u.m_r = Color::Green;
        cube2.b.m_l = Color::Green;

        cube1.u.b_r = Color::Red;
        cube2.b.t_l = Color::Red;

        cube1.b.t_l = Color::Blue;
        cube2.d.b_r = Color::Blue;

        cube1.b.m_l = Color::White;
        cube2.d.m_r = Color::White;

        cube1.b.b_l = Color::Orange;
        cube2.d.t_r = Color::Orange;

        cube1.d.t_r = Color::Orange;
        cube2.f.t_r = Color::Orange;

        cube1.d.m_r = Color::Yellow;
        cube2.f.m_r = Color::Yellow;

        cube1.d.b_r = Color::Green;
        cube2.f.b_r = Color::Green;

        cube1.f.t_r = Color::Blue;
        cube2.u.t_r = Color::Blue;

        cube1.f.m_r = Color::White;
        cube2.u.m_r = Color::White;

        cube1.f.b_r = Color::Orange;
        cube2.u.b_r = Color::Orange;

        // rotate r face clockwise
        cube1.r();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_r_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.u.t_r = Color::White;
        cube2.f.t_r = Color::White;

        cube1.u.m_r = Color::Green;
        cube2.f.m_r = Color::Green;

        cube1.u.b_r = Color::Red;
        cube2.f.b_r = Color::Red;

        cube1.b.t_l = Color::Blue;
        cube2.u.b_r = Color::Blue;

        cube1.b.m_l = Color::White;
        cube2.u.m_r = Color::White;

        cube1.b.b_l = Color::Orange;
        cube2.u.t_r = Color::Orange;

        cube1.d.t_r = Color::Orange;
        cube2.b.b_l = Color::Orange;

        cube1.d.m_r = Color::Yellow;
        cube2.b.m_l = Color::Yellow;

        cube1.d.b_r = Color::Green;
        cube2.b.t_l = Color::Green;

        cube1.f.t_r = Color::Blue;
        cube2.d.t_r = Color::Blue;

        cube1.f.m_r = Color::White;
        cube2.d.m_r = Color::White;

        cube1.f.b_r = Color::Orange;
        cube2.d.b_r = Color::Orange;

        // rotate r face counter clockwise
        cube1.r_p();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_b_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.u.t_l = Color::White;
        cube2.l.b_l = Color::White;

        cube1.u.t_m = Color::Green;
        cube2.l.m_l = Color::Green;

        cube1.u.t_r = Color::Red;
        cube2.l.t_l = Color::Red;

        cube1.l.t_l = Color::Blue;
        cube2.d.b_l = Color::Blue;

        cube1.l.m_l = Color::White;
        cube2.d.b_m = Color::White;

        cube1.l.b_l = Color::Orange;
        cube2.d.b_r = Color::Orange;

        cube1.d.b_l = Color::Orange;
        cube2.r.b_r = Color::Orange;

        cube1.d.b_m = Color::Yellow;
        cube2.r.m_r = Color::Yellow;

        cube1.d.b_r = Color::Green;
        cube2.r.t_r = Color::Green;

        cube1.r.t_r = Color::Blue;
        cube2.u.t_l = Color::Blue;

        cube1.r.m_r = Color::White;
        cube2.u.t_m = Color::White;

        cube1.r.b_r = Color::Orange;
        cube2.u.t_r = Color::Orange;

        // rotate b face clockwise
        cube1.b();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_b_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.u.t_l = Color::White;
        cube2.r.t_r = Color::White;

        cube1.u.t_m = Color::Green;
        cube2.r.m_r = Color::Green;

        cube1.u.t_r = Color::Red;
        cube2.r.b_r = Color::Red;

        cube1.l.t_l = Color::Blue;
        cube2.u.t_r = Color::Blue;

        cube1.l.m_l = Color::White;
        cube2.u.t_m = Color::White;

        cube1.l.b_l = Color::Orange;
        cube2.u.t_l = Color::Orange;

        cube1.d.b_l = Color::Orange;
        cube2.l.t_l = Color::Orange;

        cube1.d.b_m = Color::Yellow;
        cube2.l.m_l = Color::Yellow;

        cube1.d.b_r = Color::Green;
        cube2.l.b_l = Color::Green;

        cube1.r.t_r = Color::Blue;
        cube2.d.b_r = Color::Blue;

        cube1.r.m_r = Color::White;
        cube2.d.b_m = Color::White;

        cube1.r.b_r = Color::Orange;
        cube2.d.b_l = Color::Orange;

        // rotate b face clockwise
        cube1.b_p();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_u_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.f.t_l = Color::White;
        cube2.l.t_l = Color::White;

        cube1.f.t_m = Color::Green;
        cube2.l.t_m = Color::Green;

        cube1.f.t_r = Color::Red;
        cube2.l.t_r = Color::Red;

        cube1.l.t_l = Color::Blue;
        cube2.b.t_l = Color::Blue;

        cube1.l.t_m = Color::White;
        cube2.b.t_m = Color::White;

        cube1.l.t_r = Color::Orange;
        cube2.b.t_r = Color::Orange;

        cube1.b.t_l = Color::Orange;
        cube2.r.t_l = Color::Orange;

        cube1.b.t_m = Color::Yellow;
        cube2.r.t_m = Color::Yellow;

        cube1.b.t_r = Color::Green;
        cube2.r.t_r = Color::Green;

        cube1.r.t_l = Color::Blue;
        cube2.f.t_l = Color::Blue;

        cube1.r.t_m = Color::White;
        cube2.f.t_m = Color::White;

        cube1.r.t_r = Color::Orange;
        cube2.f.t_r = Color::Orange;

        // rotate u face clockwise
        cube1.u();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_u_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.f.t_l = Color::White;
        cube2.r.t_l = Color::White;

        cube1.f.t_m = Color::Green;
        cube2.r.t_m = Color::Green;

        cube1.f.t_r = Color::Red;
        cube2.r.t_r = Color::Red;

        cube1.l.t_l = Color::Blue;
        cube2.f.t_l = Color::Blue;

        cube1.l.t_m = Color::White;
        cube2.f.t_m = Color::White;

        cube1.l.t_r = Color::Orange;
        cube2.f.t_r = Color::Orange;

        cube1.b.t_l = Color::Orange;
        cube2.l.t_l = Color::Orange;

        cube1.b.t_m = Color::Yellow;
        cube2.l.t_m = Color::Yellow;

        cube1.b.t_r = Color::Green;
        cube2.l.t_r = Color::Green;

        cube1.r.t_l = Color::Blue;
        cube2.b.t_l = Color::Blue;

        cube1.r.t_m = Color::White;
        cube2.b.t_m = Color::White;

        cube1.r.t_r = Color::Orange;
        cube2.b.t_r = Color::Orange;

        // rotate u face counter clockwise
        cube1.u_p();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_d_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.f.b_l = Color::White;
        cube2.r.b_l = Color::White;

        cube1.f.b_m = Color::Green;
        cube2.r.b_m = Color::Green;

        cube1.f.b_r = Color::Red;
        cube2.r.b_r = Color::Red;

        cube1.r.b_l = Color::Blue;
        cube2.b.b_l = Color::Blue;

        cube1.r.b_m = Color::White;
        cube2.b.b_m = Color::White;

        cube1.r.b_r = Color::Orange;
        cube2.b.b_r = Color::Orange;

        cube1.b.b_l = Color::Orange;
        cube2.l.b_l = Color::Orange;

        cube1.b.b_m = Color::Yellow;
        cube2.l.b_m = Color::Yellow;

        cube1.b.b_r = Color::Green;
        cube2.l.b_r = Color::Green;

        cube1.l.b_l = Color::Blue;
        cube2.f.b_l = Color::Blue;

        cube1.l.b_m = Color::White;
        cube2.f.b_m = Color::White;

        cube1.l.b_r = Color::Orange;
        cube2.f.b_r = Color::Orange;

        // rotate d face clockwise
        cube1.d();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_d_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.f.b_l = Color::White;
        cube2.l.b_l = Color::White;

        cube1.f.b_m = Color::Green;
        cube2.l.b_m = Color::Green;

        cube1.f.b_r = Color::Red;
        cube2.l.b_r = Color::Red;

        cube1.r.b_l = Color::Blue;
        cube2.f.b_l = Color::Blue;

        cube1.r.b_m = Color::White;
        cube2.f.b_m = Color::White;

        cube1.r.b_r = Color::Orange;
        cube2.f.b_r = Color::Orange;

        cube1.b.b_l = Color::Orange;
        cube2.r.b_l = Color::Orange;

        cube1.b.b_m = Color::Yellow;
        cube2.r.b_m = Color::Yellow;

        cube1.b.b_r = Color::Green;
        cube2.r.b_r = Color::Green;

        cube1.l.b_l = Color::Blue;
        cube2.b.b_l = Color::Blue;

        cube1.l.b_m = Color::White;
        cube2.b.b_m = Color::White;

        cube1.l.b_r = Color::Orange;
        cube2.b.b_r = Color::Orange;

        // rotate d face counter clockwise
        cube1.d_p();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn cycle_all_cycle_back() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        cube1.d();
        cube1.b();
        cube1.f();
        cube1.b();
        cube1.l();

        cube1.l_p();
        cube1.b_p();
        cube1.f_p();
        cube1.b_p();
        cube1.d_p();

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn cycle_all_heavy() {
        let mut rng = thread_rng();
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        let possible_moves = CubeTurn::generate_list();
        let mut moves: Vec<&CubeTurn> = vec![];

        for i in 0..10000 {
            let turn_opt = possible_moves.choose(&mut rng);
            if let Some(turn) = turn_opt {
                moves.push(turn);
            }
        }

        for j in moves.iter() {
            cube1.rotate(j);
        }

        for k in moves.iter().rev() {
            cube1.rotate_p(k);
        }

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn base_line_validation() {
        let mut cube1 = Cube::new();

        assert!(cube1.validate_cube())
    }

    #[test]
    fn too_many_colors_validation() {
        let mut cube1 = Cube::new();

        cube1.b.b_l = Color::Orange;

        assert!(!cube1.validate_cube())
    }
}
