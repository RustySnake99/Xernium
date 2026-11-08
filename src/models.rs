use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Element {
    pub atomic_number: u8,
    pub symbol: String,
    pub name: String,
    pub atomic_mass: Option<String>,
    #[serde(rename = "CPKHexColor")]
    pub cpk_hex_color: Option<String>,
    pub electron_configuration: Option<String>,
    pub electronegativity: Option<String>,
    pub atomic_radius: Option<String>,
    pub ionization_energy: Option<String>,
    pub electron_affinity: Option<String>,
    pub oxidation_states: Option<String>,
    pub standard_state: Option<String>,
    pub melting_point: Option<String>,
    pub boiling_point: Option<String>,
    pub density: Option<String>,
    pub group_block: Option<String>,
    pub year_discovered: Option<String>,
}

pub fn load_dataset() -> Vec<Element> {
    let raw = include_str!("../data/elements.csv");
    csv::Reader::from_reader(raw.as_bytes())
        .deserialize()
        .filter_map(Result::ok)
        .collect()
}

pub fn get_grid_position(z: u8) -> (usize, usize) {
    let z = z as usize;
    match z {
        1 => (0, 0),
        2 => (0, 17),
        3..=4 => (1, z - 3),
        5..=10 => (1, z + 7),
        11..=12 => (2, z - 11),
        13..=18 => (2, z - 1),
        19..=36 => (3, z - 19),
        37..=54 => (4, z - 37),
        55..=56 => (5, z - 55),
        57..=71 => (8, z - 55),
        72..=86 => (5, z - 69),
        87..=88 => (6, z - 87),
        89..=103 => (9, z - 87),
        104..=118 => (6, z - 101),
        _ => (0, 0),
    }
}
