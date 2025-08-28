# NestHelper Control Flow

While I am NOT going near implementing a Control Flow right now, here are the
syntaxes that would be required for a proper implementation of Control Flow using
a single-line bash script.

## Modifications to Commands
to ensure that Flows can be continued even if a command fails, the following
modifications to commands are required:
```shell
if ${command}; then echo "---STAGE-2-atomtables--"; else read; fi
```

## Jump on the frontend/Dynamic Jumps
Jumps are basically impossible in bash, especially since jumps only occur
during frontend tasks. Realistically if we really wanted to implement jumping, we
could just kill the current SSH process and try to initialise a new one at the
point we want to jump at.

## Known conditional jump
Since we would know where we are going to start branching, we could form
sequential commands with if statements that determine where to go. If we are at
stage 4 and want to move to either stage5a, stage5b, or stage 7, we could use
a combination of shell functions and if statements to determine where to go based
on shell variables.

## Known unconditional jump
Realistically this doesn't involve anything until we add known conditional jumping.