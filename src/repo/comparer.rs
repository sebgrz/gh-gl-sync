#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommitDiff {
    LEFT(String),
    RIGHT(String),
    SAME(String),
}

pub fn compare_commits(
    commits_left: &mut Vec<String>,
    commits_right: &mut Vec<String>,
) -> Vec<CommitDiff> {
    commits_left.reverse();
    commits_right.reverse();

    if commits_left.len() <= commits_right.len() {
        return collect_commits_func(commits_left, commits_right, false);
    } else {
        return collect_commits_func(commits_right, commits_left, true);
    }
}

fn collect_commits_func(
    commits_left: &mut Vec<String>,
    commits_right: &mut Vec<String>,
    switch_sides: bool,
) -> Vec<CommitDiff> {
    let mut diffs: Vec<CommitDiff> = Vec::new();
    let common_size = commits_left.len();

    for i in 0..common_size {
        let sha_left = &commits_left[i];
        let sha_right = &commits_right[i];

        if sha_left == sha_right {
            diffs.push(CommitDiff::SAME(sha_left.to_string()));
        } else {
            if commits_left.contains(sha_right) {
                // OMIT
            } else {
                diffs.push(if switch_sides {
                    CommitDiff::LEFT(sha_right.to_string())
                } else {
                    CommitDiff::RIGHT(sha_right.to_string())
                });
            }

            if commits_right.contains(sha_left) {
                diffs.push(CommitDiff::SAME(sha_left.to_string()));
            } else {
                diffs.push(if switch_sides {
                    CommitDiff::RIGHT(sha_left.to_string())
                } else {
                    CommitDiff::LEFT(sha_left.to_string())
                });
            }
        }
    }

    if commits_right.len() > common_size {
        for i in common_size..commits_right.len() {
            let commit_right = &commits_right[i];
            if !commits_left.contains(commit_right) {
                diffs.push(if switch_sides {
                    CommitDiff::LEFT(commit_right.to_string())
                } else {
                    CommitDiff::RIGHT(commit_right.to_string())
                });
            }
        }
    }

    diffs.reverse();
    diffs
}

