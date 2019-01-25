use std::fmt;
use std::ops::Mul;

// ///// //
// Types //
// ///// //

/// Represents a single allelle in a factor
///
/// In genetics, each factor is comprised of two allelles. If both allelles in a factor are
/// dominant, the organism is considered Homozygous Dominant for the factor. If they are both
/// recessive, the organism is Homozygous Recessive. Otherwise, it is Heterozygous.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allelle {
    /// Dominant allelle
    D,
    /// Recessive allelle
    R,
}

impl fmt::Display for Allelle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Allelle::D => write!(f, "D"),
            Allelle::R => write!(f, "R"),
        }
    }
}

/// Represents a organism
///
/// Specifically, this represents a particular genetic factor. However, in context, a factor is
/// owned by an organism which is easier to conceptualize.
#[derive(Clone, Copy, Debug)]
pub struct Organism(Allelle, Allelle);

impl Organism {
    /// Create a new organism
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::{Organism, Allelle};
    ///
    /// let org = Organism::new(Allelle::D, Allelle::R);
    /// assert_eq!(org.to_string(), "DR");
    /// ```
    pub fn new(a: Allelle, b: Allelle) -> Organism {
        Organism(a, b)
    }

    /// Create a new homozygous dominant organism
    pub fn homozygous_dominant() -> Organism {
        Organism(Allelle::D, Allelle::D)
    }

    /// Create a new homozygous recessive organism
    pub fn homozygous_recessive() -> Organism {
        Organism(Allelle::R, Allelle::R)
    }

    /// Create a new heterozygous organism
    pub fn heterozygous() -> Organism {
        Organism(Allelle::D, Allelle::R)
    }

    /// Create a list of all possible parent combinations
    pub fn parents() -> Vec<(Organism, Organism)> {
        let opt = [
            Organism::homozygous_dominant(),
            Organism::heterozygous(),
            Organism::homozygous_recessive(),
        ];

        // We use `collect` on the inner loop to prevent the 'laziness' of the inner map and return
        // a vector, as opposed to returning a closure which causes issues with the borrow-checker.
        // This is less efficient, but shouldn't matter given the size of the resulting vector.
        opt.iter()
            .flat_map(|x| opt.iter().map(|y| (*x, *y)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    /// Return `true` if the organism displays the dominant trait
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::{Organism, Allelle};
    ///
    /// let org = Organism::new(Allelle::D, Allelle::R);
    /// assert!(org.is_dominant());
    /// ```
    pub fn is_dominant(self) -> bool {
        self.0 == Allelle::D || self.1 == Allelle::D
    }

    /// Return `true` if the organism displays the recessive trait
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::{Organism, Allelle};
    ///
    /// let org = Organism::new(Allelle::R, Allelle::R);
    /// assert!(org.is_recessive());
    /// ```
    pub fn is_recessive(self) -> bool {
        self.0 == Allelle::R && self.1 == Allelle::R
    }

    /// Return the probability of a child possessing the dominant trait
    ///
    /// Given another organism, this returns the probability that a child produced by mating it
    /// with this organism will possess the dominant trait for a factor.
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::*;
    ///
    /// let pt1 = Organism::heterozygous();
    /// let pt2 = Organism::heterozygous();
    ///
    /// assert_eq!(pt1.has_dominant_child(pt2), 0.75);
    /// ```
    pub fn has_dominant_child(self, other: Organism) -> f64 {
        let children = self * other;
        let dominant = children
            .iter()
            .fold(0u32, |acc, ch| if ch.is_dominant() { acc + 1 } else { acc });
        f64::from(dominant) / children.len() as f64
    }
}

impl Mul for Organism {
    type Output = [Organism; 4];

    /// Generate a list of probable offspring from two parent organisms
    ///
    /// There are four possible allelle combinations from mating two parents, for each factor.
    /// Depending on the genetic makeup of the parents, the possible combinations may be duplicated
    /// across the children, indicating a higher possibility of that combination appearing in any
    /// given offspring.
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::*;
    ///
    /// let parents = (Organism::heterozygous(), Organism::heterozygous());
    /// let expected_children = [
    ///     Organism::homozygous_dominant(),
    ///     Organism::heterozygous(),
    ///     Organism::heterozygous(),
    ///     Organism::homozygous_recessive(),
    /// ];
    ///
    /// assert_eq!(parents.0 * parents.1, expected_children);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        [
            Organism::new(self.0, rhs.0),
            Organism::new(self.0, rhs.1),
            Organism::new(self.1, rhs.0),
            Organism::new(self.1, rhs.1),
        ]
    }
}

impl fmt::Display for Organism {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl PartialEq for Organism {
    /// Determines the equality of organisms
    ///
    /// The order of allelles in the organism doesn't matter for equality
    ///
    /// # Examples
    /// ```rust
    /// use rosalind::gene::*;
    /// let org1 = Organism::new(Allelle::D, Allelle::R);
    /// let org2 = Organism::new(Allelle::D, Allelle::R);
    /// let org3 = Organism::new(Allelle::R, Allelle::D);
    ///
    /// // Similar order
    /// assert_eq!(org1, org2);
    ///
    /// // Alternate order
    /// assert_eq!(org1, org3);
    /// ```
    fn eq(&self, other: &Organism) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

/// Represents the makeup of a population
///
/// A population will consist of a number of homozygous dominant, homozygous recessive and
/// heterozygous organisms.
#[derive(Debug, Clone, Copy)]
pub struct Population(u32, u32, u32);

impl Population {
    /// Creates a new population
    ///
    /// Each parameter represents the number of organisms of a specific type in the population.
    ///
    /// # Params
    /// - dd : homozygous dominant organisms
    /// - dr : heterozygous organisms
    /// - rr : homozygous recessive organisms
    pub fn new(dd: u32, dr: u32, rr: u32) -> Population {
        Population(dd, dr, rr)
    }

