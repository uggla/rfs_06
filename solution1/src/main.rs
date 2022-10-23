// Solution 1 relou, on fait une fonction avec la String en retour.
//    On ajoute un type de retour --v
fn imil_say(sentence: String) -> String {
    println!("{}", sentence);
    sentence
    // ^-- pas de ; pour return implicit.
}

fn main() {
    let msg1 = String::from("RFS rules !");

    let msg2 = imil_say(msg1);

    // Say it again loud !
    imil_say(msg2);
}
