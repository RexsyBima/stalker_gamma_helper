use std::error::Error;
use std::io;

use serde::Deserialize;

use encoding_rs::WINDOWS_1252;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;

fn read_kv_csv(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;

    let mut map = HashMap::new();

    for result in rdr.byte_records() {
        let record = result?;

        let key = record
            .get(0)
            .map(|b| WINDOWS_1252.decode(b).0.into_owned())
            .unwrap_or_default();

        let value = record
            .get(1)
            .map(|b| WINDOWS_1252.decode(b).0.into_owned())
            .unwrap_or_default();

        if !key.is_empty() {
            map.insert(key, value);
        }
    }

    Ok(map)
}

#[derive(Deserialize, Debug)]
struct Ammo {
    #[serde(rename = "~")]
    id: String,

    #[serde(rename = "pda_encyclopedia_name")]
    name: String,

    #[serde(rename = "st_data_export_description")]
    description: String,

    #[serde(rename = "st_prop_weight")]
    weight: f32,

    #[serde(rename = "st_data_export_ammo_box_size")]
    box_size: u32,

    #[serde(rename = "st_upgr_cost")]
    cost: u32,

    #[serde(rename = "BR1")]
    br1: Option<String>,

    #[serde(rename = "BR2")]
    br2: Option<String>,

    #[serde(rename = "BR3")]
    br3: Option<String>,

    #[serde(rename = "BR4")]
    br4: Option<String>,

    #[serde(rename = "BR5")]
    br5: Option<String>,

    #[serde(rename = "BR6")]
    br6: Option<String>,

    #[serde(rename = "BR7")]
    br7: Option<String>,

    #[serde(rename = "st_data_export_projectiles")]
    projectiles: u32,

    #[serde(rename = "ui_inv_damage")]
    damage: f32,

    #[serde(rename = "ui_inv_accuracy")]
    accuracy: String, // e.g. "66%"

    #[serde(rename = "ui_inv_wrange")]
    range: String, // e.g. "100%"

    #[serde(rename = "st_data_export_falloff")]
    falloff: String, // e.g. "82%"

    #[serde(rename = "ui_inv_bspeed")]
    bullet_speed: String, // e.g. "187%"

    #[serde(rename = "st_data_export_impulse")]
    impulse: String, // e.g. "55%"

    #[serde(rename = "st_data_export_weapon_degradation")]
    weapon_degradation: String, // e.g. "40%"

    #[serde(rename = "st_data_export_k_hit")]
    k_hit: f32,

    #[serde(rename = "st_data_export_k_ap")]
    k_ap: f32,

    #[serde(rename = "st_data_export_k_air_resistance")]
    k_air_resistance: f32,
}

#[derive(Deserialize, Debug)]
struct Weapon {
    #[serde(rename = "placeholder")] // CSV column name
    name: String, // your struct field name

    #[serde(rename = "placeholder")] // CSV column name
    description: String,
}

fn read_csv<T: DeserializeOwned>(path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let items: Result<Vec<T>, _> = rdr.deserialize().collect();
    let items = items?;
    Ok(items)
}

fn main() {
    // todo!("get the field header of the csv file");
    let csv_file_paths = [
        "data/gamma-0.9.5/export_ammo.csv",
        "data/gamma-0.9.5/en_us.csv",
    ];
    let app_version: u8 = 1;
    let gamma_tip = "Use bandage to stop the bleeding";
    let gamma_tips = vec!["tip 1", "tip 2", "tip 3"];
    let random_number = rand::random_range(0..gamma_tips.len());
    let mut user_input = String::new();
    let ammos: Vec<Ammo> = read_csv(csv_file_paths[0]).expect("failed to read csv file");
    let db = read_kv_csv(csv_file_paths[1]);
    dbg!(ammos);
    dbg!(&db);

    dbg!(db.expect("").get("st_up_zatp_c3_descr"));
    println!("current app version is {}!", app_version);
    println!("you want to get current gamma tip?");

    io::stdin()
        .read_line(&mut user_input)
        .expect("please input your value");

    println!("input from 1 to {} to get random tip", gamma_tips.len());

    match user_input.trim() {
        "yes" | "y" => println!("Gamma tip: {}", gamma_tip),
        "no" | "n" => println!("Okay, maybe next time!"),
        _ => println!("Invalid input. Please enter 'yes' or 'no'."),
    }

    println!("{}", gamma_tips[random_number])
}
