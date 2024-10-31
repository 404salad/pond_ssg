this is a blog and a static site generator  
simply converts a folder of markdown files to a blog webpage  

todos/bugs  
- no testing yet
- using templating for generating html intead of using raw strings for correctness  
    (maybe creating my own pythonic html templating :D)
- adding color themes or custom theming in the user config
- hacky solution to check if running for base directory, set a base directory on each run
- add overide for filename to not be the name of the article
- BIGGEST ISSUE IMO sorting dates -> sort it chronologically, use flags to add a new article eg- pond add "moon river" (maybe)
  can think of many solutions like adding timestamp to the filename but that will pollute the filename,  
  maybe can read metadata for when the file was created?(could run into crossplateform issues but is the simplest)
- non markdown files in the content directory need to be ignored
- refactoring utils dir into logical .rs files?

state?
- maintaing an internal state could be useful in some cases but using the content folder as the single source of truth is very powerful
- FEATURE IDEA -> a period '.' in front of the filename for drafts or hidden articles

deploying  
currently using a github workflow to deploy on push  
should write a script to set up a web server, install pond  
then you can ssh into your vpc and write  
or can i use ftp just to send the new blogs  

