use std::error::Error;
use std::str;
use rand;
use noise::{NoiseFn, Perlin};

pub enum BoxDrawing{
    Vertical,
    Horizontal,
    Topleft,
    TopRight,
    BottomLeft,
    BottomRight,
    JunctionRight,
    JunctionLeft,
    JunctionDown,
    JunctionUp,
    JunctionCross,
    MusicalNote,
    Void
}

impl BoxDrawing {
    pub fn get_utf8(&self) -> String {
        let drawing = match self {
            BoxDrawing::Vertical => vec![0xe2,0x94,0x82],
            BoxDrawing::Horizontal => vec![0xe2,0x94,0x80],
            BoxDrawing::Topleft => vec![0xe2,0x94,0x8c],
            BoxDrawing::TopRight => vec![0xe2,0x94,0x94],
            BoxDrawing::BottomLeft => vec![0xe2,0x94,0x90],
            BoxDrawing::BottomRight => vec![0xe2,0x94,0x98],
            BoxDrawing::JunctionRight => vec![0xe2,0x94,0x9c],
            BoxDrawing::JunctionLeft => vec![0xe2,0x94,0xa4],
            BoxDrawing::JunctionDown => vec![0xe2,0x94,0xac],
            BoxDrawing::JunctionUp => vec![0xe2,0x94,0xb4],
            BoxDrawing::JunctionCross => vec![0xe2,0x94,0xbc],
            BoxDrawing::MusicalNote => vec![226,151,137],
            BoxDrawing::Void => vec![32]
        };

        let drawing = str::from_utf8(&drawing).expect("UTF 8 have some trouble!");
        drawing.to_owned()
    }
}

struct Dimension {
    width: u8,
    height: u8
}

impl Dimension{
    fn new(width: u8, height: u8) -> Dimension {
        Dimension{width, height}
    }
}

pub fn generate_perlin(size: usize) -> Vec<f64> {
    let perlin = Perlin::new();
    let ran_y = rand::random();
    let mut values: Vec<f64> = Vec::with_capacity(size);
    for i in 0..size {
        values.push(perlin.get([(i as f64)*0.05, ran_y]))
    };
    values
}

pub struct OcTable {
    lines: Vec<u8>,
    dimension: Dimension,
    // notes: Vec<u8>
}

impl OcTable{
    pub fn new() -> OcTable {
        OcTable{
            lines: vec![1,3,5,7,9],
            dimension: Dimension::new(65,15),
            
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn Error>>{
        let mut draw: BoxDrawing;
        for row in 0..self.dimension.height {
            for column in 0..self.dimension.width {
                draw = if self.lines.contains(&row){
                    if column==0{
                        BoxDrawing::JunctionRight
                    }else if column==self.dimension.width-1{
                        BoxDrawing::JunctionLeft
                    }else if column%16==0{
                        BoxDrawing::JunctionCross
                    }else{
                        BoxDrawing::Horizontal
                    }
                }else{
                    if column==0||column==self.dimension.width-1 || column%16==0{
                        BoxDrawing::Vertical
                    }else{
                        BoxDrawing::Void
                    }
                };
            
                print!("{}", draw.get_utf8()); 
            }
            println!();
        }
        println!("\n");
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawing_junction_left() {
        let left = BoxDrawing::JunctionLeft.get_utf8();
        let right = str::from_utf8(&[0xe2,0x94,0xa4]).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn drawing_bottom_right_instead_vertical() {
        let left = BoxDrawing::BottomRight.get_utf8();
        let right = str::from_utf8(&[0xe2,0x94,0x80]).unwrap();
        assert_ne!(left, right);
    }

    #[test]
    fn table_drawed() {
        let ocarina = OcTable::new();

        assert!(ocarina.render().is_ok());
    }

    #[test]
    fn generate_10_perlin_value() {
        let perlins = generate_perlin(10);
        assert_eq!(perlins.len(), 10);  
    }

    #[test]
    fn generate_34_perlin_value() {
        let perlins = generate_perlin(34);
        assert_ne!(perlins.len(), 10);
    }

    #[test]
    fn generate_perlin_in_valid_range() {
        let perlins = generate_perlin(24);
        assert!(perlins.iter().all(|&v| (v >= -1f64 || v <= 1f64)));
    }
    
    #[test]
    fn generate_16_notes() {
        let oc = OcTable::new();
        oc.generate_notes(vec![0.3345987659, 0.4756790987, 0.6567890987])
    }
}

