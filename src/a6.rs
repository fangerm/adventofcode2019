use std::{cell::RefCell, collections::HashMap, fs::read_to_string, rc::Rc};

#[derive(Default, Clone)]
struct Object {
    name: String,
    orbiting: Option<Rc<RefCell<Object>>>,
    orbited_by: Vec<Rc<RefCell<Object>>>,
}

pub fn a6() {
    // Key orbits around value
    let mut orbits = HashMap::<String, String>::new();
    let mut objects = HashMap::<String, Rc<RefCell<Object>>>::new();

    for line in read_to_string("inputs/input-6")
        .expect("Failed to read map")
        .lines()
    {
        let mut split = line.split(')');
        let orbiting = split.next().unwrap().to_string();
        let object = split.next().unwrap().to_string();
        orbits.insert(object.to_string(), orbiting);
        objects.insert(
            object.to_string(),
            Rc::new(RefCell::new(Object {
                name: object.to_string(),
                ..Object::default()
            })),
        );
    }
    objects.insert(
        "COM".to_string(),
        Rc::new(RefCell::new(Object {
            name: "COM".to_string(),
            ..Object::default()
        })),
    );

    for (object, orbiting) in orbits {
        objects[&orbiting]
            .borrow_mut()
            .orbited_by
            .push(objects[&object].clone());
        objects[&object].borrow_mut().orbiting = Some(objects[&orbiting].clone());
    }

    println!("{}", deeper(&objects["COM"].borrow(), 0));

    let you = &objects["YOU"];
    println!("{}", deeper_santa(&you.borrow(), 0));
}

fn deeper(obj: &Object, nesting: usize) -> usize {
    let mut count = obj.orbited_by.len();
    count += count * nesting;
    for obj in obj.orbited_by.iter() {
        count += deeper(&obj.borrow(), nesting + 1);
    }
    count
}

fn deeper_santa(you: &Object, dist: usize) -> usize {
    for obj in you.orbited_by.iter() {
        if let Some(depth) = has_santa(&obj.borrow(), 0) {
            return dist + depth;
        }
    }
    deeper_santa(&you.orbiting.as_ref().unwrap().borrow(), dist + 1)
}

fn has_santa(obj: &Object, depth: usize) -> Option<usize> {
    for obj in obj.orbited_by.iter() {
        let res = if obj.borrow().name == "SAN" {
            Some(depth)
        } else {
            has_santa(&obj.borrow(), depth + 1)
        };
        if res.is_some() {
            return res;
        }
    }
    None
}
