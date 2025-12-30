use anyhow::Result as AnyResult;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
struct RequiredNodeData<'s> {
    has_seen: HashMap<&'s String, bool>,
    path_count: i64,
}

impl<'s> RequiredNodeData<'s> {
    fn new<'a: 's>(required: &[&'a String]) -> Self {
        Self {
            has_seen: required.iter().map(|r| (*r, false)).collect(),
            path_count: 0,
        }
    }

    fn see(&mut self, node: &String) {
        if let Some(has_seen) = self.has_seen.get_mut(node) {
            *has_seen = true;
        }
    }

    fn merge(&mut self, other: Self) {
        self.path_count += other.path_count;
    }

    fn can_merge(&self, other: &Self) -> bool {
        self.has_seen == other.has_seen
    }

    fn all_seen(&self) -> bool {
        self.has_seen.values().all(|seen| *seen)
    }
}

#[derive(Debug, PartialEq)]
struct Network {
    devices: HashMap<String, Vec<String>>,
}

impl Network {
    fn count_paths_impl<'s: 'arg, 'arg: 'tmp, 'tmp>(
        &'s self,
        src: &'arg String,
        dest: &'arg String,
        known_paths_by_source: &mut HashMap<&'tmp String, i64>,
    ) -> i64 {
        if let Some(count) = known_paths_by_source.get(src) {
            return *count;
        }

        let Some(next) = self.devices.get(src) else {
            return 0;
        };

        let num_paths = if next.contains(dest) {
            1
        } else {
            next.iter()
                .map(|n| self.count_paths_impl(n, dest, known_paths_by_source))
                .sum()
        };
        known_paths_by_source.insert(src, num_paths);
        num_paths
    }

    fn count_paths_with_required_nodes_impl<'s: 'arg, 'arg: 'tmp + 'node_data, 'tmp, 'node_data>(
        &'s self,
        src: &'arg String,
        dest: &'arg String,
        required: &'arg [&'arg String],
        known_paths_by_source: &mut HashMap<&'tmp String, Vec<RequiredNodeData<'node_data>>>,
    ) -> Vec<RequiredNodeData<'node_data>> {
        if let Some(data) = known_paths_by_source.get(src) {
            return data.clone();
        }

        let Some(next) = self.devices.get(src) else {
            // No path, return empty dict.
            return vec![];
        };

        let results = if next.contains(dest) {
            let mut data = RequiredNodeData::new(required);
            data.see(dest);
            data.see(src);
            data.path_count = 1;
            vec![data]
        } else {
            let mut results: Vec<RequiredNodeData<'_>> = Vec::new();
            for node in next {
                let node_results = self.count_paths_with_required_nodes_impl(
                    node,
                    dest,
                    required,
                    known_paths_by_source,
                );
                for mut data in node_results {
                    data.see(src);
                    if let Some(same_requirements) = results.iter_mut().find(|d| d.can_merge(&data))
                    {
                        same_requirements.merge(data);
                    } else {
                        results.push(data);
                    }
                }
            }
            results
        };
        known_paths_by_source.insert(src, results.clone());
        results
    }

    fn count_paths(&self, src: &String, dest: &String) -> i64 {
        let mut known_paths_by_source = HashMap::new();
        self.count_paths_impl(src, dest, &mut known_paths_by_source)
    }

    fn count_paths_with_required_nodes(
        &self,
        src: &String,
        dest: &String,
        required: &[&String],
    ) -> i64 {
        let mut known_paths_by_source = HashMap::new();
        let results = self.count_paths_with_required_nodes_impl(
            src,
            dest,
            required,
            &mut known_paths_by_source,
        );
        if let Some(result) = results.iter().find(|data| data.all_seen()) {
            result.path_count
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

    let src: String = "you".to_string();
    let dest: String = "out".to_string();
    Ok(network.count_paths(&src, &dest))
}

pub fn part2(input: &str) -> AnyResult<i64> {
    let network: Network = input.parse()?;

    let src: String = "svr".to_string();
    let dest: String = "out".to_string();
    let required = ["fft".to_string(), "dac".to_string()];
    let required_refs: Vec<&String> = required.iter().collect();
    Ok(network.count_paths_with_required_nodes(&src, &dest, &required_refs))
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
        let result = part2(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
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
