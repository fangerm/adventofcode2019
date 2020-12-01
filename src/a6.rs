use std::{collections::HashMap, cell::RefCell, fs::read_to_string, rc::Rc};

#[derive(Default, Clone)]
struct Object {
    orbited_by: Rc<RefCell<Vec<Object>>>,
}

pub fn a6() {
    // Key orbits around value
    let mut orbits = HashMap::<String, String>::new();
    let mut objects = HashMap::<String, Object>::new();

    for line in read_to_string("input-6")
        .expect("Failed to read map")
        .lines()
    {
        let mut split = line.split(')');
        let orbiting = split.next().unwrap().to_string();
        let object = split.next().unwrap().to_string();
        orbits.insert(object.to_string(), orbiting);
        objects.insert(object, Object::default());
    }
    objects.insert("COM".to_string(), Object::default());

    for (object, orbiting) in orbits {
        objects[&orbiting].orbited_by.borrow_mut().push(objects[&object].clone())
    }

    println!("{}", deeper(&objects["COM"], 0));
}

fn deeper(obj: &Object, nesting: usize) -> usize {
    let mut count = obj.orbited_by.borrow().len();
    count += count * nesting;
    for obj in obj.orbited_by.borrow().iter() {
        count += deeper(obj, nesting + 1);
    }
    count
}