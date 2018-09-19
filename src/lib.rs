// Copyright © 2018 Bart Massey

//! Rust implementations of various voting systems, with a
//! common interface.

/// Newtype for candidates. For convenience, the candidates
/// are assumed to be numbered contiguously starting at 0.
pub struct Candidate(usize);

/// Type for rankings, especially ballots.
pub type Ranking = Vec<Candidate>;

/// A poll is a collection of ballots.
pub struct Poll(Vec<Ranking>);

impl Poll {

    /// Find the number of candidates implicitly in the
    /// poll.  Specifically, return one more than the
    /// highest candidate number found.
    fn ncandidates(&self) -> usize {
        self.0
            .iter()
            .flat_map(|r| r.iter().map(|c| c.0))
            .max()
            .expect("empty poll")
    }
}

/// A voting system is defined by its election function.
pub trait VotingSystem {

    /// Run an election.
    fn election(&Poll) -> Ranking;
}

/// Plurality voting ranks candidates by
/// number of top ballot rankings.
struct PluralityVoting;

impl VotingSystem for PluralityVoting {

    fn election(poll: &Poll) -> Ranking {
        unimplemented!("count top rankings")
    }
}
