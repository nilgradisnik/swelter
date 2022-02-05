use gtk::glib::Sender;

use libmedium::{
    parse_hwmons,
    hwmon::ParsingError,
    sensors::{Input, Sensor as HWSensor},
};

const INTERVAL_SECONDS: u64 = 1;

#[derive(Debug)]
pub struct Sensor {
    pub name: String,
    pub temperature: f64
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub sensors: Vec<Sensor>
}

async fn get_monitors() -> Result<Vec<Monitor>, ParsingError> {
    let hwmons = parse_hwmons()?;

    let mut monitors: Vec<Monitor> = Vec::new();

    for (_index, name, hwmon) in &hwmons {
        let mut monitor = Monitor {
            name: name.to_string(),
            sensors: Vec::new(),

        };

        for (_, sensor) in hwmon.temps() {
            if let Ok(temperature) = sensor.read_input() {
                monitor.sensors.push(Sensor {
                    name: sensor.name(),
                    temperature: temperature.as_degrees_celsius()
                })
            }
        }

        monitors.push(monitor);
    }

    Ok(monitors)
}

pub async fn poll(sender: Sender<Vec<Monitor>>) {
    let mut interval = async_timer::Interval::platform_new(core::time::Duration::from_secs(INTERVAL_SECONDS));

    loop {
        match get_monitors().await {
            Ok(monitors) => {
                sender.send(monitors).expect("Could not send monitors through channel");
            },
            Err(error) => println!("Error getting monitors: {:?}", error)
        }

        interval.as_mut().await;
    }
}
