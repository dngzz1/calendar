# Calendar Event Width

## Problem

Given a vector of intervals (start and finish time) representing meetings to be displayed on google calendar, compute the width of the div to be displayed (which should be the reciprocal of `max_overlap`).

## Example

```rust
let meetings = vec![(1., 3.), (4., 6.), (5., 9.), (10., 12.)];
    let max_overlap = solve_max_overlap(&meetings);
    println!("Checking meetings {:?}...", meetings);
    for i in 0..meetings.len() {
        println!(
            "Meeting {:?} has max overlap of {}",
            meetings[i], max_overlap[i]
        );
    }
```

Terminal:

```terminal
❯ cargo run
   Compiling calendar v0.1.0 (/Users/dan5/Documents/coding/rust/calendar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/calendar`
Checking meetings [(1.0, 3.0), (4.0, 6.0), (5.0, 9.0), (10.0, 12.0)]...
Meeting (1.0, 3.0) has max overlap of 1
Meeting (4.0, 6.0) has max overlap of 2
Meeting (5.0, 9.0) has max overlap of 2
Meeting (10.0, 12.0) has max overlap of 1
```

## Algorithm

First create a set of breakpoints (a point in time, either the start of a meeting or the end of a meeting).

```rust
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
```

Do one pass to calculate the number of meetings at each breakpoint.

```rust
let stack_count = create_stack_count(&breakpoints);
assert_eq!(stack_count, vec![1, 0, 1, 2, 1, 0, 1, 0]);
```

Finally, for each meeting, take the appropriate slice of `stack_count` and take the maximum.

```rust
assert_eq!(max_overlap, vec![1, 2, 2, 1]);
```
