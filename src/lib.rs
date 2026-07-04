#[derive(Clone)]
pub struct Employee {
    pub name: String,
    pub age: u32,
}

pub fn get_name(employee: &Employee) -> &str {
    &employee.name
}

pub fn who_is_older<'a>(employee_1: &'a Employee, employee_2: &'a Employee) -> &'a str {
    if employee_1.age > employee_2.age {
        &employee_1.name
    } else {
        &employee_2.name
    }
}

#[derive(Debug)]
pub enum Either<'a, 'b> {
    This(&'a str),
    That(&'b str),
}

pub fn either_is_older<'a, 'b>(
    employee_1: &'a Employee,
    employee_2: &'b Employee,
) -> Either<'a, 'b> {
    if employee_1.age > employee_2.age {
        Either::This(&employee_1.name)
    } else {
        Either::That(&employee_2.name)
    }
}
