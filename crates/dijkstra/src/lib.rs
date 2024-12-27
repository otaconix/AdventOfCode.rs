use std::collections::BinaryHeap;
use std::hash::Hash;
use std::iter::successors;
use std::ops::Add;

use fxhash::FxHashMap;
use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq)]
struct DijkstraVertex<N: Eq, P: Ord> {
    distance: P,
    node: N,
}

impl<T: Eq, P: Ord> PartialOrd for DijkstraVertex<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq, P: Ord> Ord for DijkstraVertex<T, P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

fn build_path<Node: Hash + Eq + Copy, P: Copy>(
    current_node: Node,
    prevs: &FxHashMap<Node, FxHashSet<Node>>,
    distances: &FxHashMap<Node, P>,
) -> Vec<Vec<(Node, P)>> {
    if let Some(current_prevs) = prevs.get(&current_node) {
        current_prevs
            .iter()
            .flat_map(|prev| {
                build_path(*prev, prevs, distances)
                    .into_iter()
                    .map(|mut prev_path| {
                        let mut path = vec![(current_node, distances[&current_node])];
                        path.append(&mut prev_path);

                        path
                    })
            })
            .collect()
    } else {
        vec![vec![(current_node, distances[&current_node])]]
    }
}

fn build_minimal_paths<Node: Hash + Eq + Copy, P: Ord + Copy>(
    ends: Vec<(Node, P)>,
    prevs: FxHashMap<Node, FxHashSet<Node>>,
    distances: FxHashMap<Node, P>,
) -> Option<Vec<Vec<(Node, P)>>> {
    let result = ends
        .into_iter()
        .min_set_by_key(|(_, distance)| *distance)
        .into_iter()
        .flat_map(|(end, _)| build_path(end, &prevs, &distances))
        .collect_vec();
    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

pub fn dijkstra_all_shortest_paths<
    Node: Hash + Eq + Copy,
    P: Add<P, Output = P> + Ord + Default + Copy,
    I: Iterator<Item = (Node, P)>,
    IsEnd: Fn(&Node) -> bool,
    Neighbors: Fn(&Node) -> I,
>(
    start: Node,
    is_end: IsEnd,
    neighbors: Neighbors,
) -> Option<Vec<Vec<(Node, P)>>> {
    let mut queue: BinaryHeap<DijkstraVertex<Node, P>> = BinaryHeap::from([DijkstraVertex {
        distance: P::default(),
        node: start,
    }]);
    let mut prevs: FxHashMap<Node, FxHashSet<Node>> = FxHashMap::default();
    let mut distances: FxHashMap<Node, P> = FxHashMap::default();
    distances.insert(start, P::default());
    let mut found_ends = vec![];

    while let Some(DijkstraVertex { distance, node }) = queue.pop() {
        if is_end(&node) {
            found_ends.push((node, distance));
            continue;
        }

        for (neighbor, neighbor_distance) in neighbors(&node) {
            let new_distance = neighbor_distance + distance;
            let distance_compared_to_original = distances
                .get(&neighbor)
                .map(|existing_distance| new_distance.cmp(existing_distance))
                .unwrap_or(std::cmp::Ordering::Less);

            if distance_compared_to_original.is_le() {
                let prevs = prevs.entry(neighbor).or_default();

                if distance_compared_to_original.is_lt() {
                    prevs.clear();
                }

                distances.insert(neighbor, new_distance);
                prevs.insert(node);
                queue.push(DijkstraVertex {
                    distance: new_distance,
                    node: neighbor,
                });
            }
        }
    }

    build_minimal_paths(found_ends, prevs, distances).map(|paths| {
        paths
            .into_iter()
            .map(|mut path| {
                path.reverse();

                path
            })
            .collect()
    })
}

pub fn dijkstra<
    Node: Hash + Eq + Copy,
    P: Add<P, Output = P> + Ord + Default + Copy,
    I: Iterator<Item = (Node, P)>,
    IsEnd: Fn(&Node) -> bool,
    Neighbors: Fn(&Node) -> I,
>(
    start: Node,
    is_end: IsEnd,
    neighbors: Neighbors,
) -> Option<Vec<(Node, P)>> {
    let mut queue: BinaryHeap<DijkstraVertex<Node, P>> = BinaryHeap::from([DijkstraVertex {
        distance: P::default(),
        node: start,
    }]);
    let mut prevs: FxHashMap<Node, Node> = FxHashMap::default();
    let mut distances: FxHashMap<Node, P> = FxHashMap::default();
    distances.insert(start, P::default());
    let mut found_end = None;

    while let Some(DijkstraVertex { distance, node }) = queue.pop() {
        if is_end(&node) {
            found_end = Some(node);
            break;
        }

        for (neighbor, neighbor_distance) in neighbors(&node) {
            let new_distance = neighbor_distance + distance;
            let existing_distance = distances.get(&neighbor);

            if existing_distance.is_none() || &new_distance < existing_distance.unwrap() {
                distances.insert(neighbor, new_distance);
                prevs.insert(neighbor, node);
                queue.push(DijkstraVertex {
                    distance: new_distance,
                    node: neighbor,
                });
            }
        }
    }

    if let Some(end) = found_end {
        let mut path = successors(Some((end, distances[&end])), |(current, _)| {
            prevs.remove(current).map(|prev| (prev, distances[&prev]))
        })
        .collect::<Vec<_>>();
        path.reverse();
        Some(path)
    } else {
        None
    }
}
