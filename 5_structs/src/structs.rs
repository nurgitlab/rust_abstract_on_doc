#[derive(Debug)]
struct Car {
    wheels: i32,
    name: String,
    model: String,
    fuel_capacity: f64,
    fuel_for_range: f64,
    fuel_current: f64,
    owner: String,
}
impl Car {
    fn new(wheels: i32, name: String, model: String, fuel_capacity: f64, fuel_for_range: f64, fuel_current: f64, owner: String) -> Self {
        Car {
            wheels,
            name,
            model,
            fuel_capacity,
            fuel_for_range,
            fuel_current,
            owner,
        }
    }

    fn add_fuel (&mut self, fuel: f64) {
        let to_full_capacity = self.fuel_capacity - self.fuel_current;
        if fuel <= 0.0 {
            println!("Incorrect value");
        } else if fuel > to_full_capacity {
            self.fuel_current = self.fuel_capacity;
            println!("Very lot fuel, we dont need {} litres", fuel - to_full_capacity);
        } else {
            self.fuel_current+=fuel;

            println!("Added {} litres, at car {} litres", fuel, self.fuel_current);
        }
    }
}

fn main () {
    let mut my_car = Car{
        wheels: 4,
        name: String::from("Toyota"),
        model: String::from("Camry"),
        fuel_capacity: 50.0,
        fuel_for_range: 7.5,
        fuel_current: 37.4,
        owner: String::from("Siegfried"),
    };

    my_car.add_fuel(30.0);

    let volvo = Car::new(4, String::from("Volvo"), String::from("XC90"), 70.0, 8.0, 50.0, String::from("Nuka"));
    println!("{:#?}", my_car);
    println!("{:#?}", volvo);

    let volkswagen = Car{
        name: String::from("Volkswagen"),
        ..volvo
    };

    println!("{:#?}", volkswagen);
}