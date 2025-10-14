```toml
title = "how to use cronjobs"
tags = ["tag1","tag2"]
date = 2025-04-20
```

> running stuff in the background at a specified time is awesome

I forget to backup my files and so i use cron to do it for me.  
Here is a basic rundown- 
- crontab is the tab of all cronjobs
- run `crontab -e` it will ask for setup, use your favorite editor

```bash
export EDITOR=nvim
```
now run `crontab -e` again and you should be able to access the file
```bash
# Add this line to your crontab file to run the backup every day at 2 AM
0 2 * * * rsync -ah --info=progress2 /path/to/source /path/to/destination
```
those asterics are wildcards! It means run it everytime  
use this command format examples for cronjobs

|m        | h        | dom      | mon      | dow      | command  |
|---------|----------|----------|----------|----------|----------|
| minutes | hours    | day      | month    | day of the week | command  |
|30 | 4  | *   | *   | 6   | 4:30 every Saturday               |
|15 | 14 | *   | *   | *   | 2:15 every day                     |
|0  | 0  | *   | *   | 0   | 12:00 AM every Sunday             |
|30 | 8  | *   | *   | 1-5 | 8:30 AM every weekday             |
|45 | 3  | 10  | *   | *   | 3:45 AM on the 10th                |
|59 | 23 | 31  | 12  | *   | 11:59 PM on December 31st         |

