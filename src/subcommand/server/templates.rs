use {super::*, boilerplate::Display};

pub(crate) mod block;
pub(crate) mod ordinal;
pub(crate) mod output;
pub(crate) mod range;
pub(crate) mod root;
pub(crate) mod transaction;

#[derive(Display)]
pub(crate) struct PageHtml {
  content: Box<dyn Content>,
}

pub(crate) trait Content: Display {
  fn title(&self) -> String;

  fn page(self) -> PageHtml;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn page() {
    struct Foo;

    impl Display for Foo {
      fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<h1>Foo</h1>")
      }
    }

    impl Content for Foo {
      fn title(&self) -> String {
        "Foo".to_string()
      }

      fn page(self) -> PageHtml {
        PageHtml {
          content: Box::new(self),
        }
      }
    }

    assert_eq!(
      Foo.page().to_string(),
      "<!doctype html>
<html lang=en>
  <head>
    <meta charset=utf-8>
    <meta name=format-detection content='telephone=no'>
    <title>Foo</title>
    <link href=/static/index.css rel=stylesheet>
  </head>
  <body>
<h1>Foo</h1>
  </body>
</html>
"
    );
  }
}