use rand::seq::SliceRandom;
use std::time::Duration;

use crate::domain;
use crate::ui;

pub(crate) struct Game<'a> {
    words: &'a [domain::Word],
    timeout: Duration,
    flip: bool,
    shuffle: bool,

    ui: &'a dyn ui::Ui,
}

impl<'a> Game<'a> {
    pub(crate) fn new(words: &'a [domain::Word], timeout: Duration, flip: bool, shuffle: bool, ui: &'a dyn ui::Ui) -> Self {
        Game {
            words,
            timeout,
            flip,
            shuffle,
            ui,
        }
    }

    pub(crate) fn run(&self) {
        let mut state = domain::GameState::default();
        let word_order = self.word_order();

        for i in word_order {
            if self.is_game_over(&state) {
                self.ui.game_over(state.num_words, state.num_mistakes);
                break;
            }

            let mut buffer = String::new();
            let mut attempts: i32 = 0;
            state.num_words += 1;

            let question = self.get_question(i);

            while question.answer != buffer {
                attempts += 1;
                if attempts > 1 {
                    self.ui.wrong_answer(question.answer);

                    if attempts > 5 {
                        break;
                    }
                }

                self.ui.start_question(
                    question.prompt,
                    self.time_remaining(&state),
                    state.num_words,
                    state.num_mistakes + attempts - 1,
                    question.notes,
                );

                buffer = self.ui.read_input();
            }

            state.num_mistakes += attempts - 1;

            self.ui.end_question();
        }
    }

    fn get_question(&self, index: usize) -> domain::Question {
        if self.flip {
            domain::Question{
                prompt: &self.words[index].left,
                answer: &self.words[index].right,
                notes: self.words[index].notes.as_deref(),
            }
        } else {
            domain::Question{
                prompt: &self.words[index].right,
                answer: &self.words[index].left,
                notes: self.words[index].notes.as_deref(),
            }
        }
    }

    fn time_remaining(&self, state: &domain::GameState) -> Duration {
        self.timeout
            .checked_sub(state.start_time.elapsed())
            .unwrap_or_default()
    }

    fn is_game_over(&self, state: &domain::GameState) -> bool {
        state.start_time.elapsed() >= self.timeout
    }

    fn word_order(&self) -> Vec<usize> {
        let mut word_order: Vec<_> = (0..self.words.len()).collect();

        if self.shuffle {
            let mut rng = rand::thread_rng();
            word_order.shuffle(&mut rng);
        }

        word_order
    }
}
