// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::*;

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
