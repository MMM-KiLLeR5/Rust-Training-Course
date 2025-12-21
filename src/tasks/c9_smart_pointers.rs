// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

use std::cell::RefCell;
use std::rc::Rc;

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        BinaryTreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode) -> Self {
        BinaryTreeNode {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn sum(&self) -> i32 {
        let left_sum = self.left_child.as_ref().map_or(0, |node| node.sum());
        let right_sum = self.right_child.as_ref().map_or(0, |node| node.sum());
        self.value + left_sum + right_sum
    }
}
// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Package>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: Package) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

#[derive(Clone)]
pub struct Package {
    name: String,
    dependencies: Rc<RefCell<Vec<Rc<Package>>>>,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            name: name.to_string(),
            dependencies: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Self {
        Package {
            name: name.to_string(),
            dependencies: Rc::new(RefCell::new(dependencies)),
        }
    }

    pub fn list_dependencies(package: Rc<Package>) -> Vec<String> {
        let mut result = Vec::new();
        result.push(package.name.clone());

        for dep in package.dependencies.borrow().iter() {
            result.append(&mut Self::list_dependencies(dep.clone()));
        }

        result
    }
}

#[test]
fn test_list_dependencies() {
    let package_a = Rc::new(Package::new("A"));
    let package_b = Rc::new(Package::new("B"));
    let package_c = Rc::new(Package::new("C"));

    let mut package_d = Package::with_dependencies("D", vec![package_a.clone(), package_b.clone()]);
    package_d.dependencies.borrow_mut().push(package_c.clone());

    let dependencies = Package::list_dependencies(Rc::new(package_d));
    
    assert_eq!(dependencies, vec!["D", "A", "B", "C"]);
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

// IMPLEMENT HERE:
pub struct SharedCounter {
    value: RefCell<i32>,
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter {
            value: RefCell::new(0),
        }
    }

    pub fn increment(&self) {
        let mut value = self.value.borrow_mut();
        *value += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
