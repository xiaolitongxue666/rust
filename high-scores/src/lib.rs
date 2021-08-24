#[derive(Debug)]
pub struct HighScores<'a> {
    hig_scores : &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            hig_scores : scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        println!("Input scores data is : {:?} ." , self.hig_scores);
        self.hig_scores
    }

    pub fn latest(&self) -> Option<u32> {

        if self.hig_scores.is_empty() {
            println!("Input scores no data");
            None
        } else {
            let mut latest = self.hig_scores[0];
            let length = self.hig_scores.len();

            for i in 0..length {
                let temp_scores = self.hig_scores[i];
                println!("Input scores data index {} data is {:?}, latest is {} .", i, temp_scores, latest);
                if latest > temp_scores && latest !=0 && temp_scores != 0 {
                    latest = temp_scores;
                }
            }
            Some(latest)
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.hig_scores.is_empty() {
            println!("Input scores no data");
            None
        } else {
            let mut best = self.hig_scores[0];
            let length = self.hig_scores.len();

            for i in 0..length {
                let temp_scores = self.hig_scores[i];
                println!("Input scores data index {} data is {:?}, best is {} .", i, temp_scores, best);
                if best < temp_scores && best !=0 && temp_scores != 0 {
                    best = temp_scores;
                }
            }
            Some(best)
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three : Vec<u32> = Vec::new();

        if self.hig_scores.is_empty() {
            println!("Input scores no data");
        } else {
            let length = self.hig_scores.len();

            for i in 0..length {
                let temp_scores = self.hig_scores[i];
                println!("Input scores data index {} data is {:?} .", i, temp_scores);

                if temp_scores != 0 {
                    if top_three.len() < 3 {
                        top_three.push(temp_scores)
                    } else {
                        for j in 0..3 {
                            if temp_scores > top_three[j] {
                                top_three[j] = temp_scores;
                                break;
                            }
                        }
                    }
                }
            }
        }
        top_three.sort();
        println!("Sort top three is : {:?} ." , top_three);
        top_three.reverse();
        println!("Reverse top three is : {:?} ." , top_three);
        top_three
    }
}