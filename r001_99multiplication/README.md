# R001 99 multiplication

## Learn
- When you use readline and want to pare the value your input, you should notice the "newline".
  ``` rust
   input.get(..input.len() - 2).unwrap().parse::<u8>().is_ok()
  ```
- There are many powerful formatting macros in the rust.
- If you want to dynamic align the output, you can use the variable in macro. It is declared like "var$".
  ```rust
   println!(
            "{:>width$} x {:^width$} = {:>width2$}",
            j,
            i,
            i * j,
            width = w,
            width2 = w2
        );
  ```