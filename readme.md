## what is this?
this is a blog and a static site generator  
simply converts a folder of markdown files to a blog webpage  

## why?  
i wanted to write blog in markdown and static deploy it without much hassle  
rust is fun  

## current status
- alpha version
- currently using a github workflow to deploy on push  
- need to configure it such that other people can use it easier
- customization options without sacrificing simplicity  

## next steps
- fix: seo and accessibility? https://pagespeed.web.dev/analysis/https-404salad-github-io-blog
- refactor: cli so its easier to setup and manage projects
- docs: a simple setup guide so that users can simply install binaries and potentiaial instructions
- feat: add custom themes
- refactor: use html templating (maud)
- feat: figure out a drafts system (when a filename starts with draft or a dot consider it to be a draft?)
- feat: maybe watcher for live reload 
- feat: maybe some backups thing
- test: write tests

