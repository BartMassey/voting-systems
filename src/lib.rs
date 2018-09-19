// Copyright © 2018 Bart Massey

//! Rust implementations of various voting systems, with a
//! common interface.

/// Newtype for candidates. For convenience, the candidates
/// are assumed to be numbered contiguously starting at 0.
#[derive(Debug, PartialEq, Eq)]
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
            + 1
    }
}

/// A voting system is defined by its election function.
pub trait VotingSystem {
    /// Run an election. Returns `Some` ranking or `None`
    /// in case of an irresolvable tie.
    fn election(Poll) -> Option<Ranking>;
}

/// Plurality voting ranks candidates by
/// number of top ballot rankings.
pub struct PluralityVoting;

impl VotingSystem for PluralityVoting {
    fn election(poll: Poll) -> Option<Ranking> {
        let ncandidates = poll.ncandidates();
        let mut votes = vec![0; ncandidates];
        for r in poll.0 {
            if r.is_empty() {
                continue;
            }
            votes[r[0].0] += 1;
        }
        let mut ranking: Vec<usize> = (0..ncandidates).collect();
        ranking.sort_by_key(|c| -votes[*c]);
        if ranking.len() >= 2 && votes[ranking[1]] == votes[ranking[0]] {
            // It's a tie.
            return None;
        }
        Some(ranking.into_iter().map(Candidate).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_POLL: &[&[usize]] = &[&[2, 1, 0], &[0, 1, 2], &[0, 2, 1], &[1, 2], &[]];

    fn make_poll() -> Poll {
        Poll(
            TEST_POLL
                .iter()
                .map(|r| r.iter().map(|c| Candidate(*c)).collect())
                .collect(),
        )
    }

    #[test]
    fn try_plurality() {
        match PluralityVoting::election(make_poll()) {
            Some(result) => assert_eq!(result[0], Candidate(0)),
            None => panic!("election rejected all candidates"),
        }
    }
}
