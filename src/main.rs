fn main() {
//Strings

let name = "Mbombela";
let enye_into = String::from("Iyana imvula namhlanje ");
println!("Your name is {}", name);
println!("Kodwa ke namhlanje kwenzeka oku -> {}", enye_into);

println!("Hello, world!");


// Showing that 'str' is dynamically sized (heap implications / 'hotel califonia mindset')

println!("A string is always {:?} bytes. It is Sized.", std::mem::size_of::<String>());
println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
println!("But a &str? It can be anything. 'Mbombela' is {:?} bytes . It is not Sized", std::mem::size_of_val("Mbombela"));
println!("And 'Andile, Iyana namhlanje imvula' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Andile, iyana namhlanje imvula"));
println!("Kodwa '.' is {:?} bytes . It is not sized.", std::mem::size_of_val(&'.'));

// One way of making a String
// fn main()..{

let my_name = "Zikhona_Zonke";
let my_khantri = "Mzantsi";
let my_home = "VV";

let together = format! (
    " Igama lamm ngu {} yaye ndingumi wase {} kodwa ndihlala e {}",
    my_name, my_khantri, my_home,

);

let my_string: String = "Try to make this a String".into();
println!("Okay then {}", my_string);

//}

// More on references

// fn main() {

    let country = String::from("South Africa");
    let kwenzeka = String::from("Iqale phi?");

    let ref_one = &country;
    let ref_two = &country;

    let kwenzeka = &kwenzeka;

    println!("{}", ref_one);
    println!("{}", ref_two);
    println!("{}", kwenzeka);
//}

// Mutable references

// fn main() {

    let mut my_number = 9;
    let num_ref = &mut my_number;

// So, how do you add to 'my_number' ?

    let mut my_number = 9;
    let num_ref = &mut my_number;
    *num_ref += 10; // Using '*' to change the i32 value.
    println!("{}", my_number);

    let second_number = 7001;
    let triple_reference = &&&second_number;
    println!("Second_number = triple_reference? {}", second_number == ***triple_reference);
     

    let mut number1 = 10;
    let number_change = &mut number1; // create a mutable reference
    *number_change += 11; // use mutable reference to add 11

    let nom_ref = &number1; // create an immutable referefence
    println!("{}", nom_ref);
     // print the immutable reference


// Shadowing - this does NOT destry a value, but blocks it

    let country = String::from("Angola"); // Now we have a String caled country
    let country_ref = &country; // country_ref is a reference to this data. It's not gonna change
    let country = 80; // Now, we have a variable called country that is an i8. But it has no relation to the other one, or to country_ref
    println!("{}, {}", country_ref, country); // country_ref still refers to the data of String::from("South Africa") that we gave


// Make print_countri

    fn print_countri(igama_lelizwe: &String) {
        println!("{}", igama_lelizwe);
        
    }

    let ilizwe = String::from("Zanzibar");
    print_countri(&ilizwe); // we print Zanzibar
    print_countri(&ilizwe); // Again...


// Doing something similar with a mutable reference

    fn add_hungary(counti_name: &mut String) { // first, we say the function takes a mutable reference
        counti_name.push_str("-Hungary"); // push_str() adds a &str to a String
        println!("Now it says: {}", counti_name);
    }

// fn main() {
        let mut kanti = String::from("Qokolweni");
        add_hungary(&mut kanti); // we also need to give it a mutable reference.

//  }

//}

// Give references to functions

     fn print_country(country_name: String) -> String {
        println!("{}", country_name);
        country_name
}

//fn main() {
    let country = String::from("Wakanda");
    let country = print_country(country);
    print_country(country);

//}

// DOING the SAME thing better with REFERENCES (Borrowing)

    fn print_khhantri(khantri_name: &String) {
        println!("{}", khantri_name);
}

    //fn main() {
        let ikhantri = String::from("Mbizana");
        print_khhantri(&ikhantri);
        print_khhantri(&ikhantri);
//}

// COPY types - resides in the stack n and small - copy types incl (intn floats, booleans, and char)
    fn prints_a_number(number: i32) {
        println!("{}", number);
    }

    //fn main() {
        let my_num = 9;
        prints_a_number(my_num);
        prints_a_number(my_num);
    //}

// Using CLONE  to send it to the function, so the 'cntry' variable is still alive

    fn prints_cntry(cntry_name: String) {
        println!("{}", cntry_name);   

    }

    //fn main() {
    let cntry = String::from("Kwazakhele");
    prints_cntry(cntry.clone());
    prints_cntry(cntry);

    //}

