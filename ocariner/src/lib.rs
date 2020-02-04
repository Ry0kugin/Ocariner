use std::{
    str,
    io,
    io::Write,
    thread,
    error::Error,
    time::Duration
};
use rand;
use noise::{NoiseFn, Perlin};

pub enum BoxDrawing {
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

pub enum Arrow {
    UpArrow,
    UpArrowDouble,
    UpArrowFilled,
}

impl Arrow {
    pub fn get_utf8(&self) -> String {
        let arrow = match self {
            Arrow::UpArrow => vec![0xe2,0x86,0x91],
            Arrow::UpArrowDouble => vec![0xe2,0x87,0x91],
            Arrow::UpArrowFilled => vec![0xe2,0xac,0x86],
        };

        let drawing = str::from_utf8(&arrow).expect("UTF 8 have some trouble!");
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
        values.push(perlin.get([(i as f64)*0.3, ran_y]))
    };
    values
}

pub struct OcTable {
    lines: Vec<u8>,
    dimension: Dimension,
    notes: Vec<u8>,
    tempo: f32,
}

impl OcTable{
    pub fn new(tempo: f32) -> OcTable {
        let notes_len = 16;
        OcTable{
            lines: vec![1,3,5,7,9],
            dimension: Dimension::new(65,15),
            notes: Vec::with_capacity(notes_len),
            tempo
        }
    }

    pub fn generate_notes(&mut self) {
        let notes = generate_perlin(16);
        self.notes = notes.iter().map(|&v| ((v+1f64)*6.5f64) as u8).collect();
    }

