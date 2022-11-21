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

fn slice_index(
    meeting: &(f32, f32),
    breakpoints: &[(f32, Cap)],
    num_meetings: usize,
) -> (usize, usize) {
    let meeting_start = meeting.0;
    let meeting_end = meeting.1;
    let mut start_index = 0;
    while breakpoints[start_index].0 < meeting_start
        || (breakpoints[start_index].0 == meeting_start && breakpoints[start_index].1 == Cap::End)
    {
        start_index += 1;
    }
    let mut end_index = start_index;
    while end_index < 2 * num_meetings && breakpoints[end_index].0 < meeting_end {
        end_index += 1;
    }
    (start_index, end_index + 1)
}

// This function has O(n^2): O(n) for looping through meetings and O(n) for finding max_overlap of each meeting.
fn solve_max_overlap(meetings: &[(f32, f32)]) -> Vec<usize> {
    is_valid(meetings);
    let breakpoints = create_breakpoints(&meetings);
    let stack_count = create_stack_count(&breakpoints);
    let mut result = vec![];
    for meeting in meetings {
        let (start_index, end_index) = slice_index(&meeting, &breakpoints, meetings.len());
        let mut max_overlap = stack_count[start_index];
        for index in start_index..end_index {
            max_overlap = std::cmp::max(stack_count[index], max_overlap);
        }
        result.push(max_overlap);
    }
    result
}

fn is_valid(meetings: &[(f32, f32)]) {
    for meeting in meetings {
        assert!(meeting.0 < meeting.1);
    }
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
        let breakpoints = create_breakpoints(&meetings);
        assert_eq!(
            breakpoints,
            vec![
                (1., Cap::Start),
                (3., Cap::End),
                (4., Cap::Start),
                (5., Cap::Start),
                (6., Cap::End),
                (9., Cap::End),
                (10., Cap::Start),
                (12., Cap::End)
            ]
        );
        let stack_count = create_stack_count(&breakpoints);
        assert_eq!(stack_count, vec![1, 0, 1, 2, 1, 0, 1, 0]);
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 2, 2, 1]);
    }

    #[test]
    fn test_1313() {
        let meetings = vec![(1., 3.), (1., 3.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![2, 2]);
    }

    #[test]
    fn test_1335() {
        let meetings = vec![(1., 3.), (3., 5.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 1]);
    }

    #[test]
    fn test_123536() {
        let meetings = vec![(1., 2.), (3., 5.), (3., 6.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 2, 2]);
    }

    #[test]
    fn test_133536() {
        let meetings = vec![(1., 3.), (3., 5.), (3., 6.)];
        let breakpoints = create_breakpoints(&meetings);
        assert_eq!(
            breakpoints,
            vec![
                (1., Cap::Start),
                (3., Cap::End),
                (3., Cap::Start),
                (3., Cap::Start),
                (5., Cap::End),
                (6., Cap::End)
            ]
        );
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![1, 2, 2]);
    }

    #[test]
    fn test_3513() {
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
    fn test_1534() {
        let meetings = vec![(1., 5.), (3., 4.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![2, 2]);
    }

    #[test]
    fn test_1325() {
        let meetings = vec![(1., 3.), (2., 5.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![2, 2]);
    }

    #[test]
    fn test_19121367() {
        let meetings = vec![(1., 9.), (1., 2.), (1., 3.), (6., 7.)];
        let max_overlap = solve_max_overlap(&meetings);
        assert_eq!(max_overlap, vec![3, 3, 3, 2]);
    }

    #[test]
    #[should_panic]
    fn test_end_time_before_start_time() {
        let meetings = vec![(3., 1.), (5., 6.)];
        solve_max_overlap(&meetings);
    }

    #[test]
    #[should_panic]
    fn test_end_time_equal_start_time() {
        let meetings = vec![(1., 1.)];
        solve_max_overlap(&meetings);
    }
}