    /// Return the number of homozygous dominant organisms in the population
    pub fn count_homozygous_dominant(&self) -> u32 {
        self.0
    }

    /// Return the number of heterozygous organisms in the population
    pub fn count_heterozygous(&self) -> u32 {
        self.1
    }

    /// Return the number of homozygous recessive organisms in the population
    pub fn count_homozygous_recessive(&self) -> u32 {
        self.2
    }

    /// Return the size of this population
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::*;
    ///
    /// let pop = Population::new(3, 4, 5);
    /// assert_eq!(pop.size(), 12);
    /// ```
    pub fn size(&self) -> u32 {
        self.0 + self.1 + self.2
    }

    /// Return the probability of selecting a pair of parents from a population
    ///
    /// # Example
    /// ```rust
    /// use rosalind::gene::*;
    ///
    /// let pop = Population::new(2, 2, 2);
    /// let parents = (Organism::heterozygous(), Organism::heterozygous());
    ///
    /// assert_eq!(pop.select_parents(parents.0, parents.1), 1f64 / 15f64);
    /// ```
    pub fn select_parents(&self, p1: Organism, p2: Organism) -> f64 {
        let (prob_p1, new_pop) = self.select_organism(p1);
        let (prob_p2, _) = new_pop.select_organism(p2);
        prob_p1 * prob_p2
    }

    fn select_organism(&self, org: Organism) -> (f64, Population) {
        match &org.to_string()[..] {
            "DD" => (
                f64::from(self.count_homozygous_dominant()) / f64::from(self.size()),
                Population(
                    self.count_homozygous_dominant() - 1,
                    self.count_heterozygous(),
                    self.count_homozygous_recessive(),
                ),
            ),
            "DR" => (
                f64::from(self.count_heterozygous()) / f64::from(self.size()),
                Population(
                    self.count_homozygous_dominant(),
                    self.count_heterozygous() - 1,
                    self.count_homozygous_recessive(),
                ),
            ),
            "RR" => (
                f64::from(self.count_homozygous_recessive()) / f64::from(self.size()),
                Population(
                    self.count_homozygous_dominant(),
                    self.count_heterozygous(),
                    self.count_homozygous_recessive() - 1,
                ),
            ),
            _ => (0f64, *self),
        }
    }
}

// /// Determine the percentage of the population with dominant genes
// ///
// /// # Example
// ///
// /// ```
// /// use rosalind::gene::{Organism, percent_dominant};
// /// let parent = Organism::DR;
// /// let population = parent.mate(Organism::DR);
// /// assert_eq!(percent_dominant(&population), 0.75f64);
// /// ```
// pub fn percent_dominant(population: &[Organism]) -> f64 {
//     let len: i32 = population.len() as i32;
//     let dom_len: i32 = population
//         .iter()
//         .filter(|org| org.is_dominant())
//         .collect::<Vec<_>>()
//         .len() as i32;

//     (dom_len as f64) / (len as f64)
// }

// impl Population {

//     /// Probability of choosing an organism from a population
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// use rosalind::gene::{Organism, Population};
//     /// let pop = Population::new(3, 4, 5);
//     /// assert_eq!(pop.choose_organism_probability(Organism::DD), 0.25f64);
//     /// ```
//     pub fn choose_organism_probability(&self, organism: Organism) -> f64 {
//         match organism {
//             DD => self.dd as f64 / self.size() as f64,
//             DR => self.dr as f64 / self.size() as f64,
//             RR => self.rr as f64 / self.size() as f64,
//         }
//     }
// }
