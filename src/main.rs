// struct Point<X1,Y1> {
//     x:X1,
//     y:Y1,
// }

// impl<X1, Y1> Point<X1, Y1>{
//     fn mixup<X2, Y2>(self,other: Point<X2,Y2>) -> Point<X1,Y2> {
//         Point{
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main(){
//     let p1 = Point{x:5,y:10.4};
//     let p2 = Point{x:"Hello",y:'c'};
//     let p3 = p1.mixup(p2);
//     println!("p3.x = {}, p3.y = {}",p3.x,p3.y);
// }

use func_exec::{NewsArticle,SocialPost,Summary};
// use func_exec::SocialPost;

fn main(){
    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championships!"),
        location: String::from("Pittsburgh,PA,USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pits penguins once again are the best hockey team in the NHL",
        ),
    };
    println!("News article available! {}",article.summarize());

    let post = SocialPost{
        username: String::from("house_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}",post.summarize());
}