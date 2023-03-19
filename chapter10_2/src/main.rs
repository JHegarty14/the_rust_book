mod aggregator;
use aggregator::{
    notify,
    notify_trait_bound,
    notify_multi,
    notify_multi_trait_bound,
    notify_multi_trait,
    notify_multi_trait_alt,
    some_function_verbose,
    some_function,
    returns_summarizable,
    Summary,
    Tweet,
    Zine
};


fn main() {
    let tweet = Tweet {
        username: String::from("wint"),
        content: String::from("IF THE ZOO BANS ME FOR HOLLERING AT THE ANIMALS I WILL FACE GOD AND WALK BACKWARDS INTO HELL"),
        retweet: false,
        reply: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let zine = Zine {
        issue: 3,
        author: String::from("A 22 year old crust punk"),
        content: String::from("asdf jkl; asdf jkl;"),
    };

    println!("New issue of your zine! {}", zine.summarize());

    notify(&tweet);

    notify_trait_bound(&zine);

    notify_multi(&tweet, &zine);

    // both args must be same concrete type
    // notify_multi_trait_bound(&tweet, &zine);

    let tweet2 = Tweet {
        username: String::from("wint"),
        content: String::from("no"),
        retweet: false,
        reply: false,
    };

    notify_multi_trait_bound(&tweet, &tweet2);

    notify_multi_trait(&tweet2);

    notify_multi_trait_alt(&tweet2);

    some_function(&tweet2, &zine);

    some_function_verbose(&tweet2, &zine);

    let new_summarizable = returns_summarizable();

    println!("summarizable: {}", new_summarizable.summarize());
}
