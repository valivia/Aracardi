use chrono::Duration;

pub trait HumanReadable {
    fn human_readable(&self) -> String;
}

impl HumanReadable for Duration {
    fn human_readable(&self) -> String {
        let total_ms = self.num_milliseconds();
        let days = total_ms / 86_400_000;
        let hours = (total_ms % 86_400_000) / 3_600_000;
        let minutes = (total_ms % 3_600_000) / 60_000;
        let seconds = (total_ms % 60_000) as f64 / 1_000.0;
        let millis = total_ms % 1_000;

        if days > 0 {
            let plural = if days != 1 { "s" } else { "" };
            if hours > 0 {
                return format!(
                    "{} day{}, {} hour{}",
                    days,
                    plural,
                    hours,
                    if hours != 1 { "s" } else { "" }
                );
            }
            return format!("{} day{}", days, plural);
        }

        if hours > 0 {
            let plural = if hours != 1 { "s" } else { "" };
            if minutes > 0 {
                return format!(
                    "{} hour{}, {} minute{}",
                    hours,
                    plural,
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                );
            }
            return format!("{} hour{}", hours, plural);
        }

        if minutes > 0 {
            return format!("{} minute{}", minutes, if minutes != 1 { "s" } else { "" });
        }

        if total_ms >= 1_000 {
            return format!("{:.1} seconds", seconds);
        }

        format!(
            "{} millisecond{}",
            millis,
            if millis != 1 { "s" } else { "" }
        )
    }
}
