#!/bin/bash

## CREDIT
# https://stackoverflow.com/questions/17601539/calculate-the-average-of-several-time-commands-in-linux
# bobmcn
#   ^ on Stack Overflow


rm -f /tmp/mtime.$$

for x in {1..10}
do
  /usr/bin/time -f "real %e user %U sys %S" -a -o /tmp/mtime.$$ $@
done

awk '{ et += $2; ut += $4; st += $6; count++ } END {  printf "Average:\nreal %.3f user %.3f sys %.3f\n", et/count, ut/count, st/count }' /tmp/mtime.$$