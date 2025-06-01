# Macros
Macros in Rust are a way to write code that writes other code, allowing for reusable and concise patterns during compilation.
Writing macros on top of `enum` or `struct` ,it will add implementation on struct or enum automatically not necessary to write code for that.

### What i Learned
- Macros help to write less code 
- We can add `cargo add cargo-expand` so we can also check what is happening when code is compiling
- Write some code in `lib` file and add some macros on top of struct or enum or where is needed.
- For this code :
  ```rust

  #[derive(Debug)]
  struct Rect {
    width: u32,
    height: u32,
  }

  fn main() {
    println!("Hello, world!");

    let rect = Rect {
        height: 32,
        width: 43,
    };

    println!("Rectangle width : {}", rect.width);
    println!("Rec height : {}", rect.height);

    println!("Rect: {:?}", rect);
  }
  ```


- Compile in this format after run **`cargo expand`**
 
      
    ```rust
      #![feature(prelude_import)]
      #[prelude_import]
      use std::prelude::rust_2024::*;
      #[macro_use]
      extern crate std;
      struct Rect {
          width: u32,
          height: u32,
      }
    ```
    <details style={{text:}}>    
     <summary><strong>👉 Click to show the rest 👈</strong></summary>

     ```rust

      #[automatically_derived]
      impl ::core::fmt::Debug for Rect {
          #[inline]
          fn fmt(&self, f: &mut ::core::fmt::Formatter) ->     ::core::fmt::Result {
              ::core::fmt::Formatter::debug_struct_field2_finish(
                  f,
                  "Rect",
                  "width",
                  &self.width,
                  "height",
                  &&self.height,
              )
          }
      }
      fn main() {
          {
              ::std::io::_print(format_args!("Hello, world!\n"));
          };
          let rect = Rect { height: 32, width: 43 };
          {
              ::std::io::_print(format_args!("Rectangle width : {0}    \n", rect.width));
          };
          {
              ::std::io::_print(format_args!("Rec height : {0}\n",     rect.height));
          };
          {
        ::std::io::_print(format_args!("Rect: {0:?}\n", rect));
          };
      }
    ```
   </details> 
 
  
- Lifetime is the concepts that follow ownership rule . 
     - when we define lifetime on struct or enum or where is needed , so it create a rule for struct or enum when its ownership will be keep.
       ```rust
        struct LRect<'a, 'b> {
           width: u32,
           height: u32,
           name: &'a String,
       
           last_name: &'b String,
         }
        
        struct Shape<'a, 'b> {
            rect1: LRect<'a, 'b>,
            rect2: LRect<'a, 'b>,
         }
       ```