mod utils;
use std::fmt;
use utils::{add_vec, is_less_than_or_equal, join_vec, subtract_vec};

#[derive(Debug, PartialEq, Clone)]
pub struct Process {
    id: u32,
    resources_allocated: Vec<u32>,
    resource_max: Vec<u32>,
    available: Vec<u32>,
}

impl Process {
    #[warn(dead_code)]
    pub fn new(id: u32, allocate: Vec<u32>, max: Vec<u32>, avail: Vec<u32>) -> Process {
        Process {
            id: id,
            resource_max: max,
            resources_allocated: allocate,
            available: avail,
        }
    }

    fn resources_needed(&self) -> Vec<u32> {
        subtract_vec(&self.resource_max, &self.resources_allocated)
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| id: {} | allocated: {} | max: {} | needed: {} | available: {}",
            self.id,
            join_vec(",", &self.resources_allocated),
            join_vec(",", &self.resource_max),
            join_vec(",", &self.resources_needed()),
            join_vec(",", &self.available),
        )
    }
}

///
/// Returns the safe order to run the provided process
/// If no safe execution order exists, Err Result is returned
///
pub fn bankers_algorithm(
    available: Vec<u32>,
    processes: Vec<Process>,
) -> Result<Vec<Process>, String> {
    recursive_bankers_algorithm(available, processes, Vec::new(), 0)
}

fn recursive_bankers_algorithm(
    available: Vec<u32>,
    mut processes: Vec<Process>,
    mut ran_processes: Vec<Process>,
    elements_checked: usize,
) -> Result<Vec<Process>, String> {
    if processes.is_empty() {
        return Ok(ran_processes);
    }
    if elements_checked >= processes.len() {
        return Err("No safe state exists".into());
    }

    let mut first_process: Process = processes.remove(0);

    // Process has available resources, GO TIME
    if is_less_than_or_equal(
        &first_process.resources_allocated,
        &first_process.resource_max,
    ) {
        if is_less_than_or_equal(&first_process.resources_needed(), &available) {
            let available = add_vec(&available, &first_process.resources_allocated);
            first_process.available = available.to_vec();
            ran_processes.push(first_process);
            return recursive_bankers_algorithm(available, processes, ran_processes, 0);
        }
    }

    // Try again
    processes.push(first_process);
    return recursive_bankers_algorithm(available, processes, ran_processes, elements_checked + 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bankers_algorithm_should_return_correct_order() {
        let available = vec![5, 7, 7, 8];

        let processes = vec![Process::new(
            0,
            vec![0, 0, 0, 0],
            vec![4, 5, 7, 8],
            available.to_vec(),
        )];

        let result = bankers_algorithm(available, processes.clone());
        assert_eq!(result.unwrap(), processes);
    }

    #[test]
    fn bankers_algorithm_almond_data_should_return_expected() {
        let available = vec![1, 5, 2, 0];

        let processes = vec![
            Process::new(0, vec![0, 0, 1, 2], vec![0, 0, 1, 2], available.to_vec()),
            Process::new(1, vec![1, 0, 0, 0], vec![1, 7, 5, 0], available.to_vec()),
            Process::new(2, vec![1, 3, 5, 4], vec![2, 3, 5, 6], available.to_vec()),
            Process::new(3, vec![0, 6, 3, 2], vec![0, 6, 5, 2], available.to_vec()),
            Process::new(4, vec![0, 0, 1, 4], vec![0, 6, 5, 6], available.to_vec()),
        ];
        let expected_result: Result<Vec<Process>, String> = Result::Ok(vec![
            Process::new(0, vec![0, 0, 1, 2], vec![0, 0, 1, 2], vec![1, 5, 3, 2]),
            Process::new(2, vec![1, 3, 5, 4], vec![2, 3, 5, 6], vec![2, 8, 8, 6]),
            Process::new(3, vec![0, 6, 3, 2], vec![0, 6, 5, 2], vec![2, 14, 11, 8]),
            Process::new(4, vec![0, 0, 1, 4], vec![0, 6, 5, 6], vec![2, 14, 12, 12]),
            Process::new(1, vec![1, 0, 0, 0], vec![1, 7, 5, 0], vec![3, 14, 12, 12]),
        ]);

        let result = bankers_algorithm(available, processes);

        assert_eq!(expected_result, result);
    }

    // #[test]
    fn bankers_algorithm_not_enough_resources_for_process_should_return_error_message() {
        let available = vec![2, 2, 2, 2];

        let processes = vec![
            Process::new(0, vec![0, 1, 0, 0], vec![3, 3, 3, 3], available.to_vec()),
            Process::new(1, vec![2, 0, 0, 0], vec![4, 4, 4, 4], available.to_vec()),
        ];
        let expected_message = "No safe state exists";
        let result = bankers_algorithm(available, processes).unwrap_err();

        assert_eq!(result.contains(expected_message), true);
    }

    #[test]
    fn bankers_algorithm_empty_process_list_should_return_empty_list() {
        let available = vec![2, 2, 2, 2];

        let result = bankers_algorithm(available, Vec::new());
        assert_eq!(result.unwrap().is_empty(), true);
    }

    #[test]
    fn bankers_algorithm_second_almond_data_should_return_expected() {
        let available = vec![3, 3, 2];

        let processes = vec![
            Process::new(0, vec![0, 1, 0], vec![7, 5, 3], available.to_vec()),
            Process::new(1, vec![2, 0, 0], vec![3, 2, 2], available.to_vec()),
            Process::new(2, vec![3, 0, 2], vec![9, 0, 2], available.to_vec()),
            Process::new(3, vec![2, 1, 1], vec![2, 2, 2], available.to_vec()),
            Process::new(4, vec![0, 0, 2], vec![4, 3, 3], available.to_vec()),
        ];
        let expected_result: Result<Vec<Process>, String> = Result::Ok(vec![
            Process::new(1, vec![2, 0, 0], vec![3, 2, 2], vec![5, 3, 2]),
            Process::new(3, vec![2, 1, 1], vec![2, 2, 2], vec![7, 4, 3]),
            Process::new(4, vec![0, 0, 2], vec![4, 3, 3], vec![7, 4, 5]),
            Process::new(0, vec![0, 1, 0], vec![7, 5, 3], vec![7, 5, 5]),
            Process::new(2, vec![3, 0, 2], vec![9, 0, 2], vec![10, 5, 7]),
        ]);

        let result = bankers_algorithm(available, processes);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn bankers_algorithm_really_large_resource_not_enough_of_others_should_fail() {
        let available = vec![20, 2, 2, 2];

        let processes = vec![Process::new(
            0,
            vec![0, 1, 0, 0],
            vec![2, 3, 3, 3],
            available.to_vec(),
        )];
        let expected_message = "No safe state exists";
        let result = bankers_algorithm(available, processes).unwrap_err();

        assert_eq!(result.contains(expected_message), true);
    }
}
