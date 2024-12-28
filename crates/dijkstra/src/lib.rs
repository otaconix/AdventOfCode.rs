use std::collections::BinaryHeap;
use std::hash::Hash;
use std::iter::successors;
use std::ops::Add;

use fxhash::FxHashMap;
use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq)]
pub struct DijkstraVertex<N: Eq, P: Ord> {
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

impl<N: Eq, P: Ord> DijkstraVertex<N, P> {
    pub fn new(node: N, distance: P) -> Self {
        Self { node, distance }
    }
}

pub struct DijkstraState<Node: Hash + Eq + Copy, P: Add<P, Output = P> + Ord + Default + Copy> {
    pub queue: BinaryHeap<DijkstraVertex<Node, P>>,
    pub prevs: FxHashMap<Node, FxHashSet<Node>>,
    pub distances: FxHashMap<Node, P>,
    pub found_ends: Vec<(Node, P)>,
}

impl<Node, P> DijkstraState<Node, P>
where
    Node: Hash + Eq + Copy,
    P: Add<P, Output = P> + Ord + Default + Copy,
{
    pub fn new(initial_node: Node) -> Self {
        Self {
            prevs: FxHashMap::default(),
            distances: {
                let mut distances = FxHashMap::default();
                distances.insert(initial_node, P::default());

                distances
            },
            found_ends: Vec::new(),
            queue: BinaryHeap::from([DijkstraVertex {
                node: initial_node,
                distance: P::default(),
            }]),
        }
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

pub fn build_minimal_paths<Node: Hash + Eq + Copy, P: Ord + Copy>(
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
) -> Option<DijkstraState<Node, P>> {
    let mut state = DijkstraState::new(start);

    while let Some(DijkstraVertex { distance, node }) = state.queue.pop() {
        if is_end(&node) {
            state.found_ends.push((node, distance));
            continue;
        }

        for (neighbor, neighbor_distance) in neighbors(&node) {
            let new_distance = neighbor_distance + distance;
            let distance_compared_to_original = state
                .distances
                .get(&neighbor)
                .map(|existing_distance| new_distance.cmp(existing_distance))
                .unwrap_or(std::cmp::Ordering::Less);

            if distance_compared_to_original.is_le() {
                let prevs = state.prevs.entry(neighbor).or_default();

                if distance_compared_to_original.is_lt() {
                    prevs.clear();
                }

                state.distances.insert(neighbor, new_distance);
                prevs.insert(node);
                state.queue.push(DijkstraVertex {
                    distance: new_distance,
                    node: neighbor,
                });
            }
        }
    }

    if !state.found_ends.is_empty() {
        Some(state)
    } else {
        None
    }
}

pub fn dijkstra_with_state<
    Node: Hash + Eq + Copy,
    P: Add<P, Output = P> + Ord + Default + Copy,
    I: Iterator<Item = (Node, P)>,
    IsEnd: Fn(&Node) -> bool,
    Neighbors: Fn(&Node) -> I,
>(
    state: &mut DijkstraState<Node, P>,
    is_end: IsEnd,
    neighbors: Neighbors,
) -> Option<Vec<(Node, P)>> {
    while let Some(DijkstraVertex { distance, node }) = state.queue.pop() {
        if is_end(&node) {
            state.found_ends.push((node, distance));
            break;
        }

        for (neighbor, neighbor_distance) in neighbors(&node) {
            let new_distance = neighbor_distance + distance;
            let existing_distance = state.distances.get(&neighbor);

            if existing_distance.is_none() || &new_distance < existing_distance.unwrap() {
                state.distances.insert(neighbor, new_distance);
                state.prevs.insert(neighbor, {
                    let mut set = FxHashSet::default();
                    set.insert(node);

                    set
                });
                state.queue.push(DijkstraVertex {
                    distance: new_distance,
                    node: neighbor,
                });
            }
        }
    }

    if let Some((end, _)) = state.found_ends.first() {
        let mut path = successors(Some((*end, state.distances[end])), |(current, _)| {
            state
                .prevs
                .remove(current)
                .map(|prev| prev.into_iter().next().unwrap())
                .map(|prev| (prev, state.distances[&prev]))
        })
        .collect::<Vec<_>>();
        path.reverse();
        Some(path)
    } else {
        None
    }
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
    dijkstra_with_state(&mut DijkstraState::new(start), is_end, neighbors)
}
