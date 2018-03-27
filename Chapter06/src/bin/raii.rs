use std::ops::Deref;

// This represents a low level, close to the metal OS feature that
// needs to be locked and unlocked in some way in order to be accessed
// and is usually unsafe to use directly
struct SomeOsSpecificFunctionalityHandle;

// This is a safe wrapper around the low level struct
struct SomeOsFunctionality<T> {
    // The data variable represents whatever useful information
    // the user might provide to the OS functionality
    data: T,
    // The underlying struct is usually not savely movable,
    // so it's given a constant address in a box
    inner: Box<SomeOsSpecificFunctionalityHandle>,
}

// Access to a locked SomeOsFunctionality is wrapped in a guard
// that automatically unlocks it when dropped
struct SomeOsFunctionalityGuard<'a, T: 'a> {
    lock: &'a SomeOsFunctionality<T>,
}

impl SomeOsSpecificFunctionalityHandle {
    unsafe fn lock(&self) {
        // Here goes the unsafe low level code
    }
    unsafe fn unlock(&self) {
        // Here goes the unsafe low level code
    }
}

impl<T> SomeOsFunctionality<T> {
    fn new(data: T) -> Self {
        let handle = SomeOsSpecificFunctionalityHandle;
        SomeOsFunctionality {
            data,
            inner: Box::new(handle),
        }
    }

    fn lock(&self) -> SomeOsFunctionalityGuard<T> {
        // Lock the underlying resource.
        unsafe {
            self.inner.lock();
        }

        // Wrap a reference to our locked selves in a guard
        SomeOsFunctionalityGuard { lock: self }
    }
}

// Automatically unlock the underlying resource on drop
impl<'a, T> Drop for SomeOsFunctionalityGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.lock.inner.unlock();
        }
    }
}

// Implementing Deref means we can directly
// treat SomeOsFunctionalityGuard as if it was T
impl<'a, T> Deref for SomeOsFunctionalityGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.lock.data
    }
}

fn main() {
    let foo = SomeOsFunctionality::new("Hello World");
    {
        // Locking foo returns an unlocked guard
        let bar = foo.lock();
        // Because of the Deref implementation on the guard,
        // we can use it as if it was the underlying data
        println!("The string behind foo is {} characters long", bar.len());

        // foo is automatically unlocked when we exit this scope
    }
    // foo could now be unlocked again if needed
}
