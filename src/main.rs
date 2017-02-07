use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::io::BufReader;
use std::fs::File;


#[derive(Eq)]
struct Person 
{
    name: String,
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
    let mut fileLine;

    //Get the file from the command line and try to open it.
    let fileLocation: String = std::env::args().nth(1).unwrap();
    let personFile: File = File::open(std::env::args().nth(1).unwrap())
        .unwrap();
    
    //Create a reader that will read the file
    let fileReader = BufReader::new(personFile);

    //Iterate through the lines in the file
    for line in fileReader.lines()
    {

        //Have to use the fileLine vriable to unwrap the line, otherwise the
        //program won't compile.
        fileLine = line.unwrap();

        //Create an iterator over the tokens on the line.
        let mut lineIter = fileLine.split(",");

        //Insert a new person structer into the B-Tree Set.
        people.insert(Person {
                name: lineIter.next().unwrap().to_string(),
                age: lineIter.next().unwrap().trim().parse()
                       .expect("The age of a person must be a positive number")
            }
        );
    }

    //Iterate through the B-Tree and print out all of the names.
    for person in people.iter() {
        println!("{}", person.name);
    }

}
