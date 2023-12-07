#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]

use tap::Pipe;

#[derive(Debug)]
pub struct Garden {
    seeds: Vec<Range>,
    seed_to_soil: RangeMaps,
    soil_to_fertilizer: RangeMaps,
    fertilizer_to_water: RangeMaps,
    water_to_light: RangeMaps,
    light_to_temperature: RangeMaps,
    temperature_to_humidity: RangeMaps,
    humidity_to_location: RangeMaps,
}

#[derive(Debug)]
pub struct Range {
    start: u64,
    length: u64,
}

#[derive(Debug)]
pub struct RangeIter {
    current: u64,
    end: u64,
}

impl Iterator for RangeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let result = Some(self.current);
            self.current += 1;
            result
        }
    }
}

impl Range {
    pub fn singular(val: u64) -> Self {
        Self {
            start: val,
            length: 1,
        }
    }

    pub fn range(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    pub fn iter(&self) -> RangeIter {
        RangeIter {
            current: self.start,
            end: self.start + self.length,
        }
    }
}

#[derive(Debug)]
pub struct RangeMaps {
    maps: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct RangeMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl RangeMaps {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Self {
        let mut ranges = Vec::new();
        while let Some(line) = lines.next() && !line.is_empty() {
            ranges.push(RangeMap::parse(line))
        }
        Self { maps: ranges }
    }
}

impl RangeMap {
    pub fn parse(s: &str) -> Self {
        let mut values = s.split_whitespace();
        let destination_start = values.next().unwrap().parse().unwrap();
        let source_start = values.next().unwrap().parse().unwrap();
        let length = values.next().unwrap().parse().unwrap();
        Self {
            source_start,
            destination_start,
            length,
        }
    }
}

pub enum Part {
    One,
    Two,
}

impl Garden {
    pub fn parse(s: &str, part: Part) -> Self {
        let mut lines = s.lines();
        let seeds = {
            let line = lines.next().unwrap();
            let mut parts = line.split(':');
            assert_eq!(parts.next(), Some("seeds"));
            match part {
                Part::One => parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| Range::singular(s.parse().unwrap()))
                    .collect(),
                Part::Two => parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .array_chunks()
                    .map(|[start, length]| {
                        Range::range(start.parse().unwrap(), length.parse().unwrap())
                    })
                    .collect(),
            }
        };
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("seed-to-soil map:"));
        let seed_to_soil = RangeMaps::parse(&mut lines);
        assert_eq!(lines.next(), Some("soil-to-fertilizer map:"));
        let soil_to_fertilizer = RangeMaps::parse(&mut lines);
        assert_eq!(lines.next(), Some("fertilizer-to-water map:"));
        let fertilizer_to_water = RangeMaps::parse(&mut lines);
        assert_eq!(lines.next(), Some("water-to-light map:"));
        let water_to_light = RangeMaps::parse(&mut lines);
        assert_eq!(lines.next(), Some("light-to-temperature map:"));
        let light_to_temperature = RangeMaps::parse(&mut lines);
        assert_eq!(lines.next(), Some("temperature-to-humidity map:"));
        let temperature_to_humidity = RangeMaps::parse(&mut lines);
        assert_eq!(lines.next(), Some("humidity-to-location map:"));
        let humidity_to_location = RangeMaps::parse(&mut lines);

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    pub fn seeds(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds.iter().map(|range| range.iter()).flatten()
    }

    pub fn location_for(&self, seed: u64) -> u64 {
        seed.pipe(&self.seed_to_soil)
            .pipe(&self.soil_to_fertilizer)
            .pipe(&self.fertilizer_to_water)
            .pipe(&self.water_to_light)
            .pipe(&self.light_to_temperature)
            .pipe(&self.temperature_to_humidity)
            .pipe(&self.humidity_to_location)
    }
}

impl FnOnce<(u64,)> for RangeMap {
    type Output = Option<u64>;

    extern "rust-call" fn call_once(self, args: (u64,)) -> Self::Output {
        Fn::call(&self, args)
    }
}

impl FnMut<(u64,)> for RangeMap {
    extern "rust-call" fn call_mut(&mut self, args: (u64,)) -> Self::Output {
        Fn::call(&*self, args)
    }
}

impl Fn<(u64,)> for RangeMap {
    extern "rust-call" fn call(&self, (arg,): (u64,)) -> Self::Output {
        match arg {
            val if val >= self.source_start && val < self.source_start + self.length => {
                Some(self.destination_start + arg - self.source_start)
            }
            _ => None,
        }
    }
}

impl FnOnce<(u64,)> for RangeMaps {
    type Output = u64;

    extern "rust-call" fn call_once(self, args: (u64,)) -> Self::Output {
        Fn::call(&self, args)
    }
}

impl FnMut<(u64,)> for RangeMaps {
    extern "rust-call" fn call_mut(&mut self, args: (u64,)) -> Self::Output {
        Fn::call(&*self, args)
    }
}

impl Fn<(u64,)> for RangeMaps {
    extern "rust-call" fn call(&self, args: (u64,)) -> Self::Output {
        for map in &self.maps {
            if let Some(value) = map(args.0) {
                return value;
            }
        }
        return args.0;
    }
}
