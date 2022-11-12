struct People {
    number: u32,
    surname: String,
    name: String,
    patronymic: String,
    age: u8,
    sex: String,
}

impl People {
    fn is_old(&self) -> bool {
        self.age > 21  
    }
}


fn main() {
    let people1 = People {
        number: 1,
        surname: String::from("Pypkin"),
        name: String::from("Vasya"),
        patronymic: String::from("Ivanovich"),
        age: 23,
        sex: String::from("male"),
    };
    
    let people2 = People {
        number: 2,
        surname: String::from("Zalypkin"),
        name: String::from("Oleg"),
        patronymic: String::from("Eduardovich"),
        age: 20,
        sex: String::from("male"),
    };
    
    let people3 = People {
        number: 3,
        surname: String::from("Eskere"),
        name: String::from("Elena"),
        patronymic: String::from("Cringovna"),
        age: 28,
        sex: String::from("female"),
    };

    let ppl = [people1, people2, people3];

    for num in ppl {
        if num.is_old() {
            println!("People {} is {} {} {}, age:{}, sex:{}", num.number, num.surname, num.name, num.patronymic, num.age, num.sex);
        } else {
            println!("People {} is {} {}, age:{}, sex:{}", num.number, num.surname, num.name, num.age, num.sex);
        }
        
    }

}
