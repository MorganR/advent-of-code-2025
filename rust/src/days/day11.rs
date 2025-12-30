use anyhow::Result as AnyResult;
use std::collections::{HashMap, HashSet, VecDeque};
use std::os::linux::net;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Network {
    devices: HashMap<String, Vec<String>>,
}

impl Network {
    fn count_paths(&self, src: &String, dest: &String) -> i64 {
        if let Some(next) = self.devices.get(src) {
            if next.contains(dest) {
                1
            } else {
                next.iter().map(|n| self.count_paths(n, dest)).sum()
            }
        } else {
            0
        }
    }
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut devices = HashMap::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (device_name, connections) = line
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("Invalid line format: missing colon"))?;

            let device_name = device_name.trim().to_string();
            let connections: Vec<String> = connections
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            devices.insert(device_name, connections);
        }

        Ok(Network { devices })
    }
}

pub fn part1(input: &str) -> AnyResult<i64> {
    let network: Network = input.parse()?;

    Ok(network.count_paths(&"you".to_string(), &"out".to_string()))
}

pub fn part2(input: &str) -> AnyResult<i64> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_part2() {
        let result = part2("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_parsing() {
        let input = "aaa: you bbb
you: bbb ccc
bbb: ccc
ccc: out";

        let network: Network = input.parse().unwrap();

        assert_eq!(network.devices.len(), 4);
        assert_eq!(network.devices.get("aaa").unwrap(), &vec!["you", "bbb"]);
        assert_eq!(network.devices.get("you").unwrap(), &vec!["bbb", "ccc"]);
        assert_eq!(network.devices.get("bbb").unwrap(), &vec!["ccc"]);
        assert_eq!(network.devices.get("ccc").unwrap(), &vec!["out"]);
    }

    #[test]
    fn test_network_parsing_empty_lines() {
        let input = "aaa: you hhh

you: bbb ccc
";

        let network: Network = input.parse().unwrap();

        assert_eq!(network.devices.len(), 2);
        assert_eq!(network.devices.get("aaa").unwrap(), &vec!["you", "hhh"]);
        assert_eq!(network.devices.get("you").unwrap(), &vec!["bbb", "ccc"]);
    }

    #[test]
    fn test_network_parsing_invalid_format() {
        let input = "invalid line without colon";
        let result: Result<Network, _> = input.parse();
        assert!(result.is_err());
    }
}
