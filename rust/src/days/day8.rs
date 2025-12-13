use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    num::ParseFloatError,
};

use nalgebra::{Point3, point};

use crate::utils::input::Error;

type BoxPoint = Point3<f64>;

struct BoxDistance {
    id_a: usize,
    id_b: usize,
    distance: f64,
}

impl BoxDistance {
    fn between(id_a: usize, a: &BoxPoint, id_b: usize, b: &BoxPoint) -> Self {
        let distance = nalgebra::distance(a, b);
        Self {
            id_a,
            id_b,
            distance,
        }
    }
}

impl PartialEq for BoxDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl Eq for BoxDistance {}

impl PartialOrd for BoxDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoxDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance).reverse()
    }
}

fn build_distance_heap(points: &[BoxPoint]) -> BinaryHeap<BoxDistance> {
    let mut heap = BinaryHeap::with_capacity(points.len() * (points.len() - 1) / 2);
    for id_a in 0..points.len() {
        let a = &points[id_a];
        #[allow(clippy::needless_range_loop)]
        for id_b in (id_a + 1)..points.len() {
            let b = &points[id_b];
            heap.push(BoxDistance::between(id_a, a, id_b, b));
        }
    }
    heap
}

fn create_closest_n_connections(n: usize, points: &[BoxPoint]) -> Vec<HashSet<usize>> {
    let mut heap = build_distance_heap(points);
    let mut circuits_by_point_id: HashMap<usize, usize> =
        (0..points.len()).map(|p_id| (p_id, p_id)).collect();
    let mut points_by_circuit_id: HashMap<usize, HashSet<usize>> = circuits_by_point_id
        .iter()
        .map(|(&p_id, &c_id)| (c_id, HashSet::from([p_id])))
        .collect();

    let mut num_connected = 0;
    while !heap.is_empty() && num_connected < n {
        let next = heap.pop().unwrap();
        let c_id_a = circuits_by_point_id[&next.id_a];
        let c_id_b = circuits_by_point_id[&next.id_b];
        num_connected += 1;

        if c_id_a == c_id_b {
            continue;
        }

        // Swap all B points to be on the A circuit, and drop the B circuit.
        let b_points = points_by_circuit_id.remove(&c_id_b).unwrap();
        points_by_circuit_id
            .get_mut(&c_id_a)
            .unwrap()
            .extend(&b_points);
        for p_id in b_points {
            circuits_by_point_id.insert(p_id, c_id_a);
        }
    }

    let mut circuits: Vec<HashSet<usize>> = points_by_circuit_id.into_values().collect();
    circuits.sort_unstable_by_key(|c| c.len());
    circuits.reverse();
    circuits
}

/// Connects the points into a single circuit, returning the pairs of points that are connected in order.
fn connect_into_one_circuit(points: &[BoxPoint]) -> Vec<(BoxPoint, BoxPoint)> {
    let mut heap = build_distance_heap(points);
    let mut circuits_by_point_id: HashMap<usize, usize> =
        (0..points.len()).map(|p_id| (p_id, p_id)).collect();
    let mut points_by_circuit_id: HashMap<usize, HashSet<usize>> = circuits_by_point_id
        .iter()
        .map(|(&p_id, &c_id)| (c_id, HashSet::from([p_id])))
        .collect();
    let mut connected_points = Vec::new();

    while !heap.is_empty() && points_by_circuit_id.len() > 1 {
        let next = heap.pop().unwrap();
        let c_id_a = circuits_by_point_id[&next.id_a];
        let c_id_b = circuits_by_point_id[&next.id_b];

        if c_id_a == c_id_b {
            continue;
        }

        connected_points.push((
            *points.get(next.id_a).unwrap(),
            *points.get(next.id_b).unwrap(),
        ));

        // Swap all B points to be on the A circuit, and drop the B circuit.
        let b_points = points_by_circuit_id.remove(&c_id_b).unwrap();
        points_by_circuit_id
            .get_mut(&c_id_a)
            .unwrap()
            .extend(&b_points);
        for p_id in b_points {
            circuits_by_point_id.insert(p_id, c_id_a);
        }
    }

    connected_points
}

pub fn parse_points(input: &str) -> Result<Vec<BoxPoint>, Error> {
    input
        .lines()
        .map(|p_str| {
            let parse_result: Result<Vec<f64>, ParseFloatError> =
                p_str.split(',').map(|n_str| n_str.trim().parse()).collect();
            let numbers = parse_result
                .map_err(|e| Error::ParseError(format!("error parsing number: {:?}", e)))?;
            if numbers.len() != 3 {
                return Err(Error::ParseError(format!(
                    "points must have three numbers, parsed {} from {}",
                    numbers.len(),
                    p_str
                )));
            }
            Ok(point![numbers[0], numbers[1], numbers[2]])
        })
        .collect()
}

pub fn multiply_n_largest_circuits_after_m_connections(
    n: usize,
    m: usize,
    points: &[BoxPoint],
) -> u64 {
    let circuits = create_closest_n_connections(m, points);
    log::info!("Created {} circuits", circuits.len());
    let mut result = 1;
    for i in 0..n {
        if let Some(c) = circuits.get(i) {
            log::info!("Multiplying circuit sized: {}", c.len());
            result *= c.len() as u64;
        } else {
            break;
        }
    }
    result
}

pub fn part2(points: &[BoxPoint]) -> f64 {
    let pairs = connect_into_one_circuit(points);
    log::info!("Connected {} pairs", pairs.len());

    pairs.last().map(|(a, b)| a.x * b.x).unwrap_or(0.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_closest_n_connections() {
        let circuits = create_closest_n_connections(
            2,
            &[
                BoxPoint::new(1., 1., 1.),
                BoxPoint::new(5., 5., 5.),
                BoxPoint::new(2., 2., 2.),
                BoxPoint::new(2., 1., 1.),
            ],
        );

        assert_eq!(circuits, vec!(HashSet::from([0, 2, 3]), HashSet::from([1])));
    }

    #[test]
    fn test_parse_points() {
        let result = parse_points(
            "1,32,3
-5,9,0
3,2,2",
        );
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap(),
            vec![point![1., 32., 3.], point![-5., 9., 0.], point![3., 2., 2.]]
        );
    }

    #[test]
    fn test_multiply_n_largest_circuits_after_m_connections() {
        let points = parse_points(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,690",
        )
        .unwrap();
        let total = multiply_n_largest_circuits_after_m_connections(3, 10, &points);

        assert_eq!(total, 40);
    }

    #[test]
    fn test_connect_into_one_circuit() {
        let points = parse_points(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,690",
        )
        .unwrap();
        let connected_pairs = connect_into_one_circuit(&points);

        let (a, b) = *connected_pairs.last().unwrap();

        assert!(a.x == 216. || a.x == 117.);
        if a.x == 216. {
            assert_eq!(b.x, 117.);
        } else {
            assert_eq!(b.x, 216.);
        }
    }

    #[test]
    fn test_part2() {
        let points = parse_points(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,690",
        )
        .unwrap();
        let total = part2(&points);

        assert_eq!(total, 25272.);
    }
}
