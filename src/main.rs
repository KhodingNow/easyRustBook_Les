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

    fn print_countri(igama_lelizwe: String) -> String {
        println!("{}", igama_lelizwe);
        igama_lelizwe // return it here
    }

    let ilizwe = String::from("Zanzibar");
    let ilizwe = print_countri(ilizwe); // we have to use let here to get the String back
    print_countri(ilizwe); 
//}

}
