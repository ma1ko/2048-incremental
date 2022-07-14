use rand::Rng;
use std::default;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("wordlist.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut rng = rand::thread_rng();

    let count = contents.lines().count();
    let index = rng.gen_range(0..count);
    let word = contents.lines().skip(index).take(1).collect::<Vec<_>>()[0];
    let mut hangman = Hangman::new(word.to_string());
    println!("{}", hangman);
    while !hangman.is_guess() {
        hangman.play();
    }

    Ok(())
}

struct Hangman {
    word: String,
    guessed: Vec<bool>,
    guesses: Vec<char>,
    wrong: usize,
}

impl Hangman {
    fn new(word: String) -> Self {
        let guessed = [false].into_iter().cycle().take(word.len()).collect();
        // println!("Word: {}", word);
        Hangman {
            word,
            guesses: Vec::new(),
            guessed,
            wrong: 0,
        }
    }
    fn is_guess(&self) -> bool {
        self.guessed.iter().all(|x| *x)
    }
    fn play(&mut self) {
        let input = std::io::stdin();
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        buf.trim().chars().for_each(|c| self.guess(c));
        println!("{}", self);
    }
    fn guess(&mut self, c: char) {
        // println!("Guessing: {}", c);
        if self.guesses.contains(&c) {
            println!("You already guessed {}", c);
            return;
        }
        if !self.word.contains(|ch| ch == c) {
            self.wrong += 1;
        }
        self.word
            .chars()
            .zip(&mut self.guessed)
            .filter(|x| !*x.1)
            .for_each(|(ch, g)| {
                if c == ch {
                    *g = true;
                }
            });
        self.guesses.push(c);
    }
    fn print_hangman(&self) -> [[char;7];5]{
        let _ = r"
  ---|
 /   o
 |  /|\
 |  / \
/ \ 
    ";
        let mut h = [[' '; 7]; 5];
        let g = self.wrong;
            if g > 0{h[4][0] ='/'; h[4][2] = '\\';}
            if g > 1  {h[3][1] ='|'; h[2][1] = '|';}
            if g > 2  {h[1][1] ='/'; }
            if g > 3  {h[0][2] ='-'; h[0][3] = '-';h[0][4] = '-';}
            if g > 4  {h[0][5] ='|'; }
            if g > 5  {h[1][5] ='o'; }
            if g > 6  {h[2][4] ='/'; h[2][5] = '|'; h[2][6] = '\\';}
            if g > 7  {h[3][4] ='/'; h[3][6] = '\\';}
            if g > 8  {h[1][5] ='x';}
        h
    }
}
impl Display for Hangman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Guesses: {:?}", self.guesses);
        let h = self.print_hangman();
        let _ = h.iter().map(|l| {
            let _ = l.iter().map(|c| {
                write!(f, "{}", c)
            }).collect::<Vec<_>>();
            writeln!(f, "")

        }).collect::<Vec<_>>();

        let _ = self
            .word
            .chars()
            .zip(&self.guessed)
            .map(|(ch, g)| {
                if *g {
                    write!(f, "{} ", ch)
                } else {
                    write!(f, "_ ")
                }
            })
            .collect::<Vec<_>>();
        writeln!(f, "")
    }
}

