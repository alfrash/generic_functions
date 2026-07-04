

#[derive(Clone)]
pub struct Employee{
    pub name: String,
    pub age: u32,
}

pub fn get_name(employee: &Employee) -> &str {
    &employee.name
}

pub fn who_is_older<'a>(employee_1: &'a Employee, employee_2: &'a Employee) -> &'a str {
    if employee_1.age > employee_2.age {
        &employee_1.name
    }else {
        &employee_2.name
    }
}