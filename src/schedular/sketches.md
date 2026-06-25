**What to expect**
The scheduler should be called once, when the user start this program.
Reads from config file, setup the shceduler.
The main factor is the date, when this programm run itself.
We have support for Days and Weeks and Months, a count number could be given as well. if user write 3 Day, it means this program run it self each 3 day and save snapshot into local or remote repo. if he didnt specify the count, the program fallback to 1, meaning each DAY/WEEK/MONTH.
If user specify , this part is tottaly skipped (no scheduling work).
about what time to run, maybe try to run it at any given time in that day, once the pc is on.
##
**Early Support**
Initially we support Linux by default, windows will be supported in another time.
Found out systemd have a unit called ".path" , it watches for file or directory changes.we might turn this into an actual feature, when bookmark changes automatically snapshot it by running the program.
##
**early assumptions**
- what if the system isn't on when time to start isn't reached.
##
**Landed understandings**
we have decided to use what so called "systemd timers"
it works natively with any major distro (some minimal distros doesn't support it),helps you orginize and schedule jobs for OS to run.
there is another software called cron. and the early research we found out that cron doesn support "routine counts" and doesnt fallback when host is off.
picking systemd timers was the logical choice, here is what i understand about it:
- two parts needs to be configured. 
  - .service: it acts like the "what", defines the program, its arguments, paths....
  - .timer : the "when" part, define the date and time logic when this program will run.
why we need to parts?. 
  what to run ".service" can be reusable many times. same "program.service" can run a single timer systemd unit.
  
- there is two systemd instances
  - user-mode systemd, which are for a specific user, stop working if user logout 
  - root systemd, which for the whole system, no matter who is logged it, if system is up, it auto runs 
- the most important feature for this project is to run this program even its time is already passed. systemd have this feature.
  - its called persistent , when we define "Persistent=true" , the system will fallback to another time when system is up.
- **Unit File Syntax**
  - every uni have there sections
    - [Unit]:  for defining titles and requirements
    - [Service]/[Timer] (type specific for each unit) : the logic
    - [Install]: it solves the question:  "if someone enables this unit, what should that hook into?"
    - e.g., 
    ```
    [Unit]
    Description=Daily timer for Bookmark Tree
    
    [Timer]
    OnCalendar=*-*-* 02:00:00
    Persistent=true
    
    [Install]
    WantedBy=timers.target
  ```


**Systemd Unit Prototypes**
- **bookmark.service**
```
[Unit]
Desctiption=Bookmark Tree Snapshot
Wants= network-online.target
After= network-online.target
[Service]
Type=oneshot
ExecStart= "path to executable"
```
- **bookmark.timer**
```
[Unit]
Desctiption= Timer for bookmark tree
[Timer]
OnCalander="routine regular expressions"
Persistent=true
[Install]
WantedBy=timer.target
```