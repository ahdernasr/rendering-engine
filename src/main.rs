pub mod htmlparser;

const HTML_DATA: &str = r#"
<html>
    <body>
        <!-- This is a comment -->
        <h1>Title</h1>
        <div id="main" class="test">
            <img src="test.png" />
            <p>Hello <em>world</em>!</p>
         </div>
    </body>
</html>
"#;

fn main() {
    println!("{:?}", htmlparser::parse(HTML_DATA.to_string()));
}