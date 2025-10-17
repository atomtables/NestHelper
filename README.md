# NestHelper

the helper for your nest!!! The HackClub Nest servers are often slow and unresponsive,
or just plain annoying to interact with over SSH, which completely emulates a PTY when
unnecessary. Instead, NestHelper abstracts this using a GUI, separating input from display.

## features

- custom flows
- filesystem
- one-shot command processing
- dashboard

## how to work

1. install dependencies for tauri (pnpm install and cargo install)
2. run `pnpm run tauri dev` to run in dev mode
3. view the instructions on tauri for your platform to compile and sign

## why

it took me ages to get AtomChatDB set up, between the constant pushes and pulls, especially
with the insane latency. abstraction allowed me to get my new project, Ondru, up and
running really really quickly.
