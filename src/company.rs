use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Department {
    Sales,
    Engineering,
    Design,
    Marketing,
}

impl Department {
    pub fn is_valid(val: Option<&str>) -> bool {
        match val {
            Some("Sales") | Some("Engineering") | Some("Design") | Some("Marketing") => true,
            _ => false,
        }
    }
}

pub type Company = HashMap<Department, Vec<String>>;

pub trait NewCompany {
    fn new() -> Company {
        let mut company: Company = HashMap::new();
        company.insert(Department::Sales, vec![]);
        company.insert(Department::Engineering, vec![]);
        company.insert(Department::Design, vec![]);
        company.insert(Department::Marketing, vec![]);
        company
    }
}

pub trait AddEmployer {
    fn add_employer(&mut self, department: Department, name: &str);
}

impl AddEmployer for Company {
    fn add_employer(&mut self, department: Department, name: &str) {
        self.entry(department)
            .or_insert(vec![])
            .push(name.to_string());
    }
}
