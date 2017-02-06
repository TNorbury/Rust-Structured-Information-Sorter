use std::collections::BTreeSet;
use std::cmp::Ordering;


#[derive(Eq)]
struct Person 
{
    name: &'static str,
    age: u8, //Assuming people won't be older than 255 or have a negative age.
}


//Implement functionality for comparing Person structs to one another. This is
//required in order to put Persons into an order collection (like a BTreeSet).
impl Ord for Person 
{

    //This will order persons by their name only.
    fn cmp(&self, other: &Person) -> Ordering 
    {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Person
{
    fn partial_cmp(&self, other: &Person) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person
{
    fn eq(&self, other: &Person) -> bool
    {
        self.name == other.name
    }
}

fn main() 
{
    //Create a BTreeSet (this will ensure that the collection is ordered)
    //that will contain all the people.
    let mut people = BTreeSet::new();

    let mut person1 = Person {name: "Tyler Norbury", age: 20};
    let mut person2 = Person {name: "Dylan Norbury", age: 17};
    let mut person3 = Person {name: "Hudson Norbury", age: 5};


    people.insert(person1);
    people.insert(person2);
    people.insert(person3);

    for person in people.iter() 
    {
        println!("{}", person.name);
    }
}
