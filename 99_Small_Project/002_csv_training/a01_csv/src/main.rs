use std::error::Error;

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Row {
    city: String,
    country: String,
    #[serde(rename = "popcount")]
    population: u64,
}

fn example() -> Result<(), Box<dyn Error>> {
    let data = "\
     city,country,popcount
Boston,United States,4628910
     ";
    let rdr = csv::Reader::from_reader(data.as_bytes());
    let mut iter = rdr.into_deserialize();

    if let Some(result) = iter.next() {
        let record: Row = result?;
        assert_eq!(
            record,
            Row {
                city: "Boston".to_string(),
                country: "United States".to_string(),
                population: 4628910,
            }
        );
        Ok(())
    } else {
        Err(From::from("expected at least one record but got none"))
    }
}

fn main() {
    example().unwrap();
}
