use rand::seq::SliceRandom;
use std::time::Instant;
use std::time::Duration;

use crate::domain;
use crate::ui;

pub(crate) struct Game<'a> {
    words: &'a [domain::Word],
    timeout: Duration,
    flip: bool,
    shuffle: bool,
    start_time: Instant,

    ui: &'a dyn ui::Ui,
}

impl<'a> Game<'a> {
    pub(crate) fn new(words: &'a [domain::Word], timeout: Duration, flip: bool, shuffle: bool, ui: &'a dyn ui::Ui) -> Self {
        Game {
            words,
            timeout,
            flip,
            shuffle,
            start_time: Instant::now(),
            ui,
        }
    }

    pub(crate) fn run(&self) {
        let mut num_mistakes: i32 = 0;
        let mut num_words: i32 = 0;
        let word_order = self.word_order();

        for i in word_order {
            if self.is_game_over() {
                self.ui.game_over(num_words, num_mistakes);
                break;
            }

            let mut buffer = String::new();
            let mut attempts: i32 = 0;
            num_words += 1;

            let (prompt, answer, notes) = self.get_question(i);

            while answer != buffer {
                attempts += 1;
                if attempts > 1 {
                    self.ui.wrong_answer(answer);

                    if attempts > 5 {
                        break;
                    }
                }

                self.ui.start_question(
                    prompt,
                    self.time_remaining(),
                    num_words,
                    num_mistakes + attempts - 1,
                    notes,
                );

                buffer = self.ui.read_input();
            }

            num_mistakes += attempts - 1;

            self.ui.end_question();
        }
    }

    pub(crate) fn get_question(&self, index: usize) -> (&str, &str, Option<&str>) {
        if self.flip {
            (
                &self.words[index].left,
                &self.words[index].right,
                self.words[index].notes.as_deref(),
            )
        } else {
            (
                &self.words[index].right,
                &self.words[index].left,
                self.words[index].notes.as_deref(),
            )
        }
    }

    pub(crate) fn time_remaining(&self) -> Duration {
        self.timeout
            .checked_sub(self.start_time.elapsed())
            .unwrap_or_default()
    }

    pub(crate) fn is_game_over(&self) -> bool {
        self.start_time.elapsed() >= self.timeout
    }

    pub(crate) fn word_order(&self) -> Vec<usize> {
        let mut word_order: Vec<_> = (0..self.words.len()).collect();

        if self.shuffle {
            let mut rng = rand::thread_rng();
            word_order.shuffle(&mut rng);
        }

        word_order
    }
}
