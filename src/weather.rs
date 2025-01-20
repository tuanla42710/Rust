use std::io::{self, Write}; 

#[derive(Debug)]
enum WeatherCondition {
    Sunny {weather : String},
    Cloudy {weather : String},
    Rainy {weather : String},
    Snowy {weather : String}
}

impl WeatherCondition {
    fn new(weather : String) -> Self {
        match weather {
            weather if weather == "sunny".to_string() => {WeatherCondition::Sunny {  weather }},
            weather if weather == "cloudy".to_string() => {WeatherCondition::Cloudy {  weather}},
            weather if weather == "rainy".to_string() => {WeatherCondition::Rainy  {  weather }},
            _ => {WeatherCondition::Snowy { weather }}
        }
    }  
}

struct Weather {
    temperature : f64,
    humidity : u32,
    condition : WeatherCondition
}

impl Weather {
    fn new(temperature : f64, humidity : u32, weather_condition : String)-> Weather{
        let condition = WeatherCondition::new(weather_condition);
        Weather { temperature, humidity, condition }
    }
    fn update_temperature(&mut self,temperature : f64){
        self.temperature = temperature;
    }
    fn update_humidity(&mut self, humidity : u32){
        self.humidity = humidity;
    }
    fn condition(&mut self, condition : WeatherCondition){
        self.condition = condition;
    }
    fn display(&self){
        println!("Temperature is {}", self.temperature);
        println!("Humidity is {}", self.humidity);
        println!("Condition is {:?}", self.condition);
    }  
}

struct CityWeather {
    city : String,
    weather : Weather
}

impl CityWeather {
    fn new (city: String, temperature : f64, humidity : u32, weather_condition : String)->Self{
        let weather = Weather::new(temperature , humidity , weather_condition );
        CityWeather {city, weather}
    }
    fn update_weather(&mut self,weather : Weather){
        self.weather = weather;
    }
    fn display(&self){
        println!("weather condition in the city {}", self.city);
        self.weather.display();
    }
}

struct WeatherStation {
    cities : Vec<CityWeather>
}

impl WeatherStation {
    fn new()-> WeatherStation {
        WeatherStation {cities : Vec::new()}
    }
    fn add_city(&mut self,city : String, temperature : f64, humidity : u32, weather_condition : String){
        let city_weather = CityWeather::new(city, temperature,humidity,weather_condition);
        self.cities.push(city_weather);
    }
    fn find_city(&self, city : String){
        for i in &self.cities {
            if i.city == city {
                i.display();
                return;
            }
        }
        println!("We dont have the weather of {}",city);

    }
    fn list_cities(&self){
        for i in &self.cities {
            print!("{} ,", i.city);
        }
    }
    
}

fn display(){
    println!("This is weather for city");
    println!("- press 1 if you want to add information for a city");
    println!("- press 2 if you want to find information of a city");
    println!("Please enter a number: ");
}

/* 
fn main(){
    let mut weather_station= WeatherStation::new();
    while true {
        let mut press_key = String::new();
        display();
        io::stdin().read_line(&mut press_key).expect("Failed to read line");
        let press_key = press_key.trim();
        if press_key == "1" {
            let mut city = String::new();
            let mut temp = String::new();
            let mut humi = String::new(); 
            let mut weather_condition = String::new();
            io::stdin().read_line(&mut city).expect("Failed to read line");
            io::stdin().read_line(&mut temp).expect("Failed to read line");
            io::stdin().read_line(&mut humi).expect("Failed to read line");
            io::stdin().read_line(&mut weather_condition).expect("Failed to read line");
            let mut temperature: f64 = temp.trim().parse().expect("fail");
            let mut humidity : u32 = humi.trim().parse().expect("fail");
            weather_station.add_city(city, temperature, humidity, weather_condition);
        } else {
            let mut city = String::new();
            io::stdin().read_line(&mut city).expect("Fail to read line");
            weather_station.find_city(city);
        }
    }
}

*/
