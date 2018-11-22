mod core;

struct Person {
   name : String,
   age : i16,
}

impl Person {
    fn answer(&self) -> String {
        self.name.to_string() + " " + &self.age.to_string()
    }

    fn birthday(&mut self) {
        self.age += 1
    }
}

fn main() {
    let x = 10;
    let mut y = 20;

    core::increase(&mut y, 10);

    println!("{}", x + y);

    let arr = vec![1,2,3,4];
    for i in &arr {
        println!("ref: {}", i)
    }

    let p = Person{name: "steve".to_string(), age: 17 };
    println!("person: {} {}", p.name, p.age);

    let arrP = vec![
        Person{name: "erny".to_string(), age: 18},
        Person{name: "batty".to_string(), age: 13},
    ];

    for i in &arrP {
        //i.birthday();
        println!("ref person: name: {}; speak: {}", i.name, i.answer());
    }
}
/*

fn increase(y : &mut i32, n : i32) {
     for i in 0..n {
         *y += i
     }
}
*/
