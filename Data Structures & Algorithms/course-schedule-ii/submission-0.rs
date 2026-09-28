use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let count = num_courses as usize;
        let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); count];
        let mut indegree = vec![0usize; count];
        for pair in prerequisites {
            let course = pair[0] as usize;
            let prerequisite = pair[1] as usize;
            outgoing[prerequisite].push(course);
            indegree[course] += 1;
        }
        let mut ready = VecDeque::new();
        for (course, &degree) in indegree.iter().enumerate() {
            if degree == 0 {
                ready.push_back(course);
            }
        }
        let mut order = Vec::with_capacity(count);
        while let Some(course) = ready.pop_front() {
            order.push(course as i32);
            for &dependent in &outgoing[course] {
                indegree[dependent] -= 1;
                if indegree[dependent] == 0 {
                    ready.push_back(dependent);
                }
            }
        }
        if order.len() == count {
            order
        } else {
            Vec::new()
        }
    }
}