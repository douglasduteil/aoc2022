//

use std::{fmt::Display, ops::RangeInclusive};

#[derive(Debug)]
pub struct Crt {
    cycle: usize,
    screen: [bool; Self::WIDE * Self::HIGH],
    x: isize,
}

impl Crt {
    const WIDE: usize = 40;
    const HIGH: usize = 6;

    fn sprite_range(&self) -> RangeInclusive<isize> {
        let index: isize = self.cycle.try_into().unwrap();
        let index = index % Crt::WIDE as isize;
        let left = index - 1;
        let right = index + 1;
        left..=right
    }

    pub fn render(&mut self, delta_x: Vec<isize>) {
        for delta in delta_x {
            self.screen[self.cycle] = self.sprite_range().contains(&self.x);
            self.cycle += 1;
            self.x += delta;
        }
    }
}

#[cfg(test)]
mod test_crt_render {
    use super::*;

    #[test]
    fn test_render_first_line_first_pixel() {
        let mut crt = Crt::default();
        crt.render(vec![0]);
        assert_eq!(
            crt.to_string().lines().take(1).collect::<Vec<_>>().join(""),
            "#......................................."
        );
    }

    #[test]
    fn test_render_second_line_first_pixel() {
        let mut crt = Crt::default();
        crt.render([&[0; Crt::WIDE], [1].as_slice()].concat().to_vec());
        assert_eq!(
            crt.to_string()
                .lines()
                .skip(1)
                .take(1)
                .collect::<Vec<_>>()
                .join(""),
            "#......................................."
        );
    }
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            cycle: 0,
            screen: [false; Self::WIDE * Self::HIGH],
            x: 1,
        }
    }
}
impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let screen = self
            .screen
            .chunks(Self::WIDE)
            .map(|row| {
                row.iter()
                    .map(|pixel| match pixel {
                        true => "#",
                        _ => ".",
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&screen)
    }
}
