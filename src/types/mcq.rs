use std::io;
/// Question struct is the foundation of the app, any question generated should be in this format
#[derive(Debug)]
pub struct Question {
    /// It contains the question
    pub question: String,
    /// The options are stored in a Vector, here order matters
    pub options: Vec<String>,
    /// The correct answer corresponds to the index in the options vector
    pub correct: u8,
    /// An optional type, used for later storing the user selected option for analysis
    pub selected: Option<u8>,
    /// Checks wether the user has seen the answer or not
    pub checked: bool,
}

impl Question {
    /// Evaluate the correct answer by checking with index of the option
    pub fn evaluate(&self, index: u8) -> bool {
        if self.correct == index {
            return true;
        } else {
            return false;
        };
    }
}

/// The app's foundational struct, it contains all the questions and information.
#[derive(Debug)]
pub struct Mcq {
    /// The current question the user is answering in the questions vector
    pub index: u8,
    /// Contains all questions inside a vector
    pub questions: Vec<Question>,
    /// The correct no of questions answered
    pub correct_no: u8,
    /// no of incorrect questions answered
    pub incorrect_no: u8,
}

impl Mcq {
    /// Initializes a  new Mcq struct
    pub fn new(vec: Vec<Question>) -> Self {
        Self {
            index: 0,
            questions: vec,
            correct_no: 0,
            incorrect_no: 0,
        }
    }
    /// Evaluates the answer
    pub fn next(&mut self, answer: u8) {
        if self.questions[self.index as usize].evaluate(answer) {
            self.correct_no += 1;
        } else {
            self.incorrect_no += 1;
        }
    }

    pub fn print(&mut self) {
        let len = self.questions.len() as u8 - 1;
        loop {
            println!(
                "{}. {}\n",
                self.index + 1,
                self.questions[self.index as usize].question
            );
            for (i, option) in self.questions[self.index as usize]
                .options
                .iter()
                .enumerate()
            {
                println!("{}) {}", i + 1, option);
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if len == self.index {
                self.next(input.trim().parse::<u8>().unwrap());
                break;
            } else {
                self.next(input.trim().parse::<u8>().unwrap());
            }
        }
    }

    /* fn parse_ai(prompt: String) -> Vec<Question> {
        let response = "prompt";
        let mut db: Vec<Question> = Vec::new();
        let split_question: Vec<&str> = response.split("\n\n").collect();
        for question in split_question {
            let options: Vec<&str> = question.split("\n").collect();
            let try_options = vec![
                options[1].to_string(),
                options[2].to_string(),
                options[3].to_string(),
                options[4].to_string(),
            ];
            db.push(Question {
                question: options[0].to_string(),
                options: try_options,
                correct: options[5].parse::<u8>().unwrap(),
                selected: None,
            });
        }
        db
    } */
}
