#[derive(Debug, PartialEq)]
struct BatteryBankSelection {
    /// The indices of the batteries to turn on in the bank.
    batteries: Vec<usize>,
    /// The joltage of this bank from turning on these batteries.
    joltage: u64
}

impl BatteryBankSelection {
    fn new(batteries: Vec<usize>, joltage: u64) -> BatteryBankSelection {
        BatteryBankSelection { batteries, joltage }
    }
}

/// Finds the joltage for the battery bank, defined as the max two digits
fn select_max_joltage(battery_bank: &str, num_batteries: u8) -> BatteryBankSelection {
    let mut selected_batteries = Vec::with_capacity(num_batteries.into());

    let last_full_index = battery_bank.len() - num_batteries as usize;
    for (i, c) in battery_bank.char_indices() {
        let mut is_done = false;
        for i_selected in 0..selected_batteries.len() {
            let last_index = last_full_index + i_selected;
            let (_, c2) = selected_batteries[i_selected];
            if c > c2 && i <= last_index {
                selected_batteries[i_selected] = (i, c);
                // Drop all subsequent selections.
                selected_batteries.resize(i_selected + 1, (0, '0'));
                is_done = true;
                break;
            }
        }

        if !is_done && selected_batteries.len() < num_batteries.into() {
            selected_batteries.push((i, c));
        }
    }

    let joltage = selected_batteries.iter().fold(0u64, |acc, (_, c)| (acc * 10) + c.to_digit(10).unwrap() as u64);

    BatteryBankSelection::new(
        selected_batteries.iter().map(|(i, _)| *i).collect(),
        joltage)
}

/// Finds the best total joltage across all the battery banks by taking the 
pub fn find_best_total_joltage(banks: &str, num_batteries_per_bank: u8) -> u64 {
    let mut joltage = 0u64; 
    for (i, bank) in banks.lines().enumerate() {
        let selection = select_max_joltage(bank, num_batteries_per_bank);
        log::debug!("Selected {:?} (joltage: {}) for bank {} ({})", selection.batteries, selection.joltage, i, bank);
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
            select_max_joltage("56", 2),
            BatteryBankSelection::new(vec!(0, 1), 56)
        )
    }

    #[test]
    fn select_max_joltage_best_last() {
        assert_eq!(
            select_max_joltage("123456", 2),
            BatteryBankSelection::new(vec!(4, 5), 56)
        );

        assert_eq!(
            select_max_joltage("111234559", 6),
            BatteryBankSelection::new(vec!(3, 4, 5, 6, 7, 8), 234559)
        );
    }

    #[test]
    fn select_max_joltage_all_same() {
        let selection = select_max_joltage("11111", 2);
        assert_eq!(
            11,
            selection.joltage
        );

        let selection = select_max_joltage("11111111", 4);
        assert_eq!(
            1111,
            selection.joltage
        );
    }

    #[test]
    fn select_max_joltage_best_first() {
        assert_eq!(
            select_max_joltage("654321", 2),
            BatteryBankSelection::new(vec!(0, 1), 65)
        );

        assert_eq!(
            select_max_joltage("54321111111", 7),
            BatteryBankSelection::new(vec!(0, 1, 2, 3, 4, 5, 6), 5432111)
        );
    }

    #[test]
    fn select_max_joltage_mixed() {
        assert_eq!(
            select_max_joltage("373561922", 2),
            BatteryBankSelection::new(vec!(6, 7), 92)
        );

        assert_eq!(
            select_max_joltage("39356192238781", 5),
            BatteryBankSelection::new(vec!(1, 6, 10, 12, 13), 99881)
        );
    }
}