#[cfg(test)]
mod tests {
    use super::CommitDiff;
    use test_case::test_case;

    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec!["444".to_string(),"333".to_string(),"222".to_string(),"111".to_string()],
        vec![
            CommitDiff::RIGHT("444".to_string()),
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits and one RIGHT"
    )]
    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec!["333".to_string(),"222".to_string(),"111".to_string(),"444".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::SAME("111".to_string()),
            CommitDiff::RIGHT("444".to_string())
        ]; "should return list with 3 SAME commits and one RIGHT as root tree"
    )]
    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec!["555".to_string(), "333".to_string(),"222".to_string(),"444".to_string()],
        vec![
            CommitDiff::RIGHT("555".to_string()),
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::LEFT("111".to_string()),
            CommitDiff::RIGHT("444".to_string()),
        ]; "should return list with 3 SAME commits and others both RIGHT and LEFT"
    )]
    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            "333".to_string(),
            "222".to_string(),
            "111".to_string(),
            "444".to_string(),
            "555".to_string(),
        ],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::RIGHT("444".to_string()),
            CommitDiff::SAME("111".to_string()),
            CommitDiff::RIGHT("555".to_string()),
        ]; "should return list with 3 SAME commits and two RIGHT"
    )]
    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            "333".to_string(),
            "222".to_string(),
            "444".to_string(),
            "111".to_string(),
        ],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::RIGHT("444".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits and one RIGHT in the center"
    )]
    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            "333".to_string(),
            "222".to_string(),
            "555".to_string(),
            "444".to_string(),
            "111".to_string(),
        ],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::RIGHT("555".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::RIGHT("444".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits and two RIGHT in the center"
    )]
    #[test_case(
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            "666".to_string(),
            "333".to_string(),
            "555".to_string(),
            "222".to_string(),
            "444".to_string(),
            "111".to_string(),
        ],
        vec![
            CommitDiff::RIGHT("666".to_string()),
            CommitDiff::RIGHT("555".to_string()),
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::RIGHT("444".to_string()),
            CommitDiff::SAME("111".to_string()),
        ]; "should return list with 3 SAME commits and 3 RIGHT"
    )]
    fn test_smaller_list_of_left_commits(
        mut commits_left: Vec<String>,
        mut commits_right: Vec<String>,
        expected_result: Vec<CommitDiff>,
    ) {
        let result = super::compare_commits(&mut commits_left, &mut commits_right);
        println!("{:?}", result);
        assert_eq!(expected_result.len(), result.len());
        expected_result.iter().enumerate().for_each(|(i, er)| {
            assert_eq!(er, &result[i]);
        });
    }

    #[test_case(
        vec!["444".to_string(),"333".to_string(),"222".to_string(),"111".to_string()],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::LEFT("444".to_string()),
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits and one LEFT"
    )]
    #[test_case(
        vec!["333".to_string(),"222".to_string(),"111".to_string(),"444".to_string()],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::SAME("111".to_string()),
            CommitDiff::LEFT("444".to_string())
        ]; "should return list with 3 SAME commits and one LEFT as root tree"
    )]
    #[test_case(
        vec![
            "333".to_string(),
            "222".to_string(),
            "111".to_string(),
            "444".to_string(),
            "555".to_string(),
        ],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::LEFT("444".to_string()),
            CommitDiff::SAME("111".to_string()),
            CommitDiff::LEFT("555".to_string()),
        ]; "should return list with 3 SAME commits and two LEFT which one is the root of tree"
    )]
    #[test_case(
        vec![
            "333".to_string(),
            "222".to_string(),
            "444".to_string(),
            "111".to_string(),
        ],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::LEFT("444".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits and one LEFT in the center"
    )]
    #[test_case(
        vec![
            "333".to_string(),
            "222".to_string(),
            "555".to_string(),
            "444".to_string(),
            "111".to_string(),
        ],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::LEFT("555".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::LEFT("444".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits and two LEFT"
    )]
    #[test_case(
        vec![
            "666".to_string(),
            "333".to_string(),
            "555".to_string(),
            "222".to_string(),
            "444".to_string(),
            "111".to_string(),
        ],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::LEFT("666".to_string()),
            CommitDiff::LEFT("555".to_string()),
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::LEFT("444".to_string()),
            CommitDiff::SAME("111".to_string()),
        ]; "should return list with 3 SAME commits and 3 LEFT"
    )]
    fn test_smaller_list_of_right_commits(
        mut commits_left: Vec<String>,
        mut commits_right: Vec<String>,
        expected_result: Vec<CommitDiff>,
    ) {
        let result = super::compare_commits(&mut commits_left, &mut commits_right);
        assert_eq!(expected_result.len(), result.len());
        expected_result.iter().enumerate().for_each(|(i, er)| {
            assert_eq!(er, &result[i]);
        });
    }

    #[test_case(
        vec!["333".to_string(),"222".to_string(),"111".to_string()],
        vec!["333".to_string(), "222".to_string(), "111".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::SAME("111".to_string())
        ]; "should return list with 3 SAME commits"
    )]
    #[test_case(
        vec!["333".to_string(),"222".to_string(),"111".to_string()],
        vec!["444".to_string(), "333".to_string(), "222".to_string()],
        vec![
            CommitDiff::SAME("333".to_string()),
            CommitDiff::RIGHT("444".to_string()),
            CommitDiff::SAME("222".to_string()),
            CommitDiff::LEFT("111".to_string()),
        ]; "should return list with 4 all kinds of commits"
    )]
    fn test_same_size_of_commits_lists(
        mut commits_left: Vec<String>,
        mut commits_right: Vec<String>,
        expected_result: Vec<CommitDiff>,
    ) {
        let result = super::compare_commits(&mut commits_left, &mut commits_right);
        assert_eq!(expected_result.len(), result.len());

        expected_result.iter().enumerate().for_each(|(i, er)| {
            assert_eq!(er, &result[i]);
        });
    }
}
