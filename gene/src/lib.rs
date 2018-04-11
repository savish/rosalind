use Organism::*;

#[derive(Clone, Copy, Debug)]
pub enum Organism {
    DD,
    DR,
    RR,
}

impl Organism {
    /// Returns `true` if the Organism contains the dominant allele
    ///
    /// # Example
    ///
    /// ```
    /// use gene::Organism;
    /// let org = Organism::RR;
    /// assert_eq!(org.is_dominant(), false);
    /// ```
    pub fn is_dominant(&self) -> bool {
        match *self {
            DD => true,
            DR => true,
            RR => false,
        }
    }

    pub fn mate(&self, other: Organism) -> Vec<Organism> {
        match *self {
            DD => Organism::mate_dom_dom(other),
            DR => Organism::mate_dom_rec(other),
            RR => Organism::mate_rec_rec(other),
        }
    }

    fn mate_dom_dom(other: Organism) -> Vec<Organism> {
        match other {
            DD => vec![DD, DD, DD, DD],
            DR => vec![DD, DD, DR, DR],
            RR => vec![DR, DR, DR, DR],
        }
    }

    fn mate_dom_rec(other: Organism) -> Vec<Organism> {
        match other {
            DD => vec![DD, DD, DR, DR],
            DR => vec![DD, DR, DR, RR],
            RR => vec![DR, DR, RR, RR],
        }
    }

    fn mate_rec_rec(other: Organism) -> Vec<Organism> {
        match other {
            DD => vec![DR, DR, DR, DR],
            DR => vec![DD, DR, DR, RR],
            RR => vec![RR, RR, RR, RR],
        }
    }
}

/// Determine the percentage of the population with dominant genes
///
/// # Example
///
/// ```
/// use gene::{Organism, percent_dominant};
/// let parent = Organism::DR;
/// let population = parent.mate(Organism::DR);
/// assert_eq!(percent_dominant(&population), 0.75f64);
/// ```
pub fn percent_dominant(population: &[Organism]) -> f64 {
    let len: i32 = population.len() as i32;
    let dom_len: i32 = population
        .iter()
        .filter(|org| org.is_dominant())
        .collect::<Vec<_>>()
        .len() as i32;

    (dom_len as f64) / (len as f64)
}

#[derive(Copy, Clone, Debug)]
pub struct Population {
    dd: i32,
    dr: i32,
    rr: i32,
}

impl Population {
    pub fn new(dd: i32, dr: i32, rr: i32) -> Population {
        Population { dd, dr, rr }
    }

    /// Returns the size of the population
    ///
    /// # Example
    ///
    /// ```
    /// use gene::Population;
    /// let pop = Population::new(3, 4, 5);
    /// assert_eq!(pop.size(), 12);
    /// ```
    pub fn size(&self) -> i32 {
        (*self).dd + (*self).dr + (*self).rr
    }

    /// Probability of choosing an organism from a population
    ///
    /// # Example
    ///
    /// ```
    /// use gene::{Organism, Population};
    /// let pop = Population::new(3, 4, 5);
    /// assert_eq!(pop.choose_organism_probability(Organism::DD), 0.25f64);
    /// ```
    pub fn choose_organism_probability(&self, organism: Organism) -> f64 {
        match organism {
            DD => self.dd as f64 / self.size() as f64,
            DR => self.dr as f64 / self.size() as f64,
            RR => self.rr as f64 / self.size() as f64,
        }
    }
}