// CLONE takes a lot of memory(if its a book, one String can be a book
// // CLONE then returns a book everytime its called) 

//     fn get_length(input: String, iteration: usize) { // owns the String
//         println!("Iteration {}: It's {} words long.", iteration, input.split_whitespace().count()); // splits to counts number of words

//     }

//     //fn main() {
//         let mut my_sstring = String::new();
//         for i in 0..10 {
//             my_sstring.push_str("Here are some more words"); // push the words on
//             get_length(my_sstring.clone(), i); // gives it a clone everytime
//     // }
//     } 

// a better VERSION of the above code with a '&' reference

    fn get_length(input: String) {
        println!("It's {} words long", input.split_whitespace().count());
    }

    //fn main() {
        let mut my_thing = String::new();
        for _ in 0..15 {
            my_thing.push_str("Nantso ke inkathazo");
            get_length(my_thing.clone());
    //    }
}

// Variables WITHOUT values

    fn loop_then_return(mut counter: i32) -> i32 {
        loop {
            counter += 1;
            if counter % 50 == 0 {
                break;
            }
        }
        counter
    }

    //fn main() {
        let my_nombolo;

        {
            my_nombolo = 100;           
            
        }
        println!("{}", my_nombolo);
    //}

// Collection types - ARRAYS

    //fn main() {
        let my_array = ["a"; 10];
        println!("{:?}", my_array);
    //}

// Indexing an Array

    //fn main() {
        let my_numbers = [1, 13, -34];
        println!("{}", my_numbers[1]);
    //}

// Slicing an ARRAY with '&'

    //fn main() {
        let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let three_to_five = &array_of_ten[2..5];
        let start_at_two = &array_of_ten[1..];
        let end_at_five = &array_of_ten[..5];
        let everything = &array_of_ten[..];

        println!("Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}", three_to_five, start_at_two, end_at_five, everything );
    //}

 // Vectors 
        
    //fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesi");

    //let mut my_vec: Vec<String> = Vec::new();
    // my_vec.push(name1);
    // my_vec.push(name2);

    //}

// Slicing vectors

    //fn main() {
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Everything is the same as the above except we added vec!
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything =&vec_of_ten[..];

    println!("Three to five: {:?},
start_at_two: {:?}
end_at_five: {:?}
everything: {:?}", three_to_five, start_at_two, end_at_five, everything);
//}

// Adding CAPACITY to a Vector

// fn main() {
    let mut numm_vec = Vec::new();
    println!("{}", numm_vec.capacity()); // 0 elements: prints 0
    numm_vec.push('a'); // add one character
    println!("{}", numm_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always starts with capacity of 4
    numm_vec.push('a'); // add one more
    numm_vec.push('a'); // add one more
    numm_vec.push('a'); // add one more
    println!("{}", numm_vec.capacity()); // 4 elements: still prints 4
    numm_vec.push('a'); // add one more
    println!("{}", numm_vec.capacity()); // pints 8. We have 5 elements, but it doubles from 4 to 8 to create space.
//}

// HOW to make this Vector FASTER?

//fn main() {
    let mut numm_vecc = Vec::with_capacity(8); // give it a capacity of 8
    println!("{}",numm_vecc.capacity()); // prints 8
    numm_vecc.push('a'); // add one more
    println!("{}", numm_vecc.capacity()); // prints 8
    numm_vecc.push('a'); // add one more
    println!("{}", numm_vecc.capacity()); // prints 8
    numm_vecc.push('a'); // add one more
    numm_vecc.push('a'); // add one more // Now we have 5 elements
    println!("{}", numm_vecc.capacity()); // Still 8


// This vector has 0 reallocations, which is better. So if you think you know how many elements you need, 
// you can use Vec::with_capacity() to make it faster.

//You remember that you can use .into() to make a &str into a String. 
// You can also use it to make an array into a Vec. You have to tell .into() that you want a Vec, but you don't have to choose the type of Vec. If you don't want to choose, you can write Vec<_>.    

// EXAMPLE:

// fn main() {
    let me_vec: Vec<u8> = [1,2, 3].into();
    let me_vec2: Vec<_> = [9, 0, 10].into();

//}

//}

// Tuples 

//fn main() {
    let random_tuple = ("Nali igama lomntu", 8, vec!['a'], 'b', [8, 10, 100], 7.1 );
    println!(
        "Inside the tuple is:\n\
First item:{:?}\n\
Second item: {:?}\n\
Third item: {:?}\n\
Fourth item: {:?}\n\
Fifth item: {:?}\n\
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5, 

    ); 

// This tuple is of type (&str, i32, Vec<char>, char, [i32; 3], f64)

// Using a tuple to create multiple variables

//fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // call them a, b, and c
    println!("{:?}", b); 
    
    // this is called 'destructuring'
//}

//}

// Destructuring WITHOUT ALL the variables by using '_'.

//fn main() {
    let str_vecc = vec!["one", "two", "three"];

    let (_, _, variables) = (str_vecc[0], str_vecc[1], str_vecc[2]);
//}

// 25. Structs

struct Colour(u8, u8, u8);
//fn main() {
    let the_colour = Colour(50, 0, 50); // Make an RGB colour
    println!("The second part of the colour is {}", the_colour.1);
        
//}

struct Khala(u8, u8, u8);

struct SizeAndKhala {
    size: u32,
    colour: Khala, // we put it in the newly named struct
}

//fn main() {
    let me_khala = Khala(50, 0, 50);

    let size_and_Klr = SizeAndKhala {
        size: 150,
        colour: me_khala
    };
//}

#[derive(Debug)]
struct Cowntry {
    population: u32,
    capital: String,
    leader_name: String
}

//fn main() {
    let population = 700_111;
    let capital = String::from("Emnyango");
    let leader_name = String::from("Bantu BonkeZonke");

    let kalanga = Cowntry {
        population,
        capital,
        leader_name,       

    };

    println!("please print {:?}", kalanga);
//}

// Enums, looks like Structs, but are different
// The diff is: you use a struct when you want to do one thing AND another thing
// You use an enum when you want one thing OR another thing
// Structs are for MANY THINGS together, while enums are for MANY CHOICES together

enum ThingsInTheSky {
    Sun,
    Stars,
}

//fn main () { an enum bcs you see the Sun OR the Stars - have to choose one

//}
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun, // btwn 6 and 18 hours we can see the Sun
        _ => ThingsInTheSky::Stars, // Otherwise, we see the Stars
    }
} // With this function, we can match against the two choices in ThingsInTheSky.
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the Sun"),
        ThingsInTheSky::Stars =>println!("I can see the Stars")
    }
}
//fn main() {
    let time = 8; // it's 8 o'clock
    let skystate = create_skystate(time); // create_skaystate returns a ThingsInTheSky
    check_skystate(&skystate); // Give it a reference so it can read the variable skystate
