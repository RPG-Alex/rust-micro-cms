use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Style {
    pub css: String,
}

impl Default for Style {
    fn default() -> Self {
        let css = r#"
        /* Ensuring the body has no margin or padding */
        body {
            margin: 0;
            padding: 0;
        }
        /* Fixed navbar with specified width and a solid border */
        nav {
            position: fixed;
            top: 0;
            left: 0;
            width: 30%;
            height: 100%;
            border-right: 2px solid #ccc;
            padding: 20px;
            box-sizing: border-box; /* Includes padding and border in the width */
        }
        /* Content offset from the left by the width of the navbar */
        .content {
            margin-left: calc(30% + 2px); /* Accounts for the border width */
            padding: 20px;
            box-sizing: border-box;
        }
        ul.links {
            list-style-type: none;
            padding: 0;
        }
        ul.links li::before {
            content: "» » ";
            color: #333;
        }
        /* Styling for posts */
        .posts {
            position: absolute; /* Ensures the posts div is positioned independently of other elements */
            top: 0;             /* Aligns the top of the div with the top of the viewport */
            left: 30%;          /* Offsets the div from the left by the width of the navbar */
            width: 70%;         /* Allocates the remaining 70% of the viewport width to the posts div */
            padding: 20px;
            box-sizing: border-box; /* Ensures padding is included within the width */
        }
        .post {
            background-color: #fff;
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
            margin: 20px 0;
            padding: 15px;
            border-radius: 8px;
        }
        .post h2 {
            font-size: 20px;
            color: #333;
        }
        .post p {
            color: #666;
            line-height: 1.6;
            overflow: hidden;
            text-overflow: ellipsis;
            display: -webkit-box;
            -webkit-line-clamp: 3;
            -webkit-box-orient: vertical;
        }
        .post .read-more {
            color: #0077cc;
            cursor: pointer;
            margin-top: 10px;
        }
        
        "#
        .to_string();

        Style { css }
    }
}
