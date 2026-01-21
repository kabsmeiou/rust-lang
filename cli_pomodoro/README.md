# cli-pomodoro
cli-pomodoro is an executable command for the pomodoro technique with a slight twist: the break time that you get is randomized! This way, the amount of rest you can get depends on your atrocious luck. Have fun studying.

## Commands

[start arg: int?]       Start command for pomodoro timer. Defaults to 25 minutes. 
[-s | --seconds]        The number of seconds you wish to add to the timer.
[--sound arg: String]   The sound after the timer ends.
[--minbreak arg: int]   The minimum break time in minutes.
[--maxbreak arg: int]   The maximum break time in minutes.

```bash
# sample command
pomodoro start 24 -s 30 --sound Ping --minbreak 5 --maxbreak 10
```