use rand::seq::SliceRandom;



#[derive(Debug)]
struct Deck{
    cards:Vec<String>,
}

impl Deck{
    fn new() -> Deck{

        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        

        let ranks = [
            "Ace", "2", "3", "4", "5", "6", "7", 
            "8", "9", "10", "Jack", "Queen", "King"
        ];
        let mut pack = vec![];

        for suit in suits{
            for rank in ranks{
               let card= format!("{} of {}",rank,suit);
               pack.push(card);
            }
        }

        Deck{cards:pack}
    }

    fn shuffle(&mut self) {
        let mut rnd = rand::rng();
        self.cards.shuffle(&mut rnd);
    }

    fn deal(&mut self,num:usize) -> Vec<String>{
        self.cards.split_off(self.cards.len()-num)
    }
}

fn main(){
 let mut result = Deck::new();

 result.shuffle();
 
 let result2 = result.deal(5);
 println!("{:#?}",result);
 println!("{:#?}",result2);

}