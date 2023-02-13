use std::{collections::HashSet};

const TOWERS: usize = 3;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Tower(Vec<u8>);
#[derive(PartialEq, Hash, Eq, Debug)]
pub struct Node(Vec<Tower>); //The representation of a Hanoi Node (a vec of vec (>towers) of u8 (>disks))
pub enum Situation {
    //Start or finish situation
    Start,
    Finish,
}

impl Tower{
    fn new(capacity: usize) -> Self {//Creates f
        let vec = Vec::with_capacity(capacity);
        Tower(vec)
    }
    fn is_valid(&self, pushed_elt: u8) -> bool {
        self.empty()|| *self.last() > pushed_elt
    }
    fn add(&mut self, disk: u8) {
        self.inner_vec_mut().push(disk);
    }
    fn empty(&self) -> bool {
        self.inner_vec().is_empty()
    }
    fn height(&self) -> usize {
        self.inner_vec().len()
    }
    fn last(&self) -> &u8 {
        match self.inner_vec().last() {
            Some(refer) => refer,
            None => panic!("Error in tower length (unexpected)")//Unexpected
        }
    }
    fn pop(&mut self) -> u8{
        match self.inner_vec_mut().pop() {
            Some(popped) => popped,
            None => panic!("Source tower was considered non-empty but is (unexpected)")
        }
    }
    fn inner_vec(&self) -> &Vec<u8> {
        &self.0
    }
    fn inner_vec_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl Node {
    fn new(vec: Vec<Tower>) -> Node {//Creates a node with a given Vec<Tower>
        Node(vec)
    }
    fn clone(&self) -> Self {
        Node(self.inner_vec().clone())
    }
    pub fn situation(disks: usize, situation: Situation) -> Self {
        //Create a "start" or "finish" situation
        let mut v = vec![Tower::new(disks); TOWERS];
        (0..disks).for_each(|n| {
            v[match situation {
                Situation::Start => 0,
                Situation::Finish => TOWERS - 1,
            }].add((disks-n) as u8);
        });
        Node::new(v)
    }
    ///All the correct child(ren)/neighbor(s) of the self situation/node
    /// # Return
    /// Returns a ```Vec<Node>```, assuming that all the contained nodes are valid
    fn neighbors(&mut self/*Situation, entire set*/) -> Vec<Node> {
        let mut res = Vec::new();
            for i in 0..TOWERS {      
                let source = self.tower_mut(i);
                if source.empty(){continue;}//Tower can't be a source
                let source_last = *source.last();
                for j in 0..TOWERS {
                    if j == i{continue;}//We want to have a look on other towers than current
                    if self.tower(j).is_valid(source_last) {
                        let mut cloned_node = self.clone();
                        let last = cloned_node.tower_mut(i).pop();
                        cloned_node.tower_mut(j).add(last);
                        res.push(cloned_node);
                    }
                }
            }
        res
    }
    ///Returns a reference to the chosen tower of the set
    fn tower(&self, tower: usize) -> &Tower {
        if tower >= TOWERS{panic!("Tower index out of bounds")}
        &self.inner_vec()[tower]
    }
    ///Returns a mutable reference to the chosen tower of the set
    fn tower_mut(&mut self, tower: usize) -> &mut Tower {
        if tower >= TOWERS{panic!("Tower index out of bounds")}
        &mut self.inner_vec_mut()[tower]
    }
    fn inner_vec(&self) -> &Vec<Tower> {
        &self.0
    }
    fn inner_vec_mut(&mut self) -> &mut Vec<Tower> {
        &mut self.0
    }
    fn equal_to(&self, other: Node) -> bool {
        *self == other
    }
    fn disk_number(&self) -> usize {
        self.tower(0).height()
    }
}

/// Main function of the Hanoi Towers problem solver.
/// Explores the graph tree (DFS algorithm) until the solution is found
///
/// [More about DFS](https://www.geeksforgeeks.org/depth-first-search-or-dfs-for-a-graph/)
/// # Returns
/// ```Some(Vec<Node>)``` if a solution is found, representing the steps path from start to finish.
/// None if no one was found
pub fn explore(node: &mut Node, set: &mut HashSet<Node>) -> Option<Vec<Node>> {
    if node.equal_to(Node::situation(node.disk_number(), Situation::Finish)){
        return Some(vec![node.clone()]);
    }
    for i in node.neighbors() {
        if set.contains(node){break;}
        if let Some(mut v) = explore(node, set) {
            v.push(node.clone());
            set.insert(node.clone());
            return Some(v);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::explore;
    #[allow(unused)]
    use crate::{Node, Situation::{Start, Finish}};

    //#[ignore = ""]
    #[test]
    fn neigbors_test() {
        let node = &mut Node::situation(5, Start);
        let set = &mut HashSet::new();
        println!("{:?}", explore(node, set))
    }
}