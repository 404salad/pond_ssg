## what is this?
this is a blog and a static site generator  
simply converts a folder of markdown files to a blog webpage  

## why?  
I wanted to write blog in markdown and static deploy it without much hassle  
rust is fun  

## current status
- alpha version
- currently using a GitHub workflow to deploy on push  
- need to configure it such that other people can use it easier
- customization options without sacrificing simplicity  

## next steps
- fix: seo and accessibility? https://pagespeed.web.dev/analysis/https-404salad-github-io-blog
- refactor: cli so it's easier to set up and manage projects
- docs: a simple setup guide so that users can simply install binaries and potential instructions
- feat: add custom themes
- feat: how to add time to the blogs (maybe for now get file created time but that's hacky)
- feat: figure out a drafts system (when a filename starts with draft or a dot consider it to be a draft?)
- feat: maybe some backups thing
- test: write tests
- * feat: in a config file have flags for features like javascript (don't need to bundle if the user doesn't need it)
    ONGOING -> FIX CRATE PROJECT DIRS AND GENEERATE THE CSS FILE AND JS FILE IF FLAG IS THERE

- (done) refactor: use html templating (maud)
- (done) feat: maybe watcher for live reload 
