# XRPLMerge

An in-progress command line utility that implements rudimentary dependencies for the 4RPL scripting language used in Creeper World 4.
It has a separate folder structure from the game, and puts the final merged scripts into a CPACK's scripts folder.

Scripts in the src folder are the main scripts that will all be directly copied to the game's scripts folder. They can include any scripts from the lib folder with the syntax `#include library.4rpl`.

Library scripts in lib can also include other library scripts.

There are currently three commands:
- new - Creates the required folders when run in a CPACK directory, and copies any existing scripts to the src folder.
- merge - Default if no command specified. Merges any library files (from the lib folder) into the src files and places the result in the scripts folder.
- update - Detects any new scripts created in the scripts folder and copies them over to src.

## Example
```
# test.4rpl
#include lib.4rpl

:Once
    @Function
```
```
# lib.4rpl
#include lib2.4rpl

:Function
    print("Function")
    @AnotherFunction
```
```
# lib2.4rpl
:AnotherFunction
    ->b ->a

    <-a <-b +
```

Merges to:
```
# test.4rpl
#include lib.4rpl

:Once
    @Function

# Source: lib.4rpl
# lib.4rpl

:Function
    print("Function")
    @AnotherFunction
# Source: lib2.4rpl
# lib2.4rpl

:AnotherFunction
    ->b ->a

    <-a <-b +
```