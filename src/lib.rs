/// Dirty wraps a value of type T with functions similiar to that of a Read/Write
/// lock but simply sets a dirty flag on write(), reset on read()
pub struct Dirty<T> {
    value: T,
    dirty: bool,
}

impl<T> Dirty<T> {
    /// Create a new Dirty
    pub fn new(val: T) -> Dirty<T> {
        Dirty {
            value: val,
            dirty: true,
        }
    }

    /// Returns true if dirty, false otherwise
    #[allow(dead_code)]
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Writable value return, sets the dirty flag
    pub fn write(&mut self) -> &mut T {
        self.dirty = true;
        &mut self.value
    }

    /// Read the value and clear the dirty flag if set
    pub fn read(&mut self) -> &T {
        self.dirty = false;
        &self.value
    }

    /// Read the value only if modified since last read, clears the dirty flag
    #[allow(dead_code)]
    pub fn read_dirty(&mut self) -> Option<&T> {
        match self.dirty {
            true => {
                self.dirty = false;
                Some(&self.value)
            },
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dirty;

    #[test]
    fn new_dirty() {
        let dirty = Dirty::new(0);
        assert!(dirty.dirty() == true);
    }

    #[test]
    fn read_clears_flag() {
        let mut dirty = Dirty::new(0);
        assert!(dirty.dirty() == true);
        assert!(*dirty.read() == 0);
        assert!(dirty.dirty() == false);
    }

    #[test]
    fn write_sets_flag() {
        let mut dirty = Dirty::new(0);
        assert!(*dirty.read() == 0);
        assert!(dirty.dirty() == false);
        *dirty.write() += 1;
        assert!(dirty.dirty() == true);
    }

    #[test]
    fn read_dirty() {
        let mut dirty = Dirty::new(0);
        assert!(dirty.read_dirty().is_some());
        assert!(dirty.dirty() == false);
        assert!(dirty.read_dirty() == None);
        assert!(dirty.dirty() == false);
        *dirty.write() += 1;
        assert!(dirty.dirty() == true);
        assert!(dirty.read_dirty().is_some());
        assert!(dirty.dirty() == false);
        assert!(dirty.read_dirty() == None);
        assert!(dirty.dirty() == false);
    }
}
