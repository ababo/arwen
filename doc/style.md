### General coding style:

- Names should not be long, but self-descriptive. Longer name is always better than some cryptic abbreviation.
- Prefer hyphens over underscores while naming files.
- Last line of each text file should be terminated by a line-feed.

### Rust coding style:

- Use coding style presented in *Rust* official documentation unless it contradicts rules below.
- File lines should not exceed 80 characters.
- Tab characters are prohibited. All tabs should be represented by 4-space sequences.
- Long function argument lists must be aligned, e.g.

    ```rust
    fn function_name(argument1: ArgType1, argument2: ArgType2,
                     argument3: ArgType3) { ...
    ```

### Markdown style:

- Paragraphs should be continuous without limitation in line width.
