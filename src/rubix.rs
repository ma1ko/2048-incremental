fn main() {

}
/*
use std::{rc::{Rc, Weak}, cell::RefCell, mem::MaybeUninit};


fn main(){

}


struct Rubix {



}
struct Side {
    center: CenterBlock,
    edges: [Rc<Edge>; 4],
    corners: [Rc<Corner>; 4]

}
struct CenterBlock {
    color: Color
}
struct Edge {
    me: Color,
    other: Rc<Edge>

}
impl Edge {
    fn new(me: Color, other: Color) -> (Rc<Edge>, Rc<Edge>) {
        let mut m : MaybeUninit<Edge> = MaybeUninit::uninit();
        // let ptr = uninit.as_mut_ptr();
        let edge = unsafe { m.assume_init() };

        let mut me = Edge { me, other: Rc::new(edge) };
        let mut other = Edge { me: other, other: me.clone()};

        let me = Rc::new(me);
        let other = Rc::new(other);
        
        (me, other)

    }

}
struct Corner {
    me: Color,
    left: Rc<Corner>,
    third: Rc<Corner>,
}




use Color::*;
enum Color {
    White,
    Red,
    Blue,
    Green,
    Yellow,
    Orange // ?
}
*/
