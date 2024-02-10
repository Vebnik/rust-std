pub enum TempUnit {
	Celsius = 0,
	Fahrenheit = 1,
}

fn celsius_flow(temp: f32, target: TempUnit) -> f32 {
	match target {
	    TempUnit::Celsius => return temp,
	    TempUnit::Fahrenheit => return temp * (9.0/5.0) + 32.0,
	}
}

fn fahrenheit_flow(temp: f32, target: TempUnit) -> f32 {
	match target {
	    TempUnit::Celsius => return (temp - 32.0) * (5.0/9.0),
	    TempUnit::Fahrenheit => return temp,
	}
}

pub fn temp_convert(temp: f32, from: TempUnit, target: TempUnit) -> f32{
	match from {
		TempUnit::Celsius => celsius_flow(temp, target),
		TempUnit::Fahrenheit => fahrenheit_flow(temp, target),
	}
}