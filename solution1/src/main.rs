// Solution 1 relou, on fait une fonction avec la String en retour.
//    On ajoute un type de retour --v
fn imil_say(sentence: String) -> String {
    println!("{}", sentence);
    sentence
    // ^-- pas de ; pour return implicit.
}

fn main() {
    let msg = String::from("RFS rules !");

    let msg = imil_say(msg);

    // Say it again loud !
    imil_say(msg);
    //        ^--- le msg ici est celui de la ligne 12
}
