#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord(pub isize, pub isize);

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1).into()
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        (self.0 - rhs.0, self.1 - rhs.1).into()
    }
}

impl std::ops::Mul<Coord> for usize {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        let i = self as isize;
        Coord(rhs.0 * i, rhs.1 * i)
    }
}

impl std::ops::Mul<Coord> for isize {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord(rhs.0 * self, rhs.1 * self)
    }
}

impl std::ops::Mul<Coord> for i32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        let i = self as isize;
        Coord(rhs.0 * i, rhs.1 * i)
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
       (row, col).into()
    }
    pub fn row(&self) -> usize {
        self.0 as _
    }

    pub fn irow(&self) -> isize {
        self.0
    }
    //
    pub fn col(&self) -> usize {
        self.1 as _
    }

    pub fn icol(&self) -> isize {
        self.1
    }

    pub fn is_positive(&self) -> bool {
        self.0 >= 0 && self.1 >= 0
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Coord> + use<> {
        self.neighbors_with_step(1)
    }

    pub fn neighbors_with_step(&self, step: isize) -> impl Iterator<Item = Self> + use<> {
        [
            (self.irow() - step, self.icol()).into(),
            (self.irow() + step, self.icol()).into(),
            (self.irow(), self.icol() - step).into(),
            (self.irow(), self.icol() + step).into(),
        ]
        .into_iter()
    }

    pub fn manhattan(&self, other: &Self) -> usize {
        ((self.irow() - other.irow()).abs() + (self.icol() - other.icol()).abs()) as usize
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

impl From<(u64, u64)> for Coord {
    fn from(value: (u64, u64)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub(crate) fn turn_left(self) -> Self {
        match self {
            Dir::N => Self::W,
            Dir::S => Self::E,
            Dir::E => Self::N,
            Dir::W => Self::S,
        }
    }

    pub(crate) fn turn_right(self) -> Self {
        match self {
            Dir::N => Self::E,
            Dir::S => Self::W,
            Dir::E => Self::S,
            Dir::W => Self::N,
        }
    }

    pub(crate) fn movement(&self) -> Coord {
        match self {
            Dir::N => (-1, 0),
            Dir::S => (1, 0),
            Dir::E => (0, 1),
            Dir::W => (0, -1),
        }
        .into()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub dir: Dir,
    pub coord: Coord,
}

impl Pos {
    pub fn move_forward(&self) -> Self {
        let dp = self.dir.movement();

        let coord = self.coord + dp;

        Self {
            dir: self.dir,
            coord,
        }
    }

    pub fn turn_right(&self) -> Self {
        Self {
            coord: self.coord,
            dir: self.dir.turn_right(),
        }
    }

    pub fn turn_left(&self) -> Self {
        Self {
            coord: self.coord,
            dir: self.dir.turn_left(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn common_n_movement() {
        let c: super::Coord = (0, 0).into();
        let nbrs: std::collections::HashSet<super::Coord> =
            [(0, 2).into(), (0, -2).into(), (2, 0).into(), (-2, 0).into()]
                .into_iter()
                .collect();
        assert_eq!(
            c.neighbors_with_step(2)
                .collect::<std::collections::HashSet<_>>(),
            nbrs
        );
    }
}
