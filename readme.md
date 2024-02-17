pond — a minimal and fast static site generator
===

### usage 
just add all your blogs in .md format to the content directory then the dist folder will have your website

then run `cargo run` making sure you have rust installed 

### current things
[ ] - adding an user focused interface to create markdown files 
[ ] - adding more metadata on the article links on homepage
[ ] - modular styles using colored variants of pico or extracting basic colors to make a basic template 
[x] - creating a search feature for main page 
[x] - setting up gh actions 
[x] - creating a toml config

## goals
- online editor
- adding color themes
- To make it standard markdown compatible
- content first approach
- image support
- use a lot of comments in code to make contributing easier
- cli based scaffolding of standard websites
    - blogsite
    - theme
- dockerizing

## tags 
```
Element         Markdown Syntax
_________________________________
Heading         # H1
                ## H2
                ### H3

Bold            **bold text**

Italic          *italicized text*

Blockquote      > blockquote

Ordered List    1. First item
                2. Second item
                3. Third item

Unordered List 	- First item
                - Second item
                - Third item

Code 	        `code`

Horizontal Rule	---

Link 	        [title](https://www.example.com)

Image 	        ![alt text](image.jpg)

Escaping        \
```
