use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

type Location = Vec<i64>; // Strictly speaking should be of length 3

const TEST_CONNS: usize = 10;
const CONNS: usize = 1000;

const X: usize = 0;

pub fn part1(
    input: impl BufRead,
    verbose: bool,
    test: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let boxes = parse(input)?;
    let distances = distances(&boxes)?;
    let indices_to_connect = distances
        .iter()
        .take(if test { TEST_CONNS } else { CONNS })
        .map(|(_, i, j)| (*i, *j))
        .collect::<Vec<_>>();
    let circuits = build_circuits(indices_to_connect, verbose);

    let mut circuit_sizes = circuits
        .into_iter()
        .map(|circuit| circuit.len())
        .collect::<Vec<_>>();

    circuit_sizes.sort();
    circuit_sizes.reverse();

    Ok(circuit_sizes
        .into_iter()
        .take(3)
        .product::<usize>()
        .to_string())
}

pub fn part2(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let boxes = parse(input)?;
    let distances = distances(&boxes)?;

    // TODO: Probs worth having a whole circuit/graph class in future
    let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();
    // Map from box index to id of the circuit it belongs to
    let mut circuits_map: HashMap<usize, usize> = HashMap::new();
    let mut circuit_id: usize = 0;

    for (_, i, j) in distances.into_iter() {
        if let Some(&id) = circuits_map.get(&i)
            && let Some(&other_id) = circuits_map.get(&j)
        {
            if id == other_id {
                // Already in same circuit so do nothing
                continue;
            }
            // Merge circuits
            let other_circuit = circuits.remove(&other_id).ok_or("Circuit does not exist")?;
            for box_index in &other_circuit {
                circuits_map.insert(*box_index, id);
            }
            let circuit = circuits.get_mut(&id).ok_or("Circuit does not exist")?;
            circuit.extend(other_circuit);
        } else if let Some(&id) = circuits_map.get(&i) {
            let circuit = circuits.get_mut(&id).ok_or("Circuit does not exist")?;
            circuit.insert(j);
            circuits_map.insert(j, id);
        } else if let Some(&id) = circuits_map.get(&j) {
            let circuit = circuits.get_mut(&id).ok_or("Circuit does not exist")?;
            circuit.insert(i);
            circuits_map.insert(i, id);
        } else {
            let new_circuit = HashSet::from([i, j]);
            circuits.insert(circuit_id, new_circuit);
            circuits_map.insert(i, circuit_id);
            circuits_map.insert(j, circuit_id);
            circuit_id += 1;
        };
        if verbose {
            println!("{:?}", circuits_map);
        }

        if circuits.len() == 1
            && circuits
                .iter()
                .next()
                .and_then(|(_, circuit)| Some(circuit.len() == boxes.len()))
                .ok_or("Failed to get circuit")?
        {
            return Ok((boxes[i][X] * boxes[j][X]).to_string());
        }
    }

    Err("No solution found".into())
}

fn parse(input: impl BufRead) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let lines = input.lines();
    let boxes: Vec<Location> = lines
        .map(|line| {
            line.map(|s| {
                s.split(',')
                    .map(|coord| coord.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()
            })
        })
        .collect::<Result<Result<Vec<_>, _>, _>>()??;
    Ok(boxes)
}

fn distances(
    boxes: &Vec<Location>,
) -> Result<Vec<(f64, usize, usize)>, Box<dyn std::error::Error>> {
    let mut distances = boxes
        .iter()
        .enumerate()
        .flat_map(|i_a| boxes.iter().enumerate().map(move |j_b| (i_a, j_b)))
        .filter(|((i, _), (j, _))| *i < *j) // Filter out themselves and only calculate if not already previously
        .map(|((i, a), (j, b))| {
            let distance = distance(a, b);
            distance.map(|d| (d, i, j))
        })
        .collect::<Result<Vec<_>, _>>()?;
    distances.sort_by(|(d1, _, _), (d2, _, _)| d1.total_cmp(d2));
    Ok(distances)
}

fn distance(a: &Location, b: &Location) -> Result<f64, Box<dyn std::error::Error>> {
    let [x, y, z] = a[..3] else {
        return Err("Invalid a".into());
    };
    let [x1, y1, z1] = b[..3] else {
        return Err("Invalid b".into());
    };
    Ok((((x - x1).pow(2) + (y - y1).pow(2) + (z - z1).pow(2)) as f64).sqrt())
}

fn build_circuits(indices_to_connect: Vec<(usize, usize)>, verbose: bool) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    // Map from box index to index into `circuits` of the circuit it belongs to
    let mut circuits_map: HashMap<usize, usize> = HashMap::new();

    for (i, j) in indices_to_connect {
        if let Some(&index) = circuits_map.get(&i)
            && let Some(&other_index) = circuits_map.get(&j)
        {
            if index == other_index {
                // Already in same circuit so do nothing
                continue;
            }
            // Merge circuits
            let other_circuit = &circuits[other_index].clone();
            circuits[other_index] = HashSet::new(); // Soft-remove circuit from old location so indices don't change
            for box_index in other_circuit {
                circuits_map.insert(*box_index, index);
            }
            circuits[index].extend(other_circuit);
        } else if let Some(&index) = circuits_map.get(&i) {
            circuits[index].insert(j);
            circuits_map.insert(j, index);
        } else if let Some(&index) = circuits_map.get(&j) {
            circuits[index].insert(i);
            circuits_map.insert(i, index);
        } else {
            let new_circuit = HashSet::from([i, j]);
            circuits.push(new_circuit);
            let new_index = circuits.len() - 1;
            circuits_map.insert(i, new_index);
            circuits_map.insert(j, new_index);
        };
        if verbose {
            println!("{:?}", circuits_map);
        }
    }
    circuits
}
