CASE 1
------

Config
------
[Journal]
RateLimitInterval=5
RateLimitBurst=5000000
#Storage=auto (/var/log/journal exists. So persistant)

* Remove all the files from /var/log/journal/xxxxxxx and restart systemd-journald (creates system.journal)
* start the utility and read (read some journals and `None` after sometime)
* start journal frame generator (user-1000.journal gets created)
* (reads still returning `None`)


CASE 2
------
* Stop and start the utility again (`Journal::open` now considers `user-1000.journal` too. Reads new journals)
* Run until log rotation happens (New `user-1000.journal` gets created) (`None`s after logrotation)