//}

// Implementing Structs and enums

//- This is where you can start to give structs and enums some real power. To call functions on a struct or an enum, use an 'impl' block. These functions are called 'METHODS'
//- Methods come in 2 kinds: 
//* Methods: these take 'self'(or '&self' or '&mut self'). Regular methods use a '.' (a dot). '.clone()' is an example of a regular method.
//* Associated functions (known as 'static' methods in some langs): these do not take 'self'. associated means "related to". They are written differently, using "::". String::from() is an associated function, 
//so is Vec::new(). YOu see associated functions most often used to create new variables.

//- For a new struct or enum, you neeed to give it DEBUG if you want to use '{:?}' to print.
//- If you write '#[derive(Debug)]' above the struct or enum then you can print it with "{:?}". 
//- These messages with "#[]" are called attributes. You can sometimes use them to tell the compiler to to give your struct an ability like Debug.
//- There are many attributes - derive is the most common.



#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Rabit,
}

impl Animal {
    fn new() -> Self {
        // Self means Animal.
        // You can also write Animal instead of Self

        Self {
        // When we write Animal::new(), we always get a cat that is 10 yo
        age: 10,
        animal_type: AnimalType::Cat,
        }    
    }

    fn change_to_rabit(&mut self) { // bcs we are inside Animal, &mut self means &mut Animal
                                    // use .change_to_rabit() to change the cat to rabit
                                    // with &mut self, we can change it
        println!("Changing animal to rabit");
        self.animal_type = AnimalType::Rabit;
    }

    fn change_to_cat(&mut self) {
        // use .change_to_cat() to change from rabit to cat
        // with &mut self we can change it
        println!("Changing animal to cat");
        self.animal_type = AnimalType::Cat;       
    }

    fn check_type(&self) {
        // we want to read self

        match self.animal_type {
            AnimalType::Cat => println!("The animal is a cat"),
            AnimalType::Rabit => println!("The animal is a rabit"),
        }
    }
}



//fn main() {
    let mut new_animal = Animal::new(); // Associated function to create a new animal
                                        // it is a cat, 10 yrs old
    new_animal.check_type();
    new_animal.change_to_rabit();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type(); 
//}

}