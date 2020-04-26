## Push support

- add file telegram.yml to .github/workflows
- add secrets (telegram_to, telegram_token)
- enjoy

**.github/workflows/telegram.yml**
```
name: telegram

on:
  push:
    branches: [ master ]  
jobs:
  build:    
    runs-on: ubuntu-latest    
    steps:        
    - uses: avkviring/telegram-github-action@0.0.2
      env:
        telegram_to: ${{ secrets.telegram_to }}  
        telegram_token: ${{ secrets.telegram_token }}
        event: ${{ toJson(github.event) }}
```

**Result in telegram**
```
user push to repo
 ➞ Commit message 1
 ➞ Commit message 2
 ➞ Commit message 3
``` 
