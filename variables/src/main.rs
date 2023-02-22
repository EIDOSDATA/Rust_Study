// 덮어쓰기
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // 덮어쓰기
    let spaces = "   ";
    let spaces = spaces.len();

    // MUT >> Error
    //let mut spaces = "   ";
    //spaces = spaces.len();
}

/*
// 상수 : 기본적으로 불변인것이 아니고, 항상 불변이다.
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
*/

/*
// 변수의 가변성 : 기본적으로 불변성을 가진 변수를 가변성 있게 바꾼다.
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
*/

/*
// 변수의 불변성 : 기본적으로 변수 선언시 불변의 특성을 가진다.
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
*/
