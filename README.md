# CL Calc

CL Calc is command line calculator that I made as a side project to learn to make a interpreter. It is messy and inefficient but is a fun proof of concept.

Running the program with no arguments allows you to enter commands line by line similar to how node works. Run with a path as the first argument to run the commands in that file. (I use ".calc" but is doesn't matter).

The valid options are:

1. "--help" which can also be accessed with "-?" and "-h"\
This will explain all the ways the program can be run. (equivalent to this list)
2. "--version" which can also be accessed with "-v"\
This will print the programs version.
3. "--about"\
This gives some basic information about this program.

There command that you can use while typing into the command line to they are prefixed with a '!' and are.

1. !exit\
This will exit the program.
2. !help\
This will give you additional information on how to use this program
3. !vars\
This will print out all of the names of the user defined functions and constants.
4. !file \<path>
This will execute all of the commands in a file
5. !out \<path>
This will output all successfully run commands to a file

The '!' commands do not work in a file.

When running a file you can put a '!' at the start of the line to have it output as well. Only the last line will be put into ans. This works when running a file as an argument and in the cli.
