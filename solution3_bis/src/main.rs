// Solution 3bis, on borrow la string. Mais on utilise &str dans la signature de la fn.
// En principe on va écrire la fn de cette manière.
// Dans rust il y a un truc qui se nomme Implicit Deref Coercion
// En gros quand tu prends une ref sur une String, tu obtiens un &str.
// The standard library provides an implementation of Deref on String that returns a string slice,
// and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str
// Je sais que ça va pas te plaire mais pour le moment, il faut admettre ce fait. (aie pas taper).
// Pour comprendre, il faut que l'on ait vu les Structs / impl et les traits.
// https://doc.rust-lang.org/std/ops/trait.Deref.html  --> explication de Deref
// https://doc.rust-lang.org/src/alloc/string.rs.html#2408 --> mais quand même en regardant un peu sous le capot
// le src de Deref pour String, on voit bien que la method deref renvoie un &str.  ;)

// Il se pourrait aussi que String soit en fait un Vec de bytes.... ;)

fn imil_say(sentence: &str) {
    //                  ^-- on change la signature de la fn pour dire
    //                  que l'on veut un une référence sur un slice (&str)
    //                  et la ça marche, car le &msg renvoie un &str pour la raison ci-dessus.
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
