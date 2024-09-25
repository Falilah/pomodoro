
## Pomodoro Technique
A CLI tool that runs in the terminal, requiring 3 arguments: the duration of work in minutes, the duration of breaks in minutesand it also allows users to specify the number of intervals that constitute a productive long break.

## Build Model

- Users run the tool with three commands. ✅
- It takes these commands and sets them up in a struct. ✅
- Asserts that the work time is at least three times greater than the break time. ✅
- Starts counting the first argument based on minutes; if conversion to seconds is needed, it handles that in the code. ✅
- Triggers a break sound after the first argument's countdown. ✅
- Allows users to acknowledge the sound. ✅
- After acknowledgment, starts counting the break time. ✅
- Triggers a continuation sound and allows users to acknowledge before continuing. ✅
- Displays the time spent.

=====================> The break sound acknowledgment prompts a `yes` or `no` question.  
If the answer is **yes**, it triggers another round of Pomodoro. If **no**, it stops the execution completely. ✅

### Bonus Features
- Asks how many rounds of Pomodoro to determine the long break. ✅
- Logs the day's productivity based on the number of rounds completed.
- Displays a progress bar. ✅

### Imaginary Features
Can it be linked to a VSCode window or any running window on the system to record time spent on activities?

