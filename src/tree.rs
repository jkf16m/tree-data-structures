#[derive(Debug, Clone)]
pub struct Node<T>{
    depth_level: usize,
    parent_id: Option<usize>,
    children_id: Vec<usize>,
    value: T
}

pub struct Tree<T>{
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T>{
    pub fn new_empty()->Tree<T>{
        Tree{
            nodes: Vec::new(),
        }
    }
    pub fn new(value: T)->Tree<T>{
        let node = Node { depth_level: 0, parent_id: None, value: value, children_id: Vec::new() };
        Tree{
            nodes: vec![node],
        }
    }


    /// Returns the max depth of the tree
    /// 
    /// # Returns
    /// 
    /// * `usize` - The max depth of the tree
    pub fn get_current_max_depth(&self)->usize{
        let mut max_depth = 0;
        for node in &self.nodes{
            if node.depth_level > max_depth{
                max_depth = node.depth_level;
            }
        }
        max_depth
    }


    pub fn get_children(&self, parent_id: usize)->Vec<&Node<T>>{
        let mut children = Vec::new();
        for child_id in &self.nodes[parent_id].children_id{
            children.push(&self.nodes[*child_id]);
        }
        children
    }

    /// Returns the node id of the children that matches the value of the parent node
    /// 
    /// # Arguments
    /// 
    /// * `parent_id` - The id of the parent node
    /// * `value` - The value of the child node
    /// 
    /// # Returns
    /// 
    /// * `Option<usize>` - The id of the child node, Some value or None, whether it matched or not
    pub fn matches_children(&self, parent_id: usize, value:T)->Option<usize> where T: PartialEq{
        for child_id in &self.nodes[parent_id].children_id{
            if self.nodes[*child_id].value == value{
                return Some(*child_id);
            }
        }
        None
    }

    /// Returns the tree itself, with the new node added (this method is chainable and
    /// takes ownership of the tree)
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value of the new node
    /// * `parent_id` - The id of the parent node of the new node
    /// 
    /// # Returns
    /// 
    /// * `Tree<T>` - The tree itself, with the new node added
    /// 
    /// # Example
    /// 
    /// ```
    /// let tree = Tree::<u32>::new(0);
    /// let tree = tree.add(1, 0).add(2, 1).add(3, 2); // chainable
    /// 
    /// assert_eq!(tree.nodes.len(), 4);
    /// assert_eq!(tree.nodes[0].value, 0);
    /// assert_eq!(tree.nodes[1].value, 1);
    /// assert_eq!(tree.nodes[2].value, 2);
    /// assert_eq!(tree.nodes[3].value, 3);
    /// ```
    pub fn add(mut self, value: T, parent_id: usize)->Tree<T>{
        let node = Node {
            depth_level: self.nodes[parent_id].depth_level + 1,
            parent_id: Some(parent_id),
            value: value,
            children_id: Vec::new()
        };
        self.nodes.push(node);

        let length_of_nodes = self.nodes.len();
        let mut parent = &mut self.nodes[parent_id];
        parent.children_id.push(length_of_nodes - 1);
        self
    }

    pub fn find(&self, predicate: &dyn Fn(&T)->bool)->Option<usize>{
        for (i, node) in self.nodes.iter().enumerate(){
            if predicate(&node.value){
                return Some(i);
            }
        }
        None
    }

    /// Returns the last node if the branch matches from the root
    /// to the last node
    /// 
    /// # Arguments
    /// 
    /// * `branch` - A vector of values that represents the branch
    /// 
    /// # Returns
    /// 
    /// * `Option<T>` - The last node matching branch, Some value or None, whether it matched or not
    /// 
    /// # Example
    /// 
    /// ```
    /// let tree = Tree::<u32>::new(0);
    /// let tree = tree.add(1, 0);
    /// let tree = tree.add(2, 1);
    /// let tree = tree.add(3, 2);
    /// // it should return the last node of this branch
    /// // in this case, 3
    /// let searched_value = tree.matches_branch([0,1,2,3].to_vec());
    /// assert_eq!(searched_value, Some(3));
    /// ```
    pub fn matches_branch(&self, branch: Vec<T>)->Option<Node<T>> where T: PartialEq + Clone{
        let result = self.matches_branch_predicated(branch, &|branch_value, tree_value| branch_value == tree_value);
        result
    }

    /// Returns the last node if the branch matches from the root, using a predicate
    /// to compare the branch value with the tree values
    /// Useful in case your T type either doesn't implement PartialEq 
    /// or you wanna compare a really specific value.
    /// 
    /// # Arguments
    /// 
    /// * `branch` - A vector of values that represents the branch
    /// * `predicate` - A function that receives the branch value and the tree value
    /// 
    /// # Returns
    /// 
    /// * `Option<T>` - The last node matching branch, wrapped in Option<T> or None,
    ///     whether it matched or not
    /// 
    /// # Example
    /// 
    /// ```
    /// let tree = Tree::<u32>::new(0);
    /// let tree = tree.add(1, 0);
    /// let tree = tree.add(2, 1);
    /// let tree = tree.add(3, 2);
    /// // it should return the last node of this branch
    /// // in this case, 3
    /// let node_option = tree.matches_branch_predicated([0,1,2,3].to_vec(),
    ///     &|branch_value, tree_value| branch_value == tree_value
    /// );
    /// let node = node_option.unwrap_or_default().value;
    /// 
    /// assert_eq!(node.value, 3);
    pub fn matches_branch_predicated<U>(&self, branch:Vec<U>, predicate: &dyn Fn(&U, &T)->bool)->Option<Node<T>> where T: Clone{
        let mut current_nodes_id = [0].to_vec();
        let mut current_node_children_id: Vec<usize> = Vec::new();
        for (id,value) in branch.iter().enumerate(){
            // this is for the case where the node has children
            if (current_nodes_id.len() > 0){
                for &current_node_id in &current_nodes_id.clone(){
                    let node = &self.nodes[current_node_id];
                    if predicate(&value,&node.value){
                        current_nodes_id = node.children_id.clone();

                        // if the node has no children, return it
                        if current_nodes_id.len() == 0 {
                            return Some(self.nodes[current_node_id].clone());
                        // if this is the last value in the branch, return it
                        }else if id == branch.len() - 1{
                            return Some(self.nodes[current_node_id].clone());
                        }else{
                            break;
                        }
                    }
                }
            }
        }
        return None;
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    // returns a tree instance for testing purposes
    fn setup_tree()->Tree<u32>{
        let tree = Tree::<u32>::new(10)
                                .add(1, 0)
                                .add(2,0)
                                .add(5,0)
                                .add(2,1)
                                .add(3,1)
                                .add(3,4)
                                .add(4,2)
                                .add(8,2);

        tree
    }

    fn setup_complex_tree()->Tree<(u32,u32)>{
        let tree = Tree::<(u32,u32)>::new((0,0))
                                .add((1,1), 0)
                                .add((2,2),0)
                                .add((5,5),0)
                                .add((2,2),1)
                                .add((3,3),1)
                                .add((3,8),4)
                                .add((4,4),2)
                                .add((8,8),2);
        tree
    }

    #[test]
    fn test_tree(){
        let tree = setup_tree();
        assert_eq!(tree.nodes.len(), 9);
    }

    #[test]
    fn added_elements_to_root(){
        let tree = Tree::<u32>::new(0);
        let tree = tree.add(1, 0);
        let tree = tree.add(2, 1);
        let tree = tree.add(3, 2);
        assert_eq!(tree.nodes.len(), 4);
    }

    #[test]
    fn check_the_children_added_to_root(){
        let tree = Tree::<u32>::new(0)
            .add(1,0)
            .add(2,0)
            .add(3,0);
        
        let children = tree.get_children(0);

        assert_eq!(children.len(), 3);
        assert_eq!(children[0].value, 1);
        assert_eq!(children[1].value, 2);
        assert_eq!(children[2].value, 3);

    }

    #[test]
    fn matches_array_of_values(){
        let tree = setup_tree();
        // it should return the last node of this branch
        // in this case, 3
        let searched_value = tree.matches_branch([10,1,2,3].to_vec()); 
        let node = if searched_value.is_some() {searched_value.unwrap()} else {return};
        let value = node.value;

        assert_eq!(value, 3);
    }

    #[test]
    fn matches_array_of_values_mid_tree(){
        let tree = setup_tree();
        // it should return the last node of this branch
        // in this case, 3
        let searched_value = tree.matches_branch([10,1,2].to_vec()); 
        let node = if searched_value.is_some() {searched_value.unwrap()} else {return};
        let value = node.value;

        assert_eq!(value, 2);
    }

    #[test]
    fn add_child_to_first_found_value_in_tree(){
        let tree = setup_tree();
        let node_id = tree.find(&|&val| val == 2).unwrap();
        let tree_updated = tree.add(4, node_id);

        let searched_value = tree_updated.matches_branch([10,1,2,4].to_vec());
        let node = if searched_value.is_some() {searched_value.unwrap()} else {return};
        let value = node.value;

        assert_eq!(value, 4);
    }

    #[test]
    fn match_branch_with_predicate(){
        let tree = setup_complex_tree();
        let returned_node = tree.matches_branch_predicated::<u32>([0,1,2,3].to_vec(),
            &|branch_value, tree_value| *branch_value == tree_value.0
        );

        let node = returned_node.unwrap();
        let value = node.value;

        assert_eq!(value, (3,8));
        assert_eq!(node.parent_id, Some(4));
    }
}