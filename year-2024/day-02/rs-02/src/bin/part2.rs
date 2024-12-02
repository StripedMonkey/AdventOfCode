use rs_2024_02::*;

fn main() {
    let input = rs_2024_02::static_read("input1.txt");
    let data = parse(input);
    let ans = valid_reports(data.iter())
        .map(|report| println!("{:?}", report))
        .count();
    println!("{}", ans);
}

fn valid_reports<'a>(
    reports: impl Iterator<Item = &'a Vec<usize>>,
) -> impl Iterator<Item = &'a Vec<usize>> {
    reports.filter(|report| {
        for dir in [true, false] {
            let Err(idx) = valid_report(report, dir) else {
                return true;
            };
            let mut working_report = report.to_vec();
            working_report.remove(idx);
            if valid_report(&working_report, dir).is_ok() {
                return true;
            }
        }
        false
    })
}

pub fn valid_report(report: &[usize], increasing: bool) -> Result<bool, usize> {
    for (i, valid) in report
        .windows(2)
        .map(|w| is_valid(w[0], w[1], increasing))
        .enumerate()
    {
        if valid {
            continue;
        }
        // Either the current index, or the next index is invalid, to determine which:
        // If i and i + 2 is valid, then it's i+1 that's invalid
        let Some(j) = report.get(i + 2) else {
            return Err(i + 1);
        };
        if is_valid(report[i], *j, increasing) {
            return Err(i + 1);
        }
        return Err(i);
    }
    Ok(true)
}

fn is_valid(a: usize, b: usize, increasing: bool) -> bool {
    let res = (a > b) ^ increasing;
    match a.abs_diff(b) {
        1..=3 => res,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_dumb() {
        assert!(!is_valid(1, 1, true));
        assert!(!is_valid(1, 1, false));

        assert!(is_valid(1, 2, true));
        assert!(!is_valid(1, 2, false));

        assert!(is_valid(2, 1, false));
        assert!(!is_valid(1, 2, false));

        assert!(!is_valid(1, 5, true));
        assert!(!is_valid(1, 5, false));

        assert!(!is_valid(5, 1, true));
        assert!(!is_valid(5, 1, false));

        assert!(is_valid(91, 88, false));
    }

    #[test]
    fn first_test() {
        let input = rs_2024_02::static_read("example1.txt");
        let data = parse(input);
        let ans = data
            .iter()
            .filter(|report| {
                for dir in [true, false] {
                    if let Err(idx) = valid_report(report, dir) {
                        let mut working_report = (*report).clone();
                        working_report.remove(idx);
                        if valid_report(&working_report, dir).is_ok() {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
                false
            })
            .count();
        println!("{}", ans);
        assert!(ans == 4)
    }

    #[test]
    fn test_validity() {
        let test_cases = [(vec![2, 5, 7, 4], 3), (vec![2, 5, 7, 7, 4], 2)];
        for (report, expected) in test_cases.iter() {
            let res = valid_report(report, true);
            assert_eq!(res.is_err(), true, "Expected Error on {:?}", report);
            assert_eq!(res.unwrap_err(), *expected);
        }
    }
}
