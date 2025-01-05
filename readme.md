this is a blog and a static site generator  
simply converts a folder of markdown files to a blog webpage  

todos/bugs  
- https://pagespeed.web.dev/analysis/https-404salad-github-io-blog/hydcc5hula?form_factor=mobile
- no testing yet
- using templating for generating html intead of using raw strings for correctness  
    (maybe creating my own pythonic html templating )
    probably maud
- adding color themes or custom theming in the user config
- add overide for filename to not be the name of the article
  maybe can read metadata for when the file was created?(could run into crossplateform issues but is the simplest)
- refactoring utils dir into logical .rs files?

state?
- maintaing an internal state could be useful in some cases but using the content folder as the single source of truth is very powerful
-  a period '.' in front of the filename for drafts or hidden articles, just dont have .md externsion and it works

deploying  
currently using a github workflow to deploy on push  
should write a script to set up a web server, install pond  
then you can ssh into your vpc and write  
or can i use ftp just to send the new blogs  


