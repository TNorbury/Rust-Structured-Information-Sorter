use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


struct Person 
{
    name: String,
    age: usize, //Make age of usize type for consitency with functions such as len
}

fn main() 
{
    //Create a vector that will contain all the people.
    let mut people = Vec::new();
    let mut fileLine;
    let mut ageSum: usize = 0;

    //Get the file from the command line and try to open it.
    let personFile: File = File::open(std::env::args().nth(1).unwrap())
        .unwrap();
    
    //Create a reader that will read the file
    let fileReader = BufReader::new(personFile);

    //Iterate through the lines in the file
    for line in fileReader.lines()
    {

        //Have to use the fileLine variable to unwrap the line, otherwise the
        //program won't compile.
        fileLine = line.unwrap();

        //Split each line on ',' and create an iterator for the other tokens.
        let mut lineIter = fileLine.split(",");

        //Insert a new person structe into the people vector.
        people.insert(0, Person {
                name: lineIter.next().unwrap().to_string(),
                age: lineIter.next().unwrap().trim().parse()
                       .expect("The age of a person must be a positive number")
            }
        );
    }

    //Order the vector based on the names of people.
    people.sort_by(|a, b| a.name.cmp(&b.name));

    //Iterate through the B-Tree and print out all of the names.
    for person in people.iter() 
    {
        //Print out everybody's name
        println!("{}", person.name);

        //Make a sum of all the ages.
        ageSum += person.age;
    }

    //Print out the average age.
    println!("Average Age: {}", (ageSum/people.len()));
}
