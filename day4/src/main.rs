fn main() {
    let input_str = include_str!("input.txt").lines().map(|str| {
        let res = str.split_once(":").unwrap().1
        .split_once("|").unwrap();
        (res.0.split_whitespace().collect::<Vec<&str>>(), res.1.split_whitespace().collect::<Vec<&str>>())
    });

    let card_vec = input_str.clone().collect::<Vec<(Vec<&str>, Vec<&str>)>>();

    let sum: usize = input_str.enumerate().map(|(id, (winning_cards,cards))| {
        let mut winners: usize = 0;

        for card in cards.iter() {
            if winning_cards.contains(&card) {
                winners += 1;
            }
        }

        if winners != 0 {
            let slice = card_vec.get(id+1..id+winners+1).unwrap();
            process_tickets(slice,&card_vec, id+1)
        } else {
            0
        }

    }).sum::<usize>() + card_vec.len();

    println!("{sum}");
    //valid part 1 sol 
}

fn process_tickets(slice: &[(Vec<&str>, Vec<&str>)], inputs: &Vec<(Vec<&str>, Vec<&str>)>, rel_id:usize) -> usize {
    slice.iter().enumerate().map( |(id, (winning_cards, cards))| {
        let id = rel_id + id;
        let mut winners: usize = 0;
        
        for card in cards.iter() {
            if winning_cards.contains(&card) {
                winners += 1;
            }
        }
        
        if winners != 0 {
            let slice = inputs.get(id+1..id+winners+1).unwrap();
            process_tickets(slice, &inputs, id+1)
        } else {
            0
        }
    }).sum::<usize>() + slice.len()
}