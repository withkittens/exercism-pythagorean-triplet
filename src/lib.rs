//! The solution for [Exercism] “Pythagorean triplet” problem.
//! 
//! Requires the nightly compiler where [`Iterator::flatten`] is stabilized. Tested on rustc
//! 1.29.0-nightly ([9fd3d7899] 2018-07-07).
//! 
//! # Description
//! 
//! A Pythagorean triplet is a set of three natural numbers, (_a_, _b_, _c_), for which,
//! ```none
//! a**2 + b**2 = c**2
//! ```
//! 
//! For example,
//! ```none
//! 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
//! ```
//! 
//! There exists exactly one Pythagorean triplet for which _a_ + _b_ + _c_ = 1000.
//! 
//! Find the product _a_ * _b_ * _c_.
//! 
//! # Terms used throughput the crate
//! 
//! **[Pythagorean triple]**:<br>
//! Three natural numbers, _a_, _b_, and _c_, such that _a_<sup>2</sup> + _b_<sup>2</sup> = 
//! _c_<sup>2</sup>. A well-known example is (3, 4, 5).
//! 
//! **Primitive Pythagorean triple**:<br>
//! A Pythagorean triple (_a_, _b_, _c_), such that gcd(_a_, _b_, _c_) = 1.
//! 
//! [`Iterator::flatten`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.flatten
//! [9fd3d7899]: https://github.com/rust-lang/rust/commit/9fd3d7899
//! [Exercism]: https://exercism.io/
//! [Pythagorean triple]: https://en.wikipedia.org/wiki/Pythagorean_triple

use std::collections::VecDeque;
use std::ops;

/// Tries to find a Pythagorean triple (_a_, _b_, _c_), such that _a_ + _b_ + _c_ = 1000.
/// 
/// Returns _abc_ if such triple is found, or [`None`] otherwise.
/// 
/// # Example
/// 
/// ```
/// # use pythagorean_triplet::find;
/// assert_eq!(find(), Some(31875000));
/// ```
/// 
/// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
pub fn find() -> Option<u32> {
    find_triple(1000).map(|t| t.0 * t.1 * t.2)
}

fn find_triple(sum: u32) -> Option<Triple> {
    let mut triples = PrimitiveTriples::new()
        .take_while(|t| t.sum() <= sum)
        .map(|t| Triples::from(t).take_while(|t1| t1.sum() <= sum))
        .flatten();

    triples.find(|t| t.sum() == sum)
}

/// Represents a Pythagorean triple.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triple(u32, u32, u32);

impl Triple {
    /// Creates a `Triple` representing a triple (_a_, _b_, _c_).
    /// 
    /// # Panics
    /// 
    /// Panics if _a_<sup>2</sup> + _b_<sup>2</sup> ≠ _c_<sup>2</sup>.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use pythagorean_triplet::Triple;
    /// let triple = Triple::new(5, 12, 13);
    /// ```
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        assert_eq!(c*c, a*a + b*b);
        Triple(a, b, c)
    }

    /// For a Pythagorean triple (_a_, _b_, _c_) returns _a_ + _b_ + _c_.
    ///
    /// # Example
    /// 
    /// ```
    /// # use pythagorean_triplet::Triple;
    /// let triple = Triple::new(5, 12, 13);
    /// assert_eq!(triple.sum(), 30);
    /// ```
    #[inline]
    pub fn sum(&self) -> u32 {
        self.0 + self.1 + self.2
    }
}

// Implements the multiplication of triple and integer.
// For example, (3, 4, 5) * 2 = (6, 8, 10).
impl ops::Mul<u32> for Triple {
    type Output = Self;
    
    fn mul(self, d: u32) -> Self::Output {
        Triple::new(self.0 * d, self.1 * d, self.2 * d)
    }
}

// Converts a tuple to Triple.
// (3, 4, 5).into() = Triplet::new(3, 4, 5).
impl From<(u32, u32, u32)> for Triple {
    fn from(t: (u32, u32, u32)) -> Self {
        Triple::new(t.0, t.1, t.2)
    }
}

/// An **infinite** iterator over triples derived from a primitive Pythagorean triple.
/// 
/// # Example
/// 
/// ```
/// # use pythagorean_triplet::{Triple, Triples};
/// let mut triples = Triples::from(Triple::new(3, 4, 5));
/// assert_eq!(triples.next(), Some((3, 4, 5).into()));
/// assert_eq!(triples.next(), Some((6, 8, 10).into()));
/// ```
#[derive(Debug)]
pub struct Triples {
    t: Triple,
    d: u32
}

