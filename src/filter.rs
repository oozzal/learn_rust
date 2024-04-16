use serde_json::Value;

fn vec_filter<'a>(iterable: &Vec<&'a Value>, key: &str, value: &str) -> Vec<&'a Value> {
    iterable
        .iter()
        .filter(|i| i[key].is_null() || i[key].as_array().unwrap().contains(&Value::from(value)))
        .map(|i| *i)
        .collect()
}

fn hour_filter<'a>(iterable: &Vec<&'a Value>, hour: u64) -> Vec<&'a Value> {
    iterable
        .iter()
        .filter(|i| {
            if i["hour"].is_null() {
                return true;
            }
            let range = &i["hour"];
            let from = range[0].as_u64().unwrap();
            let to = range[1].as_u64().unwrap();
            hour >= from && hour <= to
        })
        .map(|i| *i)
        .collect()
}

fn filter<'a>(setting: &'a Value, filters: &[(&str, &str)]) -> &'a Value {
    let mut setting: Vec<&Value> = setting.as_array().unwrap().iter().collect();
    for (key, value) in filters {
        if *key == "hour" {
            setting = hour_filter(&setting, value.parse::<u64>().unwrap());
        } else {
            setting = vec_filter(&setting, key, value);
        };
    }
    &setting[0]
}

// v1: device, hour of day and carousel
pub fn main() {
    let json = r#"[
      {"device": ["desktop"], "hour": [5,8], "ids": [1, 2, 3], "duration": 3},
      {"device": ["desktop"], "ids": [7, 8, 9], "duration": 3},
      {"device": ["mobile", "tablet"], "ids": [4, 5, 6], "duration": 3}
    ]"#;
    let v: Value = serde_json::from_str(json).unwrap();
    let filters = [("device", "mobile")];
    let result = filter(&v, &filters);
    println!("{:?}", result);
}
