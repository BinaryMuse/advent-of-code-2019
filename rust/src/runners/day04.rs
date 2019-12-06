pub fn run(input: String, _args: &[String]) {
  let nums: Vec<u32> = input.split('-').map(|s| s.parse::<u32>()).collect::<Result<_, _>>().unwrap();
  let first = nums.first().unwrap();
  let last = nums.last().unwrap();
  let valid_count = (*first..=*last).filter(|num| {
    is_valid(format!("{}", num).as_str())
  }).count();
  println!("Part 1: {} valid passwords", valid_count);
}

fn is_valid(s: &str) -> bool {
  let mut found_double = false;
  let mut last_num = None;

  for byte in s.bytes() {
    // There has got to be a better way to do this
    let num = format!("{}", byte).parse::<u32>().unwrap();

    match last_num {
      Some(digit) => {
        last_num = Some(num);
        if num == digit { found_double = true };
        if num < digit { return false };
      },
      None => {
        last_num = Some(num);
      }
    }
  };

  found_double
}

#[test]
fn test_validity() {
  assert_eq!(is_valid("111111"), true);
  assert_eq!(is_valid("223450"), false);
  assert_eq!(is_valid("123789"), false);
}
