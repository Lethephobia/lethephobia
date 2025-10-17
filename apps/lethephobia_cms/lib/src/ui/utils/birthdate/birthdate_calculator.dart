int calculateAgeFromBirthdate(DateTime birthdate, {DateTime? baseDate}) {
  final baseDateOrNow = baseDate ?? DateTime.now();
  final ageThatYear = baseDateOrNow.year - birthdate.year;
  if (baseDateOrNow.month < birthdate.month ||
      (baseDateOrNow.month == birthdate.month &&
          baseDateOrNow.day < birthdate.day)) {
    return ageThatYear - 1;
  }
  return ageThatYear;
}

DateTime calculateBirthdateFromAge(int age, {DateTime? baseDate}) {
  final baseDateOrNow = baseDate ?? DateTime.now();
  final birthdate = DateTime(baseDateOrNow.year - age, 1, 1);
  return birthdate;
}
