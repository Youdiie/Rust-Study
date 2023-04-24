trait Animal {
    // 선언부만 정의
    fn custom_bark(&self);

    // 선언부, 구현부 정의
    fn common_bark(&self) {
        println!("Common Bark");
    }
}

struct Dog {
    Pomeranian : String,
    Poodel : String,
    Dashshund : String,
}

impl Animal for Dog {
    // common_bark는 Animal 트레잇에서 구현되어 있기 때문에 Dog에서 따로 구현하지 x
    fn custom_bark(&self){
        println!("Pomeranian Bark : {}", self.Pomeranian);
        println!("Poodel Bark : {}", self.Poodel);
        println!("Dashshund Bark : {}", self.Dashshund);
    }
}

fn main(){
    let dog = Dog{
        Pomeranian : String::from("낑낑"),
        Poodel : String::from("멍멍"),
        Dashshund : String::from("댕댕"),
    };

    dog.common_bark();
    dog.custom_bark();
}