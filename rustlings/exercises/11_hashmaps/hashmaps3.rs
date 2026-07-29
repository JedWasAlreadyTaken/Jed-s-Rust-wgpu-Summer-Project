// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.
       let team_1 = scores.entry(team_1_name).or_default();
       team_1.goals_scored += team_1_score;
       team_1.goals_conceded += team_2_score;
       
       let team_2 = scores.entry(team_2_name).or_default();
       team_2.goals_scored += team_2_score;
       team_2.goals_conceded += team_1_score; 
    }


    scores
}

/*
What the problem was
The loop already parsed each line into team names and scores, but had a TODO
where the actual score-table population should go — nothing yet inserted or
updated any `TeamScores` in `scores`.

Why is this a problem?
Each team can appear on multiple lines (as team 1 in one match, team 2 in
another), so the table needs to either create a fresh `TeamScores` entry on a
team's first appearance or add to its existing one on later appearances — and
team 1's scored goals are team 2's conceded goals (and vice versa), so both
sides of the update have to happen together.

Why does `.entry(team_1_name).or_default()` (and the same for team_2) fix this?
`.entry(team_1_name).or_default()` gets a mutable reference to that team's
`TeamScores`, creating a zeroed one via `#[derive(Default)]` if it's the team's
first appearance, or reusing the existing one otherwise. Updating
`goals_scored`/`goals_conceded` in place through that reference, repeated for
both teams on every line, keeps the running totals correct. `.or_default()` is
`.or_insert()`'s cousin for types that implement `Default` — handy when the
"empty" value isn't a fixed literal like `1` but "whatever the type's zero
value is". The key type here is `&str` (not `String`), which works because the
keys borrow directly from the `results` string slice passed in — no need to
allocate new `String`s per team name, since `results` outlives the returned
`HashMap`.
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