    pub fn render(&self) -> Result<(), Box<dyn Error>>{
        let mut draw: BoxDrawing;
        let mut draw_note: bool; 
        let mut note_ct = 0;
        for row in 0..self.dimension.height {
            let notes: Vec<usize> = self.notes.iter()
                .enumerate()
                .filter(|(_, &note)| note == row)
                .map(|(i,_)| i)
                .collect();
            for column in 0..self.dimension.width{
                draw_note = ((column as i8)%4)-2==0;
                draw = if draw_note && notes.contains(&note_ct) {
                    BoxDrawing::MusicalNote
                } else if self.lines.contains(&row){
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
                if draw_note {note_ct+=1}
                print!("{}", draw.get_utf8()); 
            }
            println!();
            note_ct = 0
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.generate_notes();
        self.render()?;
        for i in 0..self.notes.len(){
            let progression = format!("{}{}", " ".repeat(2+4*i), Arrow::UpArrowFilled.get_utf8());
            print!("{}\x0d\x07", progression);
            io::stdout().flush().expect("flushing failed");
            thread::sleep(Duration::from_secs_f32(self.tempo));
        } 
        Ok(())
    }
}

pub struct Config {
    tempo: f32,
    difficulty: u8,
    wait: u8,
    notes: (u8,u8),
    ascii: bool,
    series: u8,
    measures: u8,
    time_per_measures: u8
}

impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        let args: Vec<String> = args.collect();
        let mut tempo: f32 = 0.6;
        let mut difficulty: u8 = 3;
        let mut wait: u8 = 4;
        let     notes: (u8,u8) = (0,12);
        let mut ascii: bool = false;
        let mut series: u8 = 0;
        let mut measures: u8 = 4;
        let mut time_per_measures: u8 = 4;

        if args.len() == 1 {
            return Ok(Config{tempo, difficulty, wait, notes, ascii, series, measures, time_per_measures}) 
        }else if args.len() > 16 {
            return Err("Error parsing arguments: too much arguments")
        } else if args[1].clone() == "-h" || args[1].clone() == "--help" {
            return Err("HELP!!")
        } else {
            let mut iterator = args.iter();
            iterator.next();
            for _ in 0..args.len()-1 {
                match iterator.next() {
                    Some(arg) => {
                        if arg == "-t" || arg == "--tempo" {
                            match iterator.next() {
                                Some(value) => {
                                    tempo = match value.trim().parse() {
                                        Ok(num) => {
                                            if num >= 0.3 && num <= 1.5 {
                                                num
                                            } else {
                                                return Err("Error parsing argument: -t must be between 0.3 and 1.5")
                                            }
                                        }
                                        Err(_) => return Err("Error parsing argument: invalid argument"),
                                    };
                                },
                                None => return Err("Error parsing argument: Not enough argument")
                            }
                        } else 
                        if arg == "-d" || arg == "--difficulty" {
                            match iterator.next() {
                                Some(value) => {
                                    difficulty = match value.trim().parse() {
                                        Ok(num) => {
                                            if num >= 1 && num <= 10 {
                                                num
                                            } else {
                                                return Err("Error parsing argument: -t must be between 1 and 10")
                                            }
                                        },
                                        Err(_) => return Err("Error parsing argument: invalid argument"),
                                    };
                                },
                                None => return Err("Error parsing argument: Not enough argument")
                            }
                        } else 
                        if arg == "-w" || arg == "--wait" {
                            match iterator.next() {
                                Some(value) => {
                                    wait = match value.trim().parse() {
                                        Ok(num) => {
                                            if num <= 8 {
                                                num
                                            } else {
                                                return Err("Error parsing argument: -t must be between 1 and 10")
                                            }
                                        },
                                        Err(_) => return Err("Error parsing argument: invalid argument"),
                                    };
                                },
                                None => return Err("Error parsing argument: Not enough argument")
                            }
                        } else 
                        // if arg == "-n" || arg == "--notes" {
                        //     match iterator.next() {
                        //         Some(value) => {
                        //             notes = match value.trim().parse() {
                        //                 Ok(num) => num,
                        //                 Err(_) => return Err("Error parsing argument: invalid argument"),
                        //             };
                        //         },
                        //         None => return Err("Error parsing argument: Not enough argument")
                        //     }
                        // } else 
                        if arg == "-a" || arg == "--ascii" {
                            ascii = true;
                        } else 
                        if arg == "-s" || arg == "--series" {
                            match iterator.next() {
                                Some(value) => {
                                    series = match value.trim().parse() {
                                        Ok(num) => num,
                                        Err(_) => return Err("Error parsing argument: invalid argument"),
                                    };
                                },
                                None => return Err("Error parsing argument: Not enough argument")
                            }
                        } else 
                        if arg == "-m" || arg == "--measure" {
                            match iterator.next() {
                                Some(value) => {
                                    measures = match value.trim().parse() {
                                        Ok(num) => {
                                            if num >= 1 && num <= 8 {
                                                num
                                            } else {
                                                return Err("Error parsing argument: -t must be between 1 and 8")
                                            }
                                        },
                                        Err(_) => return Err("Error parsing argument: invalid argument"),
                                    };
                                },
                                None => return Err("Error parsing argument: Not enough argument")
                            }
                        } else 
                        if arg == "-T" || arg == "--time" {
                            match iterator.next() {
                                Some(value) => {
                                    time_per_measures = match value.trim().parse() {
                                        Ok(num) => {
                                            if num >= 1 && num <= 8 {
                                                num
                                            } else {
                                                return Err("Error parsing argument: -t must be between 1 and 8")
                                            }
                                        },
                                        Err(_) => return Err("Error parsing argument: invalid argument"),
                                    };
                                },
                                None => return Err("Error parsing argument: Not enough argument")
                            }
                        } else {
                            return Err("Error parsing argument: invalid argument")
                        }
                    }
                    None => break  
                }
            }
        }

        println!("-> {}\n-> {}\n-> {}\n-> {:?}\n-> {}\n-> {}\n-> {}\n-> {}\n",
            tempo, difficulty, wait, notes, ascii, series, measures, time_per_measures);
        Ok(Config{tempo, difficulty, wait, notes, ascii, series, measures, time_per_measures}) 
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
    fn drawing_up_arrow_double() {
        let left = Arrow::UpArrowDouble.get_utf8();
        let right = str::from_utf8(&[0xe2,0x87,0x91]).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn drawing_up_arrow_instead_up_arrow_filled() {
        let left = Arrow::UpArrow.get_utf8();
        let right = str::from_utf8(&[0xe2,0xac,0x86]).unwrap();
        assert_ne!(left, right);
    }

    #[test]
    fn table_drawed() {
        let ocarina = OcTable::new(0.8);

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
        let mut oc = OcTable::new(0.6);
        oc.generate_notes();
        assert!(oc.notes.iter().all(|&v| (v < 13u8)));
        assert_eq!(oc.notes.len(), 16)
    }

    #[test]
    fn run_table() {
        let mut oc = OcTable::new(0.01);
        assert!(oc.run().is_ok());
    }

}

