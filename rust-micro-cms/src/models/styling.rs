use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Style {
    pub id: i64,
    pub name: String,
    pub css: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NewStyle {
     pub name: String,
    pub css: String,
}


impl Default for Style {
    fn default() -> Self {
        let css = r#"
        
        body {
            margin: 0;
            padding: 0;
        }
        
        nav {
            position: fixed;
            top: 0;
            left: 0;
            width: 30%;
            height: 100%;
            border-right: 2px solid #ccc;
            padding: 20px;
            box-sizing: border-box; 
        }
        
        .content {
            margin-left: calc(30% + 2px); 
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
        
        .posts {
            position: absolute; 
            top: 0;             
            left: 30%;          
            width: 70%;         
            padding: 20px;
            box-sizing: border-box; 
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
        textarea#style_update_text {
            width: 100%;
            height: 300px; 
            box-sizing: border-box;
        }
    
        
        .form-buttons {
            display: flex;
            justify-content: space-between;
            margin-top: 10px;
        }
    
        
        .form-buttons input {
            width: 48%; 
        }
        .submit-button {
            background-color: green;
            color: white;
            border: none;
            padding: 10px;
            border-radius: 5px;
            cursor: pointer;
        }
    
        
        .reset-button {
            background-color: red;
            color: white;
            border: none;
            padding: 10px;
            border-radius: 5px;
            cursor: pointer;
        }
    
        
        .submit-button:hover {
            background-color: darkgreen;
        }
    
        .reset-button:hover {
            background-color: darkred;
        }
        "#
        .to_string();

        Style { 
            id: i64::MAX,
            name: "template".to_string(),
            css 
        }
    }
}
