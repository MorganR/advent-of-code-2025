#[derive(Debug, PartialEq)]
struct BatteryBankSelection {
    /// The indices of the batteries to turn on in the bank.
    batteries: Vec<usize>,
    /// The joltage of this bank from turning on these batteries.
    joltage: u32
}

impl BatteryBankSelection {
    fn new(batteries: Vec<usize>, joltage: u32) -> BatteryBankSelection {
        BatteryBankSelection { batteries, joltage }
    }
}

/// Finds the joltage for the battery bank, defined as the max two digits
fn select_max_joltage(battery_bank: &str) -> BatteryBankSelection {
    let mut best_first_index = 0;
    let mut best_next_index = 0;
    let mut best_first_value = '0';
    let mut best_next_value = '0';

    let last_index = battery_bank.len() - 1;
    for (i, c) in battery_bank.char_indices() {

        if c > best_first_value && i < last_index {
            best_first_index = i;
            best_first_value = c;
            best_next_index = 0;
            best_next_value = '0';
            continue;
        }

        if c > best_next_value {
            best_next_index = i;
            best_next_value = c;
        }
    }

    let joltage = (best_first_value.to_digit(10).unwrap() * 10) + best_next_value.to_digit(10).unwrap();

    BatteryBankSelection::new(vec!(best_first_index, best_next_index), joltage)
}

pub fn find_best_total_joltage(banks: &str) -> u32 {
    let mut joltage = 0u32; 
    for (i, bank) in banks.lines().enumerate() {
        let selection = select_max_joltage(bank);
        log::debug!("Selected {:?} (joltage: {}) for bank {}", selection.batteries, selection.joltage, i);
        joltage += selection.joltage;
    }
    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_max_joltage_one_option() {
        assert_eq!(
            select_max_joltage("56"),
            BatteryBankSelection::new(vec!(0, 1), 56)
        )
    }

    #[test]
    fn select_max_joltage_best_last() {
        assert_eq!(
            select_max_joltage("123456"),
            BatteryBankSelection::new(vec!(4, 5), 56)
        )
    }

    #[test]
    fn select_max_joltage_all_same() {
        let selection = select_max_joltage("11111");
        assert_eq!(
            11,
            selection.joltage
        )
    }

    #[test]
    fn select_max_joltage_best_first() {
        assert_eq!(
            select_max_joltage("654321"),
            BatteryBankSelection::new(vec!(0, 1), 65)
        );
    }

    #[test]
    fn select_max_joltage_mixed() {
        assert_eq!(
            select_max_joltage("393561922"),
            BatteryBankSelection::new(vec!(1, 6), 99)
        );
    }
}