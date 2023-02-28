use device_query::{DeviceState, Keycode, DeviceQuery};

static ALL_KEYS: [Keycode; 16] =
[
    Keycode::Key1,
    Keycode::Key2,
    Keycode::Key3,
    Keycode::Key4,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::R,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::F,
    Keycode::Z,
    Keycode::X,
    Keycode::C,
    Keycode::V,
];

pub struct Keyboard
{
    pub keys: [u8; 16]
}

impl Keyboard
{
    pub const fn new() -> Self
    {
        Self
        {
            keys: [0; 16]
        }
    }

    fn map_keycode(key: &Keycode) -> Option<usize>
    {
        match key
        {
            Keycode::Key1 => Some(0x1),
            Keycode::Key2 => Some(0x2),
            Keycode::Key3 => Some(0x3),
            Keycode::Key4 => Some(0xC),
            Keycode::Q    => Some(0x4),
            Keycode::W    => Some(0x5),
            Keycode::E    => Some(0x6),
            Keycode::R    => Some(0xD),
            Keycode::A    => Some(0x7),
            Keycode::S    => Some(0x8),
            Keycode::D    => Some(0x9),
            Keycode::F    => Some(0xE),
            Keycode::Z    => Some(0xA),
            Keycode::X    => Some(0x0),
            Keycode::C    => Some(0xB),
            Keycode::V    => Some(0xF),
            _ => None
        }
    }

    fn key_on(&mut self, key: &Keycode)
    {
        if let Some(index) = Keyboard::map_keycode(key)
        {
            self.keys[index] = 1;
        }
    }

    fn key_off(&mut self, key: &Keycode)
    {
        if let Some(index) = Keyboard::map_keycode(key)
        {
            self.keys[index] = 0;
        }
    }

    pub fn query_keystate(&mut self, device_state: &DeviceState)
    {
        let keys = device_state.get_keys();

        for key in ALL_KEYS.iter()
        {
            if keys.contains(key)
            {
                self.key_on(key);
            }
            else
            {
                self.key_off(key);
            }
        }
    }
}
