# Soothsayer

This is a cli tool that helps automate some of the work flow and project setup for SEER.
It will make running projects locally a lot easier as well as setting up new projects.


### WIP
Soothsayer is currently a work in progress.


### Install

If you have Rust installed on your computer you have `cargo` installed on your computer.
You can use this to install Soothsayer, simply:

```bash
cargo install soothsayer
```


### Init (optional)

There is an init command that allows you to walk through a given set of options that will be used within the commands.
These will only be used later when running other commands so don't expect a lot from the output.


### Usage

Well first off you can run `ss --help` and that will show you all the current commands for soothsayer.

An example command you could run in a seer local project:

```bash
ss run config:dev
```

This would run `npm run config:dev` under the hood. I bet you are thinking to yourself "Why would I add another step
to a nearly identical command? Soothsayer will run the command and then wait for your username and password prompts 
to pop. On first run it will save your provided login and then auto populate those fields in the prompts.

Essentially making your life a breeze because it has the foresight to tell your programs futures!

