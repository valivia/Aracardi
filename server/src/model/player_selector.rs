#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerSelector {
    Host,
    Offset(u32),
    Fill(RangeSelector),
    Random(RangeSelector),
}

impl TryFrom<&str> for PlayerSelector {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, ()> {
        let target = input.split_once('.');
        match (input, target) {
            ("host", _) => Ok(Self::Host),
            (_, Some(("offset", target))) => Ok(Self::Offset(target.parse().map_err(|_| ())?)),
            (_, Some(("fill", target))) => Ok(Self::Fill(RangeSelector::try_from(target)?)),
            (_, Some(("random", target))) => Ok(Self::Random(RangeSelector::try_from(target)?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeSelector {
    pub min: Option<usize>,
    /// _inclusive_ max
    pub max: Option<usize>,
}

impl RangeSelector {
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }

    /// adds the minima and maxima, assuming max of None is infinity
    pub fn combine_substages(&self, input: &RangeSelector) -> Self {
        let min = match (self.min, input.min) {
            (Some(one), Some(two)) => Some(one + two),
            (Some(one), None) => Some(one),
            (None, Some(two)) => Some(two),
            (None, None) => None,
        };

        let max = match (self.max, input.max) {
            (Some(one), Some(two)) => Some(one + two),
            _ => None,
        };

        Self { min, max }
    }

    /// returns the intersection of the two ranges
    pub fn intersect(&self, input: &RangeSelector) -> Self {
        let min = match (self.min, input.min) {
            (Some(one), Some(two)) => Some(one.max(two)),
            (Some(one), None) => Some(one),
            (None, Some(two)) => Some(two),
            (None, None) => None,
        };

        let max = match (self.max, input.max) {
            (Some(one), Some(two)) => Some(one.min(two)),
            (Some(one), None) => Some(one),
            (None, Some(two)) => Some(two),
            (None, None) => None,
        };

        Self { min, max }
    }
}

impl TryFrom<&str> for RangeSelector {
    type Error = ();

    /// ```
    /// use server::model::player::FillSelector;
    /// assert_eq!(FillSelector::try_from_str("0..5").unwrap(), FillSelector {start: Some(0), end: Some(5)});
    /// ```
    fn try_from(input: &str) -> Result<Self, ()> {
        let range = input.split_once("..");
        Ok(RangeSelector {
            min: match range {
                Some((start, _)) => Some(start.parse().map_err(|_| ())?),
                None => None,
            },
            max: match range {
                Some((_, end)) => Some(end.parse::<usize>().map_err(|_| ())?),
                None => None,
            },
        })
    }
}
