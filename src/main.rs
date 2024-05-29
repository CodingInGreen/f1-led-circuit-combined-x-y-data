use std::error::Error;
use std::fs::File;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Timestamp {
    x: f64,
    y: f64,
    z: f64,  // Added z column
    date: String,
    designator: String,
}

#[derive(Debug, Deserialize)]
struct LedCoord {
    designator: String,
    x_led: f64,
    y_led: f64,
}

#[derive(Debug, Serialize)]
struct Combined {
    x: f64,
    y: f64,
    z: f64,  // Added z column to Combined struct
    date: String,
    designator: String,
    x_led: f64,
    y_led: f64,
}

fn read_csv<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }
    Ok(records)
}

fn write_csv<T: Serialize>(file_path: &str, records: &[T]) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = csv::Writer::from_writer(file);
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let timestamps: Vec<Timestamp> = read_csv("led_designator_labeled_zandvoort_data_verstappen.csv")?;
    let led_coords: Vec<LedCoord> = read_csv("led_coords.csv")?;
    
    let mut led_map = HashMap::new();
    for coord in led_coords {
        led_map.insert(coord.designator.clone(), (coord.x_led, coord.y_led));
    }

    let mut combined_data = Vec::new();
    for record in timestamps {
        if let Some(&(x_led, y_led)) = led_map.get(&record.designator) {
            combined_data.push(Combined {
                x: record.x,
                y: record.y,
                z: record.z,  // Including z column in the Combined struct
                date: record.date.clone(),
                designator: record.designator.clone(),
                x_led,
                y_led,
            });
        }
    }

    write_csv("combined-x-y-verstappen.csv", &combined_data)?;

    Ok(())
}
