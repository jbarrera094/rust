use std::collections::{HashMap, HashSet};

fn main() {
    // This is a comment
    /*
    This is a multi-line comment
    */
    println!("Hello, world!");

    // region Variables
    let mut my_string = "Hello, world!";
    println!("Variable: {}", my_string);
    my_string = "Vamosa cambiar el valor";
    println!("{my_string}");

    // my_string = 6; // Error

    let my_string2 = String::from("Esta es otra cadena de texto");
    println!("{my_string2}");

    let mut my_int = 7;
    my_int = my_int + 5;
    println!("{my_int}");
    println!("{}", my_int - 1);

    println!("Este es el valor de my_int: {}", my_int);

    let my_int64: i64 = 7;
    println!("{my_int64}");

    let my_float = 2.5;
    println!("{my_float}");
    // my_float = my_float + my_int; // Error

    let mut my_bool = false;
    println!("{my_bool}");
    my_bool = true;
    println!("{my_bool}");

    // Constantes
    const MY_CONST: &str = "Mi propiedad constante";
    println!("{MY_CONST}");

    // Control de flujo
    if my_int == 10 && my_string == "Hola" {
        println!("El valor es 10")
    } else if my_int == 12 || my_string == "Hola" {
        println!("El valor es 12")
    } else {
        println!("El valor no es 10")
    }

    // Lista
    let mut my_list = vec!["Jeisson", "Barrera", "jbarrera094", "@json094"];
    my_list.push("Sanchez");
    println!("{:?}", my_list);
    println!("{}", my_list[0]);
    println!("{}", my_list[my_list.len() - 1]);

    // Sets
    let mut my_set: HashSet<&str> = vec!["Jeisson", "Barrera", "jbarrera094", "@json094"]
        .into_iter()
        .collect();
    my_set.insert("Jeisson");
    println!("{:?}", my_set);

    // Mapas
    let mut my_map: HashMap<&str, i32> = vec![("Jey", 24), ("Juan", 34), ("Pepito", 42)]
        .into_iter()
        .collect();
    my_map.insert("Luis", 28);
    println!("{:?}", my_map);

    // Bucles
    for value in &my_list {
        println!("{value}")
    }

    for value in &my_set {
        println!("{value}")
    }

    for (key, value) in &my_map {
        println!("{} {}", key, value)
    }

    let mut my_counter = 0;
    while my_counter < my_list.len() {
        println!("{}", my_list[my_counter]);
        my_counter += 1;
    }

    // Funciones
    my_function();

    // Estructuras
    let my_person = Person::new("Jey", 24);
    println!("{} {}", my_person.name, my_person.age)
}

fn my_function() {
    println!("Esto es una función");
}

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }
}
