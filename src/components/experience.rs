use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Experience {
    pub experience: i64,
}

impl Experience {
	#[allow(dead_code)]
    pub fn calculate_level(&self) -> u8 {
        for (level, required_experience) in EXPERIENCE_TABLE.iter().enumerate().take(254) {
            if self.experience < *required_experience {
               return level as u8; 
            }
        }
        1
    }  

	pub fn should_level_up(&self, current_level: u8) -> bool {
		EXPERIENCE_TABLE[current_level as usize] <= self.experience
	}  
}

impl From<&PlayerRow> for Experience {
    fn from(player_row: &PlayerRow) -> Self {
        Experience { 
            experience: player_row.experience
        }
    }
}

const EXPERIENCE_TABLE: [i64; 255] = [
	0, 5, 24, 60, 80, 164, 271, 407, 579, 794, 1125, 1543, 2068, 2722,
	3534, 4563, 5830, 7385, 9286, 11607, 17493, 21845, 27147, 33593, 41416,
	51341, 63394, 78005, 95696, 117090, 166758, 203151, 247043, 299942, 363659,
	440404, 532757, 643849, 777437, 938032, 1211834, 1460324, 1758856, 2117449, 2548126,
	3065315, 3686331, 4431950, 5327106, 6401717, 10255633, 12320243, 14798389, 17772795,
	21342730, 25627317, 30769501, 36940820, 44347118, 53235407, 63902104, 76702906, 92064653,
	110499551, 132622249, 159170373, 191028978, 229260177, 275138508, 330193415, 396260232, 1188853398,
	1426699190, 1712116603, 2054620005, 2465626643, 2958837208, 3550692535, 4260921625, 5113199283, 12271870776,
	14726443120, 17671935710, 21206532706, 31810068850, 38172360013, 45807117133, 54968833525, 65962901170, 94986948540,
	113984718984, 136782053544, 164138865195, 196967049519, 315147841592, 378177986472, 453814174732, 544577615260, 653493758732,
	980241432400, 1029253504020, 1080716179221, 1134751988182, 1191489587591, 1251064066970, 1313617270318, 1379298133833, 1448263040524, 1448263040524,
	1520676192550, 1596710002177, 1676545502285, 1760372777399, 1848391416268, 1940810987081, 2037851536435, 2139744113256, 2246731318918, 2359067884863, 2477021279106,
	2600872343061, 2730915960214, 2867461758224, 3010834846135, 3161376588441, 3319445417863, 3485417688756, 3659688573193, 3842673001852, 4034806651944, 4236546984541,
	4448374333768, 4670793050456, 4904332702978, 5149549338126, 5407026805032, 5677378145283, 5961247052547, 6259309405174, 6572274875432, 6900888619203, 7245933050163,
	7608229702671, 7988641187804, 8388073247194, 8807476909553, 9247850755030, 9710243292781, 10195755457420, 10705543230291, 11240820391805, 11802861411395, 12393004481964,
	13012654706062, 13663287441365, 13936553190192, 14215284253995, 14499589939074, 14789581737855, 15085373372612, 15387080840064, 15694822456865, 16008718906002, 16328893284122,
	16655471149804, 16988580572800, 17328352184256, 17674919227941, 18028417612499, 18388985964748, 18756765684042, 19131900997722, 19514539017676, 19904829798029, 20302926393989,
	20708984921868, 21123164620305, 21545627912711, 21976540470965, 22416071280384, 22864392705991, 23321680560110, 23788114171312, 24263876454738, 24749153983832, 25244137063508,
	25749019804778, 26264000200873, 26789280204890, 27325065808987, 27871567125166, 28428998467669, 28997578437022, 29577530005762, 30169080605877, 30772462217994, 31387911462353,
	32015669691600, 32655983085432, 33309102747140, 33975284802082, 34654790498123, 35347886308085, 36054844034246, 36775940914930, 37511459733228, 38261688927892, 39026922706449,
	39807461160577, 40603610383788, 41415682591463, 42243996243292, 43088876168157, 43950653691520, 44829666765350, 45726260100657, 46640785302670, 47573601008723, 48525073028897,
	49495574489474, 50485485979263, 51495195698848, 52525099612824, 53575601605080, 54647113637181, 55740055909924, 56854857028122, 57991954168684, 59151793252057, 60334829117098,
	61541525699439, 62772356213427, 64027803337695, 65308359404448, 66614526592536, 67946817124386, 69305753466873, 70691868536210, 72105705906934, 73547820025072, 75018776425573,
	76519151954084, 78049534993165, 79610525693028, 81202736206888, 82826790931025, 84483326749645, 86172993284637, 87896453150329, 89654382213335, 91447469857601, 93276419254753,
	95141947639848, 97044786592644, 106749265251908,
];