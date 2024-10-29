this is a blog and a static site generator  
simply converts a folder of markdown files to a blog webpage  

todos/bugs  
- no testing yet
- Does not check for directory from which the program is run, leading to bugs (to fix this dont hardcode directory names)
- using templating for generating html intead of using raw strings for correctness
- adding color themes or custom theming in the user config
- too many unwraps
- hacky solution to check if running for base directory, set a base directory on each run
- add overide for filename to not be the name of the article
- sort it chronologically, use flags to add a new article eg- pond add "moon river" (maybe)
- non markdown files in the content directory need to be ignored
- refactoring utils dir into logical .rs files?

some ideas for usabilty and performance  
- should probably add a file mode to watch a specific file for changes and then update it
- maybe i should watch all the files for changes and then update specifi file but then again idk how it iwll handle file name changes 
- if the folder dretects change then we can rerender the entire thing, otherwise we can partially update the articles that are being watched specifically??
- former overrides the latter

state
- maintaing an internal state could be useful in some cases but using the content folder as the single source of truth is very powerful
- FEATURE IDEA -> a period '.' in front of the filename for drafts or hidden articles
