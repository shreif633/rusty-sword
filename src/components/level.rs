use bevy::prelude::*;
use crate::enums::level_color::LevelColor;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Level {
    pub level: u8,
}

impl Level {
    pub fn get_target_color(&self, target_level: u8) -> LevelColor {
        let n_level_form = [ [1, 10], [11, 20], [21, 30], [31, 40], [41, 50], [51, 60], [61, 70], [71, 255] ];
        let n_vs_form: [[[i16; 2]; 7]; 8] = [ 
            [ [-255, -7], [-6, -4], [-3, -1], [0, 2], [3, 6], [7, 11], [12, 255] ],
            [ [-255, -8], [-7, -5], [-4, -2], [-1, 2], [3, 6], [7, 11], [12, 255] ],
            [ [-255, -9], [-8, -6], [-5, -3], [-2, 1], [2, 5], [6, 10], [11, 255] ],
            [ [-255, -10], [-9, -7], [-6, -4], [-3, 1], [2, 5], [6, 10], [11, 255] ],
            [ [-255, -11], [-10, -8], [-7, -5], [-4, 0], [1, 4], [5, 9], [10, 255] ],
            [ [-255, -12], [-11, -9], [-8, -6], [-5, 0], [1, 4], [5, 9], [10, 255] ],
            [ [-255, -13], [-12, -10], [-9, -7], [-6, 0], [1, 3], [4, 8], [9, 255] ],
            [ [-255, -14], [-13, -11], [-10, -8], [-7, 0], [1, 3], [4, 8], [9, 255] ],
        ];
        for i in 0..8 {
            if n_level_form[i][0] <= self.level && n_level_form[i][1] >= self.level {
                for k in 0..7 {
                    let n_account_level = target_level as i16 - self.level as i16;
                    if n_vs_form[i][k][0] <= n_account_level && n_vs_form[i][k][1] >= n_account_level {
                        return LevelColor::from(k as u8);
                    }
                }
            }
        }
        LevelColor::Gray
    }
}

impl From<&PlayerRow> for Level {
    fn from(player_row: &PlayerRow) -> Self {
        Level {
            level: player_row.level
        }
    }
}
