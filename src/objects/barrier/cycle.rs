use super::components::{BarrierFence, BarrierPost};
use bevy::{ecs::system::SystemParam, prelude::*};
use core::hash::Hash;
use std::collections::HashMap;

pub trait UndirectedGraph {
    /// Common type that the graph nodes are stored as
    type Node: Copy + Eq + Hash;

    /// Return all neighboring nodes from the given node
    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node>;

    /// Searches from the given root node to detect if a cycle occurs in the graph
    fn has_cycle(&self, root: Self::Node) -> bool {
        let mut visited = HashMap::new();
        self.dfs(root, None, &mut visited).is_some()
    }

    /// Searches from the given root node to detect if a cycle occurs in the graph
    ///
    /// Returns either `Some` list containing all nodes in the cycle or `None` if there is no cycle
    fn get_cycle(&self, root: Self::Node) -> Option<Vec<Self::Node>> {
        let mut visited = HashMap::new();

        // run dfs on the graph and if a cycle is found, collect all nodes in it
        self.dfs(root, None, &mut visited).map(|end| {
            let mut nodes = vec![end];
            let mut current = end;

            // retrace steps from the end node back to the root node
            while current != root {
                let parent = visited[&current];

                nodes.push(parent);
                current = parent;
            }

            nodes
        })
    }

    /// Recursively searches through the graph, marking nodes as visited
    ///
    /// If a cycle is found, the ending node is propogated back to the initial caller so that the path can be retraced
    fn dfs(
        &self,
        current: Self::Node,
        parent: Option<Self::Node>,
        visited: &mut HashMap<Self::Node, Self::Node>,
    ) -> Option<Self::Node> {
        visited.insert(current, parent.unwrap_or(current));

        for neighbor in self.neighbors(current) {
            if Some(neighbor) == parent {
                continue;
            }

            if visited.contains_key(&neighbor) {
                return Some(current);
            } else if let Some(end) = self.dfs(neighbor, Some(current), visited) {
                return Some(end);
            }
        }

        None
    }
}

/// Utility system parameter used for detecting cycles within the barriers
#[derive(SystemParam)]
pub struct BarrierCycleUtil<'w, 's> {
    posts: Query<'w, 's, &'static BarrierPost>,
    fences: Query<'w, 's, &'static BarrierFence>,
}

impl<'w, 's> UndirectedGraph for BarrierCycleUtil<'w, 's> {
    type Node = Entity;

    fn neighbors(&self, node: Entity) -> Vec<Entity> {
        match self.posts.get(node) {
            Ok(post) => {
                // get all neighboring posts from this post's fence connections
                post.fences
                    .iter()
                    .filter_map(|&fence_entity| self.fences.get(fence_entity).ok())
                    .filter_map(|fence| {
                        // return the connection that is not the given post
                        fence
                            .connection
                            .iter()
                            .find(|&&connect| connect != node)
                            .copied()
                    })
                    .collect::<Vec<_>>()
            }

            // if the entity given is not a post, return an empty list
            Err(_) => vec![],
        }
    }
}
