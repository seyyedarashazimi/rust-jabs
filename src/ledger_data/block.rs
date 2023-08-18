use std::hash::Hash;

pub trait Block: Eq + PartialEq + Ord + Hash {
    // fn generate_genesis_block() -> Self;

    fn get_creation_time(&self) -> f64;
    fn get_creator(&self) -> Option<usize>;
    fn get_height(&self) -> i32;
    fn get_size(&self) -> u64;
    fn get_parents(&self) -> &Vec<usize>;

    fn set_creation_time(&mut self, creation_time: f64);
    fn set_creator(&mut self, creator: Option<usize>);
    fn set_height(&mut self, height: i32);
    fn set_size(&mut self, size: u64);
    fn set_parents(&mut self, parents: Vec<usize>);

    fn set_base_block_fields(
        &mut self,
        creation_time: f64,
        creator: Option<usize>,
        height: i32,
        size: u64,
        parents: Vec<usize>,
    ) {
        self.set_creation_time(creation_time);
        self.set_creator(creator);
        self.set_height(height);
        self.set_size(size);
        self.set_parents(parents);
    }
}
