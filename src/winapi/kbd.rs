#[derive(Debug, Clone, Copy)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub caps: bool,
}

impl From<&pc_keyboard::Modifiers> for KeyMods {
    fn from(m: &pc_keyboard::Modifiers) -> Self {
        Self {
            shift: m.is_shifted(),
            ctrl:  m.is_ctrl(),
            alt:   m.is_alt() || m.is_altgr(),
            caps:  m.is_caps(),
        }
    }
}
