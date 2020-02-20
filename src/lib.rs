mod utils;
use utils::{add_vec, is_less_than_or_equal, subtract_vec};

#[derive(Debug, PartialEq, Clone)]
pub struct Process {
    resources_allocated: Vec<u32>,
    resource_max: Vec<u32>,
}

impl Process {
    #[warn(dead_code)]
    fn new(allocate: Vec<u32>, max: Vec<u32>) -> Process {
        Process {
            resource_max: max,
            resources_allocated: allocate,
        }
    }

    fn resources_needed(&self) -> Vec<u32> {
        subtract_vec(&self.resource_max, &self.resources_allocated)
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

    let first_process: Process = processes.remove(0);

    // Process has available resources, GO TIME
    if is_less_than_or_equal(&first_process.resources_needed(), &available) {
        let available = add_vec(&available, &first_process.resources_allocated);
        ran_processes.push(first_process);
        return recursive_bankers_algorithm(available, processes, ran_processes, 0);
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

        let processes = vec![Process::new(vec![0, 0, 0, 0], vec![4, 5, 7, 8])];

        let result = bankers_algorithm(available, processes.clone());
        assert_eq!(result.unwrap(), processes);
    }

    #[test]
    fn bankers_algorithm_almond_data_should_return_expected() {
        let available = vec![1, 5, 2, 0];

        let processes = vec![
            Process::new(vec![0, 0, 1, 2], vec![0, 0, 1, 2]),
            Process::new(vec![1, 0, 0, 0], vec![1, 7, 5, 0]),
            Process::new(vec![1, 3, 5, 4], vec![2, 3, 5, 6]),
            Process::new(vec![0, 6, 3, 2], vec![0, 6, 5, 2]),
            Process::new(vec![0, 0, 1, 4], vec![0, 6, 5, 6]),
        ];
        let expected_result: Result<Vec<Process>, String> = Result::Ok(vec![
            Process::new(vec![0, 0, 1, 2], vec![0, 0, 1, 2]),
            Process::new(vec![1, 3, 5, 4], vec![2, 3, 5, 6]),
            Process::new(vec![0, 6, 3, 2], vec![0, 6, 5, 2]),
            Process::new(vec![0, 0, 1, 4], vec![0, 6, 5, 6]),
            Process::new(vec![1, 0, 0, 0], vec![1, 7, 5, 0]),
        ]);

        let result = bankers_algorithm(available, processes);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn bankers_algorithm_not_enough_resources_for_process_should_return_error_message() {
        let available = vec![2, 2, 2, 2];

        let processes = vec![
            Process::new(vec![0, 1, 0, 0], vec![3, 3, 3, 3]),
            Process::new(vec![2, 0, 0, 0], vec![4, 4, 4, 4]),
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
            Process::new(vec![0, 1, 0], vec![7, 5, 3]),
            Process::new(vec![2, 0, 0], vec![3, 2, 2]),
            Process::new(vec![3, 0, 2], vec![9, 0, 2]),
            Process::new(vec![2, 1, 1], vec![2, 2, 2]),
            Process::new(vec![0, 0, 2], vec![4, 3, 3]),
        ];
        let expected_result: Result<Vec<Process>, String> = Result::Ok(vec![
            Process::new(vec![2, 0, 0], vec![3, 2, 2]),
            Process::new(vec![2, 1, 1], vec![2, 2, 2]),
            Process::new(vec![0, 0, 2], vec![4, 3, 3]),
            Process::new(vec![0, 1, 0], vec![7, 5, 3]),
            Process::new(vec![3, 0, 2], vec![9, 0, 2]),
        ]);

        let result = bankers_algorithm(available, processes);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn bankers_algorithm_really_large_resource_not_enough_of_others_should_fail() {
        let available = vec![20, 2, 2, 2];

        let processes = vec![Process::new(vec![0, 1, 0, 0], vec![2, 3, 3, 3])];
        let expected_message = "No safe state exists";
        let result = bankers_algorithm(available, processes).unwrap_err();

        assert_eq!(result.contains(expected_message), true);
    }
}
