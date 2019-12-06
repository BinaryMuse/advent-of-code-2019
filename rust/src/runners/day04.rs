pub fn run(input: String, _args: &[String]) {
  let nums: Vec<u32> = input.split('-').map(|s| s.parse::<u32>()).collect::<Result<_, _>>().unwrap();
  let first = nums.first().unwrap();
  let last = nums.last().unwrap();
  let valid = (*first..=*last).filter(|num| {
    is_valid(&num.to_string(), false)
  });
  println!("Part 1: {} valid passwords", valid.count());

  let extra_valid = (*first..=*last).filter(|num| {
    is_valid(&num.to_string(), true)
  });
  println!("Part 2: {} valid passwords", extra_valid.count());
}

fn is_valid(s: &str, enforce_runs: bool) -> bool {
  let mut last_num = None;
  let mut runs: Vec<(u32, usize)> = vec![];

  for byte in s.bytes() {
    // There has got to be a better way to do this
    let num = format!("{}", byte).parse::<u32>().unwrap();

    match last_num {
      Some(digit) => {
        last_num = Some(num);
        if num < digit { return false };
        if num == digit {
          // same as last number; increment the run
          let run = runs.pop().unwrap();
          runs.push((run.0, run.1 + 1));
        } else {
          // different; add a new run
          runs.push((num, 1));
        }
      },
      None => {
        last_num = Some(num);
        runs.push((num, 1));
      }
    }
  };

  if enforce_runs {
    runs.iter().any(|(_, count)| *count == 2)
  } else {
    runs.iter().any(|(_, count)| *count >= 2)
  }
}

#[test]
fn test_validity() {
  assert_eq!(is_valid("111111", false), true);
  assert_eq!(is_valid("223450", false), false);
  assert_eq!(is_valid("123789", false), false);
}

#[test]
fn test_validity_pt_2() {
  assert_eq!(is_valid("112233", true), true);
  assert_eq!(is_valid("123444", true), false);
  assert_eq!(is_valid("111122", true), true);
}
