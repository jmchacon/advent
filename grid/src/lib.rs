//! A grid library for use in AoC problems.
use std::cmp::Ordering;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
/// Location defines an x,y coordinate
pub struct Location(pub isize, pub isize);

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

/// An XxY grid of T
///
/// 0,0 is the upper left and projects rightward and down as coordinates advance.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Grid<T: Default + Clone> {
    g: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

/// GridIter is the iterator for Grid
pub struct GridIter<'a, T: Default + Clone> {
    grid: &'a Grid<T>,
    cur: Option<Location>,
}

impl<'a, T: Default + Clone> Iterator for GridIter<'a, T> {
    type Item = (Location, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let new = match self.cur.clone() {
            Some(mut c) => {
                c.0 += 1;
                if c.0 >= self.grid.width.try_into().unwrap() {
                    c.0 = 0;
                    c.1 += 1;
                }
                if c.1 >= self.grid.height.try_into().unwrap() {
                    None
                } else {
                    Some(c)
                }
            }
            None => Some(Location(0, 0)),
        };
        self.cur = new;
        if self.cur.is_none() {
            None
        } else {
            Some((
                self.cur.as_ref().unwrap().clone(),
                self.grid.get(self.cur.as_ref().unwrap()),
            ))
        }
    }
}

impl<'a, T: Default + Clone> IntoIterator for &'a Grid<T> {
    type Item = (Location, &'a T);
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Default + Clone> Grid<T> {
    /// Define a new grid of size XxY
    pub fn new(x: usize, y: usize) -> Self {
        Grid {
            g: vec![vec![T::default(); x]; y],
            width: x,
            height: y,
        }
    }

    /// `iter` gives a reference iterator to a Grid<T>
    pub fn iter(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid: self,
            cur: None,
        }
    }

    /// The grid width.
    /// NOTE: The grid is indexed from 0 so this is one past the last index.
    pub fn width(&self) -> usize {
        self.g[0].len()
    }

    /// The grid height.
    /// NOTE: The grid is indexed from 0 so this is one past the last index.
    pub fn height(&self) -> usize {
        self.g.len()
    }

    /// Replace the given Location with a new T
    pub fn add(&mut self, l: &Location, t: T) {
        self.g[l.1 as usize][l.0 as usize] = t
    }

    /// Return the T at the given Location
    pub fn get(&'a self, l: &Location) -> &'a T {
        &self.g[l.1 as usize][l.0 as usize]
    }

    /// Return the mutable T at the given Location
    pub fn get_mut(&'a mut self, l: &Location) -> &'a mut T {
        &mut self.g[l.1 as usize][l.0 as usize]
    }

    fn neighbors_impl(&'a self, l: &Location, all: bool) -> Vec<(Location, &T)> {
        let x = l.0;
        let y = l.1;
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
                n.push((Location(t.0, t.1), &self.g[y][x]));
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

/// Given a grid<T> print it out. This is not part of the main impl as it does put
/// additional constraints on T that may not be needed in all cases.
pub fn print_grid<T: Default + Clone + std::fmt::Debug + std::fmt::Display>(grid: &Grid<T>) {
    for g in grid {
        print!("{}", g.1);
        if usize::try_from(g.0 .0).unwrap() == grid.width() - 1 {
            println!();
        }
    }
    println!();
}
