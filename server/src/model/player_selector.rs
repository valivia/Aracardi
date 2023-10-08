#[derive(Debug, Clone)]
pub enum PlayerSelector {
    Host,
    Current,
    Offset(u32),
    Fill(FillSelector),
    Random(FillSelector),
}

impl TryFrom<&str> for PlayerSelector {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, ()> {
        let target = input.split_once('.');
        match (input, target) {
            ("host", _) => Ok(Self::Host),
            ("current", _) => Ok(Self::Current),
            (_, Some(("offset", target))) => Ok(Self::Offset(target.parse().map_err(|_| ())?)),
            (_, Some(("fill", target))) => Ok(Self::Fill(FillSelector::try_from(target)?)),
            (_, Some(("random", target))) => Ok(Self::Random(FillSelector::try_from(target)?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FillSelector {
    pub min: Option<usize>,
    /// _inclusive_ max
    pub max: Option<usize>,
}

impl TryFrom<&str> for FillSelector {
    type Error = ();

    /// ```
    /// use server::model::player::FillSelector;
    /// assert_eq!(FillSelector::try_from_str("0..5").unwrap(), FillSelector {start: Some(0), end: Some(5)});
    /// ```
    fn try_from(input: &str) -> Result<Self, ()> {
        let range = input.split_once("..");
        Ok(FillSelector {
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
