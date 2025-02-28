use std::collections::{HashMap, HashSet};
use std::fs;

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
pub fn count_provinces() -> String {
    let json_str = fs::read_to_string("./district.json").expect("Failed to read file");
    let data: HashMap<String, Districts> =
        serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut count = vec![];

    let mut keys = data.keys().collect::<Vec<_>>();
    keys.sort();

    for key in keys {
        let districts = data.get(key).unwrap();

        let mut city_sets: Vec<HashSet<String>> = Vec::new();
        districts.0.iter().for_each(|(city, related)| {
            let mut city_set = HashSet::new();
            city_set.insert(city.clone());
            city_set.extend(related.clone());

            let mut is_intersect = false;
            for s in &mut city_sets {
                let intersection = s
                    .intersection(&city_set)
                    .map(|c| c.clone())
                    .collect::<HashSet<_>>();
                if intersection.len() > 0 {
                    s.extend(city_set.clone());
                    is_intersect = true;
                    break;
                }
            }

            if !is_intersect {
                city_sets.push(city_set);
            }
        });

        let mut c = city_sets.len();
        city_sets.iter().enumerate().for_each(|(i, s)| {
            for j in i + 1..city_sets.len() {
                let intersection = s.intersection(&city_sets[j]);
                if intersection.count() > 0 {
                    c -= 1;
                }
            }
        });

        count.push(c);
    }

    count
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Clone, Debug)]
struct Districts(HashMap<String, HashSet<String>>);

impl<'de> Deserialize<'de> for Districts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'d> Visitor<'d> for MyVisitor {
            type Value = Districts;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("a map of abilities")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'d>,
            {
                let mut district = Districts(HashMap::new());
                while let Some((key, value)) = access.next_entry::<String, HashSet<String>>()? {
                    district
                        .0
                        .entry(key)
                        .and_modify(|s| s.extend(value.clone()))
                        .or_insert(HashSet::from_iter(value.into_iter()));
                }

                Ok(district)
            }
        }
        Ok(deserializer.deserialize_map(MyVisitor)?)
    }
}
