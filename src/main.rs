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
        println!("{}", my_nombolo)
    //}




}
