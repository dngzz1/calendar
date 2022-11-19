//! Calculates the width of a Google calendar event.
//! The width should be the reciprocal of max_overlap.
//! Inspired by moinudin's comment on
//! https://stackoverflow.com/questions/4542892/possible-interview-question-how-to-find-all-overlapping-intervals

use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
enum Cap {
    End,
    Start,
}

fn main() {
    let meetings = vec![(1., 3.), (4., 6.), (5., 9.), (10., 12.)];
    let max_overlap = solve_max_overlap(&meetings);
    println!("Checking meetings {:?}...", meetings);
    for i in 0..meetings.len() {
        println!(
            "Meeting {:?} has max overlap of {}",
            meetings[i], max_overlap[i]
        );
    }
}

fn create_breakpoints(meetings: &[(f32, f32)]) -> Vec<(f32, Cap)> {
    let mut endpoints = vec![];
    for meeting in meetings {
        endpoints.push((meeting.0, Cap::Start));
        endpoints.push((meeting.1, Cap::End));
    }
    endpoints.sort_by(|a, b| match (a.0).partial_cmp(&b.0).unwrap() {
        Ordering::Equal => (a.1).partial_cmp(&b.1).unwrap(),
        other => other,
    });
    endpoints
}

fn create_stack_count(breakpoints: &[(f32, Cap)]) -> Vec<usize> {
    let mut count = vec![];
    let mut curr = 0;
    for breakpoint in breakpoints {
        if breakpoint.1 == Cap::Start {
            curr += 1;
        } else {
            curr -= 1;
        }
        count.push(curr);
    }
    count
}

// This function has O(n^2): O(n) for looping through meetings and O(n) for finding max_overlap of each meeting.
fn solve_max_overlap(meetings: &[(f32, f32)]) -> Vec<usize> {
    let breakpoints = create_breakpoints(&meetings);
    let stack_count = create_stack_count(&breakpoints);
    let mut result = vec![];
    for meeting in meetings {
        let meeting_start = meeting.0;
        let meeting_end = meeting.1;
        let mut start_index = 0;
        while breakpoints[start_index].0 < meeting_start
            || (breakpoints[start_index].0 == meeting_start
                && breakpoints[start_index].1 == Cap::End)
        {
            start_index += 1;
        }
        let mut index = start_index;
        let mut max_overlap = stack_count[index];
        while breakpoints[index].0 <= meeting_end && index < meetings.len() {
            max_overlap = std::cmp::max(stack_count[index], max_overlap);
            index += 1;
        }
        result.push(max_overlap);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_breakpoints() {
        let meetings = [(1.0, 2.0), (2.0, 3.0)];
        let breakpoints = create_breakpoints(&meetings);
        assert_eq!(
            breakpoints,
            vec![
                (1.0, Cap::Start),
                (2.0, Cap::End),
                (2.0, Cap::Start),
                (3.0, Cap::End)
            ]
        );
    }

    #[test]
    fn test_stack_count() {
        let breakpoints = vec![
            (1.0, Cap::Start),
            (2.0, Cap::End),
            (2.0, Cap::Start),
            (3.0, Cap::End),
        ];
        let count = create_stack_count(&breakpoints);
        assert_eq!(count, vec![1, 0, 1, 0]);
    }

    #[test]
    fn test_max_overlap_standard() {
        let meetings = vec![(1., 3.), (4., 6.), (5., 9.), (10., 12.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 2, 2, 1]);
    }

    #[test]
    fn test_max_overlap_consecutive() {
        let meetings = vec![(1., 3.), (3., 5.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 1]);
    }

    #[test]
    fn test_max_overlap_consecutive_reversed() {
        let meetings = vec![(3., 5.), (1., 3.)];
        let breakpoints = create_breakpoints(&meetings);
        assert_eq!(
            breakpoints,
            vec![
                (1., Cap::Start),
                (3., Cap::End),
                (3., Cap::Start),
                (5., Cap::End)
            ]
        );
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 1]);
    }

    #[test]
    fn test_max_overlap_nested() {
        let meetings = vec![(1., 5.), (3., 4.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![2, 2]);
    }

    #[test]
    fn test_max_overlap_staggered() {
        let meetings = vec![(1., 3.), (2., 5.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![2, 2]);
    }

    #[test]
    fn test_max_overlap_three_two() {
        let meetings = vec![(1., 10.), (1., 2.), (1., 3.), (8., 9.)];
        let breakpoints = create_breakpoints(&meetings);
        assert_eq!(
            breakpoints,
            vec![
                (1., Cap::Start),
                (1., Cap::Start),
                (1., Cap::Start),
                (2., Cap::End),
                (3., Cap::End),
                (8., Cap::Start),
                (9., Cap::End),
                (10., Cap::End)
            ]
        );
        let stack_count = create_stack_count(&breakpoints);
        assert_eq!(stack_count, vec![1, 2, 3, 2, 1, 2, 1, 0]);
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![3, 3, 3, 2]);
    }
}
