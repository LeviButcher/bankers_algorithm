mod utils;
use utils::{add_tuple, subtract_tuple, sum_tuple};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Process {
    resources_allocated: (u32, u32, u32, u32),
    resource_max: (u32, u32, u32, u32),
}

impl Process {
    fn new(allocate: (u32, u32, u32, u32), max: (u32, u32, u32, u32)) -> Process {
        Process {
            resource_max: max,
            resources_allocated: allocate,
        }
    }

    fn resources_needed(&self) -> (u32, u32, u32, u32) {
        subtract_tuple(self.resource_max, self.resources_allocated)
    }
}

///
/// Returns the safe order to run the provided process
/// If no safe execution order exists, Err Result is returned
///
pub fn bankers_algorithm(
    available: (u32, u32, u32, u32),
    processes: Vec<Process>,
) -> Result<Vec<Process>, String> {
    recursive_bankers_algorithm(available, processes, Vec::new(), 0)
}

fn recursive_bankers_algorithm(
    available: (u32, u32, u32, u32),
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
    if sum_tuple(first_process.resources_needed()) < sum_tuple(available) {
        let available = add_tuple(available, first_process.resources_allocated);
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
        let available = (5, 7, 6, 8);

        let processes = vec![Process::new((0, 0, 0, 0), (4, 5, 7, 8))];

        let result = bankers_algorithm(available, processes.clone());
        assert_eq!(result.unwrap(), processes);
    }

    #[test]
    fn teller_scheduler_almond_data_should_return_correct_order() {
        let available = (1, 5, 2, 0);

        let processes = vec![
            Process::new((0, 0, 1, 2), (0, 0, 1, 2)),
            Process::new((1, 0, 0, 0), (1, 7, 5, 0)),
            Process::new((1, 3, 5, 4), (2, 3, 5, 6)),
            Process::new((0, 6, 3, 2), (0, 6, 5, 2)),
            Process::new((0, 0, 1, 4), (0, 6, 5, 6)),
        ];
        let expected_result: Result<Vec<Process>, String> = Result::Ok(vec![
            processes[0],
            processes[2],
            processes[3],
            processes[4],
            processes[1],
        ]);

        let result = bankers_algorithm(available, processes);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn bankers_algorithm_not_enough_resources_for_process_should_return_error_message() {
        let available = (2, 2, 2, 2);

        let processes = vec![
            Process::new((0, 1, 0, 0), (3, 3, 3, 3)),
            Process::new((2, 0, 0, 0), (4, 4, 4, 4)),
        ];
        let expected_message = "No safe state exists";
        let result = bankers_algorithm(available, processes).unwrap_err();

        assert_eq!(result.contains(expected_message), true);
    }

    #[test]
    fn bankers_algorithm_empty_process_list_should_return_empty_list() {
        let available = (2, 2, 2, 2);

        let result = bankers_algorithm(available, Vec::new());
        assert_eq!(result.unwrap().is_empty(), true);
    }
}
