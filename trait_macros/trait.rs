struct Tweet {
    username: String,
    content: String,
}

struct Article {
    author: String,
    headline: String,
    content: String,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}: {}", self.headline, self.author, self.content)
    }
}

// Syntax Suger
fn print_summary(source: &impl Summary) {
    println!("{}", source.summarize());
}
// Type Bound Syntax
fn print_summary_trait_bound<T: Summary>(source: &T){
    println!("{}", source.summarize());
}

fn main(){
    let new_tweet = Tweet{
        username: String::from("@KunalSin9h"),
        content: String::from("Tweet Content"),
    };

    let new_article = Article {
        author: String::from("Kunal Singh"),
        headline: String::from("The Article, Wow"),
        content: String::from("The article content"),
    };

    print_summary(&new_tweet); 
    print_summary(&new_article);
    print_summary_trait_bound(&new_tweet); 
    print_summary_trait_bound(&new_article);
}

