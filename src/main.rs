struct User {
    name: u32,
    age: u32,
    height : f64
}
struct alwaysEqual;
fn main() {
    let user1 : User = User {
        name :3, 
        age : 30,
        height : 1.75
    };
    let data : alwaysEqual = alwaysEqual;
    
    let user2 = User {
        ..user1
    };
    println!("{} is {} years old and {} meters tall", user1.name, user1.age, user1.height);
    println!("{} is {} years old and {} meters tall", user2.name, user2.age, user2.height);
}
