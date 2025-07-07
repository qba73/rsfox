pub fn print_intro() -> String {
    String::from("Reading Fox device status")
}

#[test]
fn print_status_for_energy_meter() {
    let result = print_intro();
    assert_eq!(result, "Reading Fox device status")
}
