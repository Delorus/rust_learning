use crate::blog::Post;

fn main() {
    rust_way_state_pattern();
}

fn rust_way_state_pattern() {
    let expected_str = "I ate a salad for lunch today";
    let mut post = Post::new();

    post.add_text(expected_str);

    let post = post.request_preview();

    let post = post.approve();

    assert_eq!(expected_str, post.content());
}

mod blog {

    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_preview(self) -> PendingReviewPost {

        }
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}