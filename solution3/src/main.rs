// Solution 3, on borrow la string.

fn imil_say(sentence: &String) {
    //                  ^-- on change la signature de la fn pour dire
    //                  que l'on veut une référence ("pointeur") sur la String.
    println!("{}", sentence);
}

fn main() {
    let msg = String::from("RFS rules !");
    imil_say(&msg);
    //         ^-- on passe la référence

    // Say it again loud !
    imil_say(&msg);
    //         ^-- on passe la référence
}
