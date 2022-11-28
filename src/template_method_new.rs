pub trait Display {
    fn open(&self);
    fn print(&self);
    fn close(&self);
}

pub enum DisplayType {
    Char(char),
    String(String),
}

impl DisplayType {
    pub fn print_line(&self) {
        match self {
            DisplayType::Char(_) => {}
            DisplayType::String(s)=> {
                print!("+");
                for _ in 0..s.len() {
                    print!("-");
                }
                println!("+");
            }
        }
    }
}

impl Display for DisplayType {
    fn open(&self) {
        match self {
            DisplayType::Char(_) => print!("<<"),
            DisplayType::String(..) => self.print_line(),
        }
    }

    fn print(&self) {
        match self {
            DisplayType::Char(c) => print!("{}", c),
            DisplayType::String(s) => println!("|{}|", s),
        }
    }

    fn close(&self) {
        match self {
            DisplayType::Char(_) => println!(">>"),
            DisplayType::String(..) => self.print_line(),
        }
    }
}

pub struct Template<T> {
    display: T,
}

impl<T: Display> Template<T> {
    pub fn new(display: T) -> Self {
        Self { display }
    }

    pub fn display(&self) {
        self.display.open();
        for _ in 0..5 {
            self.display.print();
        }
        self.display.close();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let d1 = DisplayType::Char('H');
        let d2 = DisplayType::String("Hello,world.".to_owned());

        Template::new(d1).display();
        Template::new(d2).display();
    }
}