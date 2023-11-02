// Defining a struct adn the associated fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // we are using the field init shorthand to avoid writing username:username, the parameter/field are identical
        email,
        sign_in_count: 1,
    }
}

// tuple struct are useful when you don't need field names and want to distinguish between tuples by naming them
struct Point(i32, i32);

fn main() {
    let mut user1 = User {
        // the instance doesn't have to be mutable, but if not then we can't change field values
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email); // can't format struct field access inside braces in string print. needs to be like this
    user1.email = String::from("someone@example.net");
    println!("{}", user1.email);

    // using build_user function to create a new instance of the User struct
    let user2 = build_user(
        String::from("thisperson@yahoo.com"),
        String::from("anotherusername"),
    );

    // Struct update syntax allows for copying most values from existing instance when creating new instance
    let user3 = User {
        username: String::from("newusername123"),
        ..user1 // no comma here, since base truct must be the last field!
    };

    // note that we can't use all of user1 after using it in the creation of user3, as the String email is moved into user3, not copied.
    // we can only use copyable values, which still exist after we define user3: e.g. user1.active is still available to us

    println!("{}", user1.active);
    println!("{}", user2.email);
    println!("{}", user3.username);

    let tuple_struct1 = Point(0, 1);
    // can access elements like in a tuple, by using .index notation
    println!("{}", tuple_struct1.0);

    
}
