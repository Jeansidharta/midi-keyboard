use crate::constants;

#[derive(Clone)]
pub enum NoteName {
    C(u8),
    CS(u8),
    D(u8),
    DS(u8),
    E(u8),
    F(u8),
    FS(u8),
    G(u8),
    GS(u8),
    A(u8),
    AS(u8),
    B(u8),
}

impl NoteName {
    pub fn parse(num: u8) -> NoteName {
        let octave = num / 12;
        match num % 12 {
            0 => NoteName::C(octave),
            1 => NoteName::CS(octave),
            2 => NoteName::D(octave),
            3 => NoteName::DS(octave),
            4 => NoteName::E(octave),
            5 => NoteName::F(octave),
            6 => NoteName::FS(octave),
            7 => NoteName::G(octave),
            8 => NoteName::GS(octave),
            9 => NoteName::A(octave),
            10 => NoteName::AS(octave),
            11 => NoteName::B(octave),
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for NoteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteName::C(o) => write!(f, "C{}", o),
            NoteName::CS(o) => write!(f, "C#{}", o),
            NoteName::D(o) => write!(f, "D{}", o),
            NoteName::DS(o) => write!(f, "D#{}", o),
            NoteName::E(o) => write!(f, "E{}", o),
            NoteName::F(o) => write!(f, "F{}", o),
            NoteName::FS(o) => write!(f, "F#{}", o),
            NoteName::G(o) => write!(f, "G{}", o),
            NoteName::GS(o) => write!(f, "G#{}", o),
            NoteName::A(o) => write!(f, "A{}", o),
            NoteName::AS(o) => write!(f, "A#{}", o),
            NoteName::B(o) => write!(f, "B{}", o),
        }
    }
}

impl NoteName {
    pub fn display_with_velocity(&self, vel: u8) {
        println!(
            "{} {} ({})",
            self,
            match vel / 16 {
                0 => "[       ]",
                1 => "[\u{2581}      ]",
                2 => "[\u{2581}\u{2582}     ]",
                3 => "[\u{2581}\u{2582}\u{2583}    ]",
                4 => "[\u{2581}\u{2582}\u{2583}\u{2584}   ]",
                5 => "[\u{2581}\u{2582}\u{2583}\u{2584}\u{2585}  ]",
                6 => "[\u{2581}\u{2582}\u{2583}\u{2584}\u{2585}\u{2586} ]",
                7 => "[\u{2581}\u{2582}\u{2583}\u{2584}\u{2585}\u{2586}\u{2587}]",
                _ => unreachable!(),
            },
            vel
        )
    }

    pub fn into_hue(&self) -> f32 {
        match self {
            NoteName::C(_) => 30.0,
            NoteName::CS(_) => 60.0,
            NoteName::D(_) => 90.0,
            NoteName::DS(_) => 120.0,
            NoteName::E(_) => 150.0,
            NoteName::F(_) => 180.0,
            NoteName::FS(_) => 210.0,
            NoteName::G(_) => 240.0,
            NoteName::GS(_) => 270.0,
            NoteName::A(_) => 300.0,
            NoteName::AS(_) => 330.0,
            NoteName::B(_) => 359.0,
        }
    }

    pub fn into_index(&self) -> u8 {
        match self {
            NoteName::C(oct) => oct * 12 + 0,
            NoteName::CS(oct) => oct * 12 + 1,
            NoteName::D(oct) => oct * 12 + 2,
            NoteName::DS(oct) => oct * 12 + 3,
            NoteName::E(oct) => oct * 12 + 4,
            NoteName::F(oct) => oct * 12 + 5,
            NoteName::FS(oct) => oct * 12 + 6,
            NoteName::G(oct) => oct * 12 + 7,
            NoteName::GS(oct) => oct * 12 + 8,
            NoteName::A(oct) => oct * 12 + 9,
            NoteName::AS(oct) => oct * 12 + 10,
            NoteName::B(oct) => oct * 12 + 11,
        }
    }

    pub fn into_scale_index(&self) -> u8 {
        match self {
            NoteName::C(_) => 0,
            NoteName::CS(_) => 1,
            NoteName::D(_) => 2,
            NoteName::DS(_) => 3,
            NoteName::E(_) => 4,
            NoteName::F(_) => 5,
            NoteName::FS(_) => 6,
            NoteName::G(_) => 7,
            NoteName::GS(_) => 8,
            NoteName::A(_) => 9,
            NoteName::AS(_) => 10,
            NoteName::B(_) => 11,
        }
    }

    pub fn into_lamp_id(&self) -> Option<u64> {
        match self.into_scale_index() {
            0 => Some(constants::LAMP_ID_JEAN),
            2 => Some(constants::LAMP_ID_STRIP_JEAN),
            _ => None,
        }
    }
}
