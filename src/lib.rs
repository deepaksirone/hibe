#![no_std]

trait GroupElement {
    
}

trait BilinearGroup<T: GroupElement> {
    fn multiply(&self, other: T) -> T;
    fn inverse(&self) -> T;
    fn identity(&self) -> T;
}