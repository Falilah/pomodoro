## Pomodoro techniques
A cli tool executable in the terminal, it will require two arguments to run, asking users for how many minutes of work and how many minutes of break, how many interval make a productive  lenghty break.

## build model

- User run with two commands, ✅
- It takes those command and set them up in a struct ✅
- assert worktime is atleat 3times greater than break time ✅
- start counting the first args based on minutes, ==> if conversion to seconds is needed do well to convert in the code
- After the first args trigger a break sound
- Allow users to acknowledge
- After aknowldging, start counting the break time
- trigger a continuation sound ==> allow users to acknowledge before continuation.

=====================> The break sound aknowledgment is a `yes` or `No` question
if yest ===> trigger another round of pomodoro, if no trigger a total stop to the execution.



### Bonus feature 
- ask for how many round of pomodoro to determine the long break 
- log the productivity of the day base on how may round was done

### imaginary features
 
Can it be link to a vscode window or any window running on the system to record time activities on it 