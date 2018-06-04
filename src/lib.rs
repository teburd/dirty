use std::ops::Deref;

/// Dirty wraps a value of type T with functions similiar to that of a Read/Write
/// lock but simply sets a dirty flag on write(), reset on clear().
/// Use read() or deref (*dirty_variable) to access the inner value.
pub struct Dirty<T> {
    value: T,
    dirty: bool,
}

impl<T> Dirty<T> {
    /// Create a new Dirty.
    pub fn new(val: T) -> Dirty<T> {
        Dirty {
            value: val,
            dirty: true,
        }
    }

    /// Returns true if dirty, false otherwise.
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Writable value return, sets the dirty flag.
    pub fn write(&mut self) -> &mut T {
        self.dirty = true;
        &mut self.value
    }

    /// Read the value.
    pub fn read(&self) -> &T {
        &self.value
    }
    
    /// Clears the dirty flag.
    pub fn clear(&mut self) {
        self.dirty = false;
    }

    /// Read the value only if modified since last read.
    pub fn read_dirty(&self) -> Option<&T> {
        match self.dirty {
            true => {
                Some(&self.value)
            },
            false => None,
        }
    }
}

impl<T> Deref for Dirty<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> Default for Dirty<T> where T: Default {
    fn default() -> Self {
        Dirty::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::Dirty;

    #[test]
    fn new_dirty() {
        let dirty = Dirty::new(0);
        assert!(dirty.dirty());
    }

    #[test]
    fn read_doesnt_clear_flag() {
        let dirty = Dirty::new(0);
        assert!(dirty.dirty());
        assert!(*dirty.read() == 0);
        assert!(dirty.dirty());
    }

    #[test]
    fn write_sets_flag() {
        let mut dirty = Dirty::new(0);
        assert!(*dirty.read() == 0);
        dirty.clear();
        assert!(!dirty.dirty());
        *dirty.write() += 1;
        assert!(dirty.dirty());
    }

    #[test]
    fn read_dirty() {
        let mut dirty = Dirty::new(0);
        assert!(dirty.read_dirty().is_some());
		dirty.clear();
        assert!(!dirty.dirty());
        assert!(dirty.read_dirty() == None);
        assert!(!dirty.dirty());
        *dirty.write() += 1;
        assert!(dirty.dirty());
        assert!(dirty.read_dirty().is_some());
		dirty.clear();
        assert!(!dirty.dirty());
        assert!(dirty.read_dirty() == None);
    }
    
    #[test]
    fn access_inner_deref() {
        let dirty = Dirty::new(0);
        assert!(*dirty == 0);
    }
    
    #[test]
    fn default_value() {
        let dirty = Dirty::<i32>::default();
        assert!(*dirty == 0);
    }
}