// Creates an iterator over triples, given an initial Triplet.
impl From<Triple> for Triples {
    fn from(t: Triple) -> Self {
        Triples { t, d: 0 }
    }
}

impl Iterator for Triples {
    type Item = Triple;
    
    // Increments d, returns a new Triplet.
    fn next(&mut self) -> Option<Self::Item> {
        self.d += 1;
        Some(self.t * self.d)
    }
}

/// An **infinite** iterator over primitive Pythagorean triples.
/// 
/// Starting from some _parent_ primitive Pythagorean triple (_a_, _b_, _c_), the iterator derives
/// three _children_ triples (_a_<sub>1</sub>, _b_<sub>1</sub>, _c_<sub>1</sub>), (_a_<sub>2</sub>, 
/// _b_<sub>2</sub>, _c_<sub>2</sub>), and (_a_<sub>3</sub>, _b_<sub>3</sub>, _c_<sub>3</sub>) that
/// are also primitive, and queues them for subsequent iteration.
/// 
/// The iterator uses matrix multiplication and Berggren's matrices _A_, _B_, _C_ as follows:
/// 
/// <div style="padding-left: 25px">
/// 
/// ![](http://quicklatex.com/cache3/e0/ql_4ba658f0e3c8edf90bfb268d4abf7ae0_l3.png)
/// ![](http://quicklatex.com/cache3/17/ql_e84feadf6317fe9260acf4665cc02b17_l3.png)
/// ![](http://quicklatex.com/cache3/4b/ql_6174ada8f7dd42380bf2eec405178f4b_l3.png)
/// <br><span style="color: #999; font-size: smaller">— _Images are rendered and hosted by
/// [QuickLaTeX]_</span>
/// 
/// </div>
/// 
/// More information may be found on [Wikipedia].
/// 
/// [Wikipedia]: https://en.wikipedia.org/wiki/Formulas_for_generating_Pythagorean_triples#Pythagorean_triples_by_use_of_matrices_and_linear_transformations
/// [QuickLaTeX]: http://quicklatex.com/
#[derive(Debug)]
pub struct PrimitiveTriples {
    queue: VecDeque<Triple>
}

impl PrimitiveTriples {
    /// Creates a new iterator.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use pythagorean_triplet::{Triple, PrimitiveTriples};
    /// let mut triples = PrimitiveTriples::new();
    /// assert_eq!(triples.next(), Some((3, 4, 5).into()));
    /// assert_eq!(triples.next(), Some((5, 12, 13).into()));
    /// assert_eq!(triples.next(), Some((21, 20, 29).into()));
    /// assert_eq!(triples.next(), Some((15, 8, 17).into()));
    /// ```
    pub fn new() -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(Triple::new(3, 4, 5));
        PrimitiveTriples { queue }
    }
}

// Berggren's matrices
static A: [i8; 9] = [ 1, -2,  2,
                      2, -1,  2,
                      2, -2,  3 ];
static B: [i8; 9] = [ 1,  2,  2,
                      2,  1,  2, 
                      2,  2,  3 ];
static C: [i8; 9] = [-1,  2,  2,
                     -2,  1,  2,
                     -2,  2,  3 ];

// Multiplies a single matrix row and a column vector.
// Hides conversion casts.
macro_rules! mul {
    ( row($mat:ident, $row_idx:expr), $t:ident ) => ({
        const I: usize = 3 * $row_idx;
        let a = $mat[I  ] as i64 * $t.0 as i64;
        let b = $mat[I+1] as i64 * $t.1 as i64;
        let c = $mat[I+2] as i64 * $t.2 as i64;
        (a + b + c) as u32
    });
}

impl Iterator for PrimitiveTriples {
    type Item = Triple;
    
    // Pops the front triple, derives three children triples using the matrices above, and pushes
    // them to the queue.
    fn next(&mut self) -> Option<Self::Item> {
        fn mul_triple(t: Triple, mat: &[i8; 9]) -> Triple {
            Triple(
                mul!(row(mat, 0), t),
                mul!(row(mat, 1), t),
                mul!(row(mat, 2), t),
            )
        }

        let triple = self.queue.pop_front()?;
        
        self.queue.push_back(mul_triple(triple, &A));
        self.queue.push_back(mul_triple(triple, &B));
        self.queue.push_back(mul_triple(triple, &C));
        
        Some(triple)
    }
}