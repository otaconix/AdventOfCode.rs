use std::collections::BinaryHeap;
use std::hash::Hash;
use std::iter::successors;
use std::ops::Add;

use fxhash::FxHashMap;

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
