struct Celcius {
    temperature: f64,
}

impl Celcius {
    fn new(temperature: f64) -> Celcius {
        Celcius { temperature }
    }

    fn to_fahrenheit(&self) -> f64 {
        self.temperature * 1.8 + 32.0
    }
}

struct Fahrenheit {
    temperature: f64,
}

impl Fahrenheit {
    fn new(temperature: f64) -> Fahrenheit {
        Fahrenheit { temperature }
    }

    fn to_celcius(&self) -> f64 {
        (self.temperature - 32.0) / 1.8
    }
}

fn main() {
    // Taking input from user in Celcius
    let celcius = Celcius::new(36.0);
    println!("Temperature in Celcius: {}", celcius.temperature);
    println!("Temperature in Fahrenheit: {}", celcius.to_fahrenheit());
    let fahrenheit = Fahrenheit::new(98.6);
    println!("Temperature in Fahrenheit: {}", fahrenheit.temperature);
    println!("Temperature in Celcius: {}", fahrenheit.to_celcius());
}
