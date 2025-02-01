pub(crate) fn normalize_day(day: &str) -> String {
  if day.len() == 1 {
    format!("0{}", day)
  } else {
    day.into()
  }
}
