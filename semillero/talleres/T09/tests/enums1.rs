// I AM NOT DONE

// código incorrecto

/*#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);    
    }

}*/




// Para hacer que funcione, se definen cada una de las variantes de la enumeración
// desde la línea 28 a la 31
#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);    
    }

}

