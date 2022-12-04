fn main() -> color_eyre::Result<()> {
    let file = include_str!("input");

    let pairs = file
        .lines()
        .map(|line| line.try_into().unwrap())
        .filter(|pair: &Pair| pair.contains());

    println!("there are {} containing pairs", pairs.count());

    let pairs = file
        .lines()
        .map(|line| line.try_into().unwrap())
        .filter(|pair: &Pair| pair.overlaps());

    println!("there are {} overlapping pairs", pairs.count());
    Ok(())
}

#[derive(Debug)]
struct Pair(Assignment, Assignment);

impl Pair {
    fn contains(&self) -> bool {
        self.0.section_start >= self.1.section_start && self.0.section_end <= self.1.section_end
            || self.1.section_start >= self.0.section_start
                && self.1.section_end <= self.0.section_end
    }

    fn overlaps(&self) -> bool {
        self.0.section_end >= self.1.section_start && self.0.section_start <= self.1.section_start
            || self.1.section_end >= self.0.section_start
                && self.1.section_start <= self.0.section_start
    }
}

#[cfg(test)]
mod test_pair {
    use super::*;

    #[test]
    fn test_contains() {
        // a pair contains itself
        let pair: Pair = "5-5,5-5".try_into().unwrap();
        assert!(pair.contains());

        // contains returns false when they dont overlap
        let pair: Pair = "5-5,4-4".try_into().unwrap();
        assert!(!pair.contains());

        // contains returns true when a inside b
        let pair: Pair = "5-5,4-6".try_into().unwrap();
        assert!(pair.contains());

        // contains returns true when b inside a
        let pair: Pair = "1-10,4-6".try_into().unwrap();
        assert!(pair.contains());
    }

    #[test]
    fn test_overlaps() {
        // a pair overlaps itself
        let pair: Pair = "5-5,5-5".try_into().unwrap();
        assert!(pair.overlaps());

        // a overlaps b
        let pair: Pair = "5-6,6-10".try_into().unwrap();
        assert!(pair.overlaps());

        // a doesn't overlap b
        let pair: Pair = "5-5,6-10".try_into().unwrap();
        assert!(!pair.overlaps());
    }
}

impl TryFrom<&str> for Pair {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (pair_1, pair_2) = value
            .split_once(',')
            .ok_or_else(|| color_eyre::eyre::eyre!("could not split pair with ',', {:?}", value))?;
        Ok(Self(pair_1.try_into()?, pair_2.try_into()?))
    }
}

#[derive(Debug)]
struct Assignment {
    section_start: u32,
    section_end: u32,
}

impl TryFrom<&str> for Assignment {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .split_once('-')
            .map(
                |sections| match (sections.0.parse::<u32>(), sections.1.parse::<u32>()) {
                    (Ok(section_start), Ok(section_end)) => Ok(Assignment {
                        section_start,
                        section_end,
                    }),
                    _ => Err(color_eyre::eyre::eyre!(
                        "could not parse assignment {:?}",
                        value
                    )),
                },
            )
            .ok_or_else(|| {
                color_eyre::eyre::eyre!("could not split assignment at '-' {:?}", value)
            })?
    }
}
