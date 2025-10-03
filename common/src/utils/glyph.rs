use std::collections::HashMap;

#[derive(Default)]
pub struct Glyph {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub x_offset: i16,
    pub y_offset: i16,
    pub x_advance: u16,
}

pub struct GlyphData {
    pub map: HashMap<u32, Glyph>,
}

impl GlyphData {
    pub fn get(&self, character: char) -> &Glyph {
        self.map
            .get(&(character as u32))
            .expect(&format!("Glyph {} not found", character))
    }

    /**
     * BMFont text-based font descriptor (.fnt file)
     * Each glyph is specified by a line with the format below.
     * char id=38 x=92 y=64 width=9 height=14 xoffset=0 yoffset=3 xadvance=8 page=0 chnl=15
     *
     * See https://snowb.org/ for more info on the data format.
     */
    pub fn from_fnt_file(file_src: &str) -> Self {
        let mut characters = HashMap::new();
        for line in file_src.lines() {
            let mut parts = line.split_ascii_whitespace();
            if parts.next().expect("Empty line in font file") != "char" {
                continue;
            }

            let mut block: Glyph = Default::default();
            for kv in parts {
                let mut split = kv.split('=');
                let key = split.next().unwrap();
                let val = split.next().unwrap().trim_matches('"');
                match key {
                    "id" => block.id = val.parse().unwrap(),
                    "x" => block.x = val.parse().unwrap(),
                    "y" => block.y = val.parse().unwrap(),
                    "width" => block.width = val.parse().unwrap(),
                    "height" => block.height = val.parse().unwrap(),
                    "xoffset" => block.x_offset = val.parse().unwrap(),
                    "yoffset" => block.y_offset = val.parse().unwrap(),
                    "xadvance" => block.x_advance = val.parse().unwrap(),
                    _ => {}
                }
            }
            characters.insert(block.id, block);
        }

        GlyphData { map: characters }
    }
}
