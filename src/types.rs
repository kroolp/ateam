use chrono::prelude::{DateTime, Utc};

pub struct Pr {
    pub title: String,
    pub url: String,
    pub last_commit_pushed_date: Option<DateTime<Utc>>,
    pub tests_result: i64,
    pub open_conversations: i64,
    pub num_approvals: i64,
    pub num_reviewers: i64,
    pub additions: i64,
    pub deletions: i64,
    pub based_on_main_branch: i64,
}

pub struct ScoredPr {
    pub pr: Pr,
    pub score: Score,
}

#[derive(Debug)]
pub struct Score {
    pub age: f64,
    pub tests_result: f64,
    pub open_conversations: f64,
    pub num_approvals: f64,
    pub num_reviewers: f64,
    pub additions: f64,
    pub deletions: f64,
    pub based_on_main_branch: f64,
}

fn age(date_time: Option<DateTime<Utc>>) -> i64 {
    match date_time {
        Some(date_time) => (Utc::now() - date_time).num_hours(),
        None => 0,
    }
}

impl Score {
    pub fn from_pr(required_approvals: u8, pr: &Pr) -> Score {
        Score {
            age: age(pr.last_commit_pushed_date) as f64 * 0.5,
            tests_result: (pr.tests_result - 1) as f64 * -200.0,
            open_conversations: pr.open_conversations as f64 * -30.0,
            num_approvals: (pr.num_approvals - required_approvals as i64) as f64 * -80.0,
            num_reviewers: (pr.num_reviewers - required_approvals as i64) as f64 * -50.0,
            additions: pr.additions as f64 * -0.5,
            deletions: pr.deletions as f64 * -0.1,
            based_on_main_branch: pr.based_on_main_branch as f64 * 200.0,
        }
    }

    pub fn total(&self) -> f64 {
        self.age
            + self.tests_result
            + self.open_conversations
            + self.num_approvals
            + self.num_reviewers
            + self.additions
            + self.deletions
            + self.based_on_main_branch
    }
}

impl std::fmt::Display for Pr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:60.60} {}",
            // "{} {} {:?} {} OC:{} Appr:{}/{} +{} -{}",
            self.title,
            self.url,
            // self.last_commit_pushed_date,
            // self.tests_result,
            // self.open_conversations,
            // self.num_approvals,
            // self.num_reviewers,
            // self.additions,
            // self.deletions,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_fmt_method() {
        let pr = Pr {
            title: "Some important changes".to_string(),
            url: "https://github.com/frisoft/ateam/pull/1".to_string(),
            last_commit_pushed_date: None,
            tests_result: 0,
            open_conversations: 0,
            num_approvals: 1,
            num_reviewers: 2,
            additions: 1000,
            deletions: 999,
            based_on_main_branch: 200,
        };

        assert_eq!(
            format!("{}", pr),
            "Some important changes                                       https://github.com/frisoft/ateam/pull/1"
        );
    }
}

// impl std::fmt::Display for ScoredPr {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let pr = &self.pr;
//         writeln!(
//             f,
//             "{} {} {:?} {} OC:{} Appr:{}/{} +{} -{} S:{}",
//             pr.title,
//             pr.url,
//             pr.last_commit_pushed_date,
//             pr.tests_result,
//             pr.open_conversations,
//             pr.num_approvals,
//             pr.num_reviewers,
//             pr.additions,
//             pr.deletions,
//             self.score.total(),
//         )
//     }
// }

// impl std::fmt::Debug for ScoredPr {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let pr = &self.pr;

//         writeln!(
//             f,
//             "===============================================================\n
//                 PR title: {:?}\n
//                 PR URL: {:?}\n
//                 Last commit pushed date {:?}\n
//                 Tests result {}\n
//                 Open conversations {}\n
//                 Approvals {}\n
//                 Reviewers {}\n
//                 PR additions: {:?}\n
//                 PR deletions: {:?}\n
//                 Score {:?}\n
//                 Score details {:?}",
//             pr.title,
//             pr.url,
//             pr.last_commit_pushed_date,
//             pr.tests_result,
//             pr.open_conversations,
//             pr.num_approvals,
//             pr.num_reviewers,
//             pr.additions,
//             pr.deletions,
//             self.score.total(),
//             self.score
//         )
//     }
// }
