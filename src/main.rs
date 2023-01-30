use std::fmt::Debug;


type ChildNode<T> = Option<Box<BinData<T>>>;
pub struct BinData<T> {
   pub data: T,
   pub left: ChildNode<T>,
   pub right: ChildNode<T>
   
}

pub struct BinTree<T> (Option<Box<BinData<T>>>);

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
}
impl<T: PartialOrd> BinTree<T>{
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd ) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                }
                else{
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }))

            }

        }
    }
}


fn main(){
    print!("Hola");
}





