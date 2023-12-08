#[derive(Debug)]
pub struct Race {
    pub duration: u64,
    pub record: u64,
}

#[derive(Debug)]
pub struct Races {
    pub races: Vec<Race>,
}

impl Races {
    pub fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let mut times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|s| !s.is_empty());
        assert_eq!(times.next(), Some("Time:"));

        let mut records = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|s| !s.is_empty());
        assert_eq!(records.next(), Some("Distance:"));

        let mut races = Vec::new();
        for (time, distance) in times.zip(records) {
            races.push(Race {
                duration: time.parse().unwrap(),
                record: distance.parse().unwrap(),
            })
        }
        Self { races }
    }
}

impl Race {
    pub fn ways_to_win(&self) -> u64 {
        // record = (duration - x) * x
        // -x^2 + x * duration = record
        // x^2 -x*duration + record = 0
        // x = (duration +- sqrt(duration^2 - 4*record)) / 2

        let duration = self.duration as f64;
        let record = self.record as f64;

        let x1 = ((duration - (duration.powi(2) - 4.0 * record).sqrt()) / 2.0).ceil() as u64;
        let x2 = ((duration + (duration.powi(2) - 4.0 * record).sqrt()) / 2.0).floor() as u64;
        x2 - x1 + 1
    }

    pub fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let mut time = lines.next().unwrap().split(':').filter(|s| !s.is_empty());
        assert_eq!(time.next(), Some("Time"));
        let time = time
            .next()
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse()
            .unwrap();

        let mut distance = lines.next().unwrap().split(':').filter(|s| !s.is_empty());
        assert_eq!(distance.next(), Some("Distance"));
        let distance = distance
            .next()
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse()
            .unwrap();

        Self {
            duration: time,
            record: distance,
        }
    }

    pub fn simulate(&self, time_pressed: u64) -> u64 {
        time_pressed * (self.duration - time_pressed)
    }
}
