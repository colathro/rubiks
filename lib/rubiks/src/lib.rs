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
    /// (
    ///     self.u.t_l, self.u.m_l, self.u.b_l,
    ///     self.f.t_l, self.f.m_l, self.f.b_l,
    ///     self.d.t_l, self.d.m_l, self.d.b_l,
    ///     self.b.t_r, self.b.m_r, self.b.b_r,
    /// ) = (
    ///     self.b.t_r, self.b.m_r, self.b.b_r,
    ///     self.u.t_l, self.u.m_l, self.u.b_l,
    ///     self.f.t_l, self.f.m_l, self.f.b_l,
    ///     self.d.t_l, self.d.m_l, self.d.b_l,
    /// )
    pub fn l(&mut self) {
        self.l.rotate();

        (
            self.u.t_l, self.u.m_l, self.u.b_l, self.f.t_l, self.f.m_l, self.f.b_l, self.d.t_l,
            self.d.m_l, self.d.b_l, self.b.t_r, self.b.m_r, self.b.b_r,
        ) = (
            self.b.t_r, self.b.m_r, self.b.b_r, self.u.t_l, self.u.m_l, self.u.b_l, self.f.t_l,
            self.f.m_l, self.f.b_l, self.d.t_l, self.d.m_l, self.d.b_l,
        );
    }

    /// L'
    /// (
    ///     self.u.t_l, self.u.m_l, self.u.b_l,
    ///     self.f.t_l, self.f.m_l, self.f.b_l,
    ///     self.d.t_l, self.d.m_l, self.d.b_l,
    ///     self.b.t_r, self.b.m_r, self.b.b_r,
    /// ) = (
    ///     self.f.t_l, self.f.m_l, self.f.b_l,
    ///     self.d.t_l, self.d.m_l, self.d.b_l,
    ///     self.b.t_r, self.b.m_r, self.b.b_r,
    ///     self.u.t_l, self.u.m_l, self.u.b_l,
    /// );
    pub fn l_p(&mut self) {
        self.l.rotate_p();

        (
            self.u.t_l, self.u.m_l, self.u.b_l, self.f.t_l, self.f.m_l, self.f.b_l, self.d.t_l,
            self.d.m_l, self.d.b_l, self.b.t_r, self.b.m_r, self.b.b_r,
        ) = (
            self.f.t_l, self.f.m_l, self.f.b_l, self.d.t_l, self.d.m_l, self.d.b_l, self.b.t_r,
            self.b.m_r, self.b.b_r, self.u.t_l, self.u.m_l, self.u.b_l,
        );
    }

    /// F
    /// (
    ///     self.u.b_l, self.u.b_m, self.u.b_r,
    ///     self.l.t_r, self.l.m_r, self.l.b_r,
    ///     self.r.t_l, self.r.m_l, self.r.b_l,
    ///     self.d.t_l, self.d.t_m, self.d.t_r,
    /// ) = (
    ///     self.l.t_r, self.l.m_r, self.l.b_r,
    ///     self.d.t_l, self.d.t_m, self.d.t_r,
    ///     self.u.b_l, self.u.b_m, self.u.b_r,
    ///     self.r.t_l, self.r.m_l, self.r.b_l,
    /// );
    pub fn f(&mut self) {
        self.f.rotate();
        (
            self.u.b_l, self.u.b_m, self.u.b_r, self.l.t_r, self.l.m_r, self.l.b_r, self.r.t_l,
            self.r.m_l, self.r.b_l, self.d.t_l, self.d.t_m, self.d.t_r,
        ) = (
            self.l.t_r, self.l.m_r, self.l.b_r, self.d.t_l, self.d.t_m, self.d.t_r, self.u.b_l,
            self.u.b_m, self.u.b_r, self.r.t_l, self.r.m_l, self.r.b_l,
        );
    }

    /// F'
    /// (
    ///     self.u.b_l, self.u.b_m, self.u.b_r,
    ///     self.l.t_r, self.l.m_r, self.l.b_r,
    ///     self.r.t_l, self.r.m_l, self.r.b_l,
    ///     self.d.t_l, self.d.t_m, self.d.t_r,
    /// ) = (
    ///    self.r.t_l, self.r.m_l, self.r.b_l,
    ///    self.u.b_l, self.u.b_m, self.u.b_r,
    ///    self.d.t_l, self.d.t_m, self.d.t_r,
    ///    self.l.t_r, self.l.m_r, self.l.b_r,
    /// );
    pub fn f_p(&mut self) {
        self.f.rotate_p();
        (
            self.u.b_l, self.u.b_m, self.u.b_r, self.l.t_r, self.l.m_r, self.l.b_r, self.r.t_l,
            self.r.m_l, self.r.b_l, self.d.t_l, self.d.t_m, self.d.t_r,
        ) = (
            self.r.t_l, self.r.m_l, self.r.b_l, self.u.b_l, self.u.b_m, self.u.b_r, self.d.t_l,
            self.d.t_m, self.d.t_r, self.l.t_r, self.l.m_r, self.l.b_r,
        );
    }

    /// R
    pub fn r(&mut self) {}

    /// R'
    pub fn r_p(&mut self) {}

    /// B
    pub fn b(&mut self) {}

    /// B'
    pub fn b_p(&mut self) {}

    /// U
    pub fn u(&mut self) {}

    /// U'
    pub fn u_p(&mut self) {}

    /// D
    pub fn d(&mut self) {}

    /// D'
    pub fn d_p(&mut self) {}
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Orange,
    Green,
    Red,
    Blue,
    White,
    Yellow,
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

        // rotate l face clockwise
        cube1.l();

        // set cube 2 to expected state
        cube2.f.t_l = Color::White;
        cube2.f.m_l = Color::White;
        cube2.f.b_l = Color::White;

        cube2.u.t_l = Color::Blue;
        cube2.u.m_l = Color::Blue;
        cube2.u.b_l = Color::Blue;

        cube2.d.t_l = Color::Green;
        cube2.d.m_l = Color::Green;
        cube2.d.b_l = Color::Green;

        cube2.b.t_r = Color::Yellow;
        cube2.b.m_r = Color::Yellow;
        cube2.b.b_r = Color::Yellow;

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_l_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // rotate l face clockwise
        cube1.l_p();

        // set cube 2 to expected state
        cube2.f.t_l = Color::Yellow;
        cube2.f.m_l = Color::Yellow;
        cube2.f.b_l = Color::Yellow;

        cube2.u.t_l = Color::Green;
        cube2.u.m_l = Color::Green;
        cube2.u.b_l = Color::Green;

        cube2.d.t_l = Color::Blue;
        cube2.d.m_l = Color::Blue;
        cube2.d.b_l = Color::Blue;

        cube2.b.t_r = Color::White;
        cube2.b.m_r = Color::White;
        cube2.b.b_r = Color::White;

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_f_face_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // rotate l face clockwise
        cube1.f();

        // set cube 2 to expected state
        cube2.u.b_l = Color::Orange;
        cube2.u.b_m = Color::Orange;
        cube2.u.b_r = Color::Orange;

        cube2.r.t_l = Color::White;
        cube2.r.m_l = Color::White;
        cube2.r.b_l = Color::White;

        cube2.d.t_l = Color::Red;
        cube2.d.t_m = Color::Red;
        cube2.d.t_r = Color::Red;

        cube2.l.t_r = Color::Yellow;
        cube2.l.m_r = Color::Yellow;
        cube2.l.b_r = Color::Yellow;

        assert_eq!(cube1, cube2);
    }

    #[test]
    fn rotate_f_face_counter_clockwise() {
        let mut cube1 = Cube::new();
        let mut cube2 = Cube::new();

        // rotate l face clockwise
        cube1.f_p();

        // set cube 2 to expected state
        cube2.u.b_l = Color::Red;
        cube2.u.b_m = Color::Red;
        cube2.u.b_r = Color::Red;

        cube2.r.t_l = Color::Yellow;
        cube2.r.m_l = Color::Yellow;
        cube2.r.b_l = Color::Yellow;

        cube2.d.t_l = Color::Orange;
        cube2.d.t_m = Color::Orange;
        cube2.d.t_r = Color::Orange;

        cube2.l.t_r = Color::White;
        cube2.l.m_r = Color::White;
        cube2.l.b_r = Color::White;

        assert_eq!(cube1, cube2);
    }
}
