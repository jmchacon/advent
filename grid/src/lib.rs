//! A grid library for use in AoC problems.
use std::cmp::Ordering;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
/// Location defines an x,y coordinate
pub struct Location(usize, usize);

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want to sort by row, then column order.
        // If we derive we get column/row instead unless we reverse
        // the tuple.
        let o = self.1.cmp(&other.1);
        match o {
            std::cmp::Ordering::Less => o,
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            std::cmp::Ordering::Greater => o,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Location {
    /// distance computes the Manhattan distance between 2 `Location`'s
    pub fn distance(&self, other: &Location) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Grid<T: Default + Clone> {
    g: Vec<Vec<T>>,
}

impl<'a, T: Default + Clone> Grid<T> {
    /// Define a new grid of size XxY
    pub fn new(x: usize, y: usize) -> Self {
        Grid {
            g: vec![vec![T::default(); x]; y],
        }
    }

    /// The grid width.
    /// NOTE: The grid is indexed from 0 so this is once past the last index.
    pub fn width(&self) -> usize {
        self.g[0].len()
    }

    /// The grid height.
    /// NOTE: The grid is indexed from 0 so this is once past the last index.
    pub fn height(&self) -> usize {
        self.g.len()
    }

    /// Replace the given Location with a new T
    pub fn add(&mut self, l: &Location, t: T) {
        self.g[l.1][l.0] = t
    }

    /// Return the T at the given Location
    pub fn get(&'a self, l: &Location) -> &'a T {
        &self.g[l.1][l.0]
    }

    /// Return a mutable T from the given Location
    pub fn get_mut(&'a mut self, l: &Location) -> &'a mut T {
        &mut self.g[l.1][l.0]
    }

    fn neighbors_impl(&'a self, l: &Location, all: bool) -> Vec<(Location, &T)> {
        let x = l.0 as isize;
        let y = l.1 as isize;
        let mut n = Vec::new();
        let mut tests = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        if all {
            tests.push((x + 1, y + 1));
            tests.push((x + 1, y - 1));
            tests.push((x - 1, y + 1));
            tests.push((x - 1, y - 1));
        }
        for t in &tests {
            if t.0 >= 0 && t.1 >= 0 && t.0 < self.g[0].len() as isize && t.1 < self.g.len() as isize
            {
                let x = t.0 as usize;
                let y = t.1 as usize;
                n.push((Location(x, y), &self.g[y][x]));
            }
        }
        n
    }

    /// Return the standard 4 compass direction neighbors (N/S/E/W) accounting for grid edges.
    pub fn neighbors(&'a self, l: &Location) -> Vec<(Location, &T)> {
        self.neighbors_impl(l, false)
    }

    /// Return the standard 8 compass direction neighbors (N/S/E/W/NE/NW/SE/SW) accounting for grid edges.
    pub fn neighbors_all(&'a self, l: &Location) -> Vec<(Location, &T)> {
        self.neighbors_impl(l, true)
    }
}
