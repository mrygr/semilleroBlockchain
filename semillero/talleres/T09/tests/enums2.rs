// I AM NOT DONE

// Código incorrecto

/* 
#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}*/



// Para hacer que funcione, se definen cada una de las variantes de la enumeración
// desde la línea 39 a la 42
// teniendo en cuenta los tipos de mensajes que va a recibir el enum desde la función main

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move {x:i64, y:i64},
    Echo (String),
    ChangeColor (i64, i64, i64),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
