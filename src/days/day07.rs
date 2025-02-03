use crate::computer::Computer;
use itertools::Itertools;

pub(crate) fn run(input: String) {
    let amps = AmpSet::new(0, 4, input.clone());
    let max = amps.find_max_thruster_signal();
    println!("Part 1: {}", max);

    let amps = AmpSet::new(5, 9, input.clone());
    let max = amps.find_max_feedback_signal();
    println!("Part 2: {}", max);
}

struct AmpSet {
    phase_low: usize,
    phase_high: usize,
    program: String,
}

impl AmpSet {
    pub fn new(phase_low: usize, phase_high: usize, program: String) -> Self {
        Self {
            phase_low,
            phase_high,
            program,
        }
    }

    pub fn find_max_thruster_signal(&self) -> usize {
        let range = self.phase_low..=self.phase_high;
        let count = range.try_len().unwrap();
        range
            .into_iter()
            .permutations(count)
            .map(|p| self.get_thruster_signal_for_sequence(p))
            .max()
            .unwrap()
    }

    pub fn find_max_feedback_signal(&self) -> usize {
        let range = self.phase_low..=self.phase_high;
        let count = range.try_len().unwrap();
        range
            .into_iter()
            .permutations(count)
            .map(|p| self.get_feedback_signal_for_sequence(p))
            .max()
            .unwrap()
    }

    fn get_thruster_signal_for_sequence(&self, sequence: Vec<usize>) -> usize {
        let mut last_output = 0;
        for i in sequence {
            let mut computer = Computer::new(self.program.clone().into());
            computer.set_input(vec![i.try_into().unwrap(), last_output]);
            computer.run();
            let output = computer.get_output()[0];
            last_output = output;
        }

        last_output as usize
    }

    fn get_feedback_signal_for_sequence(&self, sequence: Vec<usize>) -> usize {
        let mut computers = vec![
            Computer::new(self.program.clone().into()),
            Computer::new(self.program.clone().into()),
            Computer::new(self.program.clone().into()),
            Computer::new(self.program.clone().into()),
            Computer::new(self.program.clone().into()),
        ];

        for (index, phase) in sequence.iter().enumerate() {
            let computer = computers.get_mut(index).unwrap();
            computer.set_yield_on_output(true);
            computer.push_input((*phase).try_into().unwrap());
        }

        let mut next_input = Some(0);
        let mut last_output = None;
        loop {
            for (index, computer) in computers.iter_mut().enumerate() {
                if let Some(next) = next_input {
                    computer.push_input(next);
                }
                next_input = None;
                computer.run();

                if computer.is_halted() && index == 4 {
                    if let Some(output) = computers.last().unwrap().get_output().first() {
                        return *output as usize;
                    } else {
                        return last_output.unwrap_or(0);
                    }
                } else if let Some(output) = computer.next_output() {
                    next_input = Some(output);
                    last_output = Some(output as usize);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::AmpSet;

    #[test]
    fn test_find_max_thruster_signal() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
        let amp_set = AmpSet::new(0, 4, program);
        let max = amp_set.find_max_thruster_signal();
        assert_eq!(max, 43210);
    }

    #[test]
    fn test_find_max_feedback_signal() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_string();
        let amp_set = AmpSet::new(5, 9, program);
        let max = amp_set.find_max_feedback_signal();
        assert_eq!(max, 139629729);
    }
}
