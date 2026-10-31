// NO TOUCHING THE BOMB
// (You may not edit this file)

/// Dr. Evil's bomb! Panics when dropped.
pub struct Bomb;

impl Drop for Bomb {
    fn drop(&mut self) {
        panic!("BOOM!!!");
    }
